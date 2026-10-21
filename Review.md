# RustyRook — Code Review, Strength Assessment & Improvement Plan

_Review date: 2026-10-21 · Reviewed commit: `b9033bd` · Reviewer: code audit of all 5,303 lines of `src/`_

> **Method / caveat:** This is a static review. No Rust toolchain was available on the review
> machine, so the perft suite, the unit tests and NPS benchmarks were **not** executed. Every
> finding below is derived from reading the source; items marked **(unverified)** should be
> confirmed with a perft run before acting on them. Elo figures are rough engineering estimates,
> not measured results — see [§9 How to measure](#9-how-to-measure-this-properly).

---

## 1. Verdict

| Dimension | Rating | Comment |
|---|---|---|
| **Code quality** | **6.5 / 10** | Clean module layout, readable, genuinely tested (incl. perft). Held back by non-idiomatic Rust, duplication, `panic!` as control flow, and a global `DashMap` leaking into move generation. |
| **Chess correctness** | **5 / 10** | Move generation looks sound in design (magic bitboards, pin rays, double-check handling), but the UCI layer has bugs that produce *illegal output* and *mis-parsed input*. |
| **Search quality** | **4 / 10** | Alpha-beta + ID + TT are present, but the TT flag logic is wrong, the root doesn't actually re-order on the previous best move, and quiescence is a fixed-depth capture search. |
| **Evaluation** | **5.5 / 10** | Material + PSQT + mobility + king safety + passed pawns + doubled pawns is a respectable set; no tapering, no bishop pair, and one color-confusion bug. |
| **Playing strength** | **≈ 1800 Lichess blitz** | See below. |

### Playing strength

The linked bot account [`RustyRookChessBot`](https://lichess.org/@/RustyRookChessBot) currently shows:

| Time control | Rating | Games |
|---|---|---|
| Bullet | 1824 (provisional) | 43 |
| Blitz | **1806** (provisional) | 131 |
| Rapid | 2003 (provisional) | 1 |

Blitz over 131 games is the only sample with any weight. ~1800 on Lichess is a solid club
player — the engine is clearly past the "hangs pieces" stage and is doing real tactical work.

Translated to engine rating lists (which are far more deflated than the Lichess bot pool), this
behaviour is consistent with roughly **1300–1600 CCRL blitz**. The profile matches the code:
nominal depth 4–8 plus a 3–4 ply capture search, no null-move/LMR/killers, hand-tuned eval,
opening book. For reference, a clean 1-thread engine with the same feature set *plus* correct
TT flags, PVS, killers, MVV-LVA and a true quiescence typically lands 2200–2400 CCRL. **Most of
that gap is recoverable without rewriting anything** — it's in Tiers 0–2 below.

The single most alarming finding is **§2.1**: the engine emits promotion moves in a format that
strict UCI clients (including python-chess, which `lichess-bot` uses) reject outright. Any game
where RustyRook promotes is at risk of being lost on an illegal-move/parse error rather than on
the board. That alone could be suppressing the rating noticeably.

---

## 2. Tier 0 — Bugs that lose games outright

These are not "strength" issues; they are protocol/state-corruption issues. Fix these first.

### 2.1 Promotion moves are sent in invalid UCI notation ⚠️ **critical**

`src/board/promotion.rs` → `convert_promotion_to_output_string`:

```rust
Promotion::Queen => String::from("Q"),
Promotion::Knight => String::from("N"),
```

The UCI long-algebraic format requires a **lowercase** promotion suffix: `e7e8q`. The engine emits
`e7e8Q`. python-chess parses the 5th character against `PIECE_SYMBOLS = [None,"p","n","b","r","q","k"]`
— uppercase raises `InvalidMoveError`, which in `lichess-bot` means the move is never played.
Tolerant GUIs (Arena, some builds of Cute Chess) accept it, which is why this can hide for a long time.

**Fix:**

```rust
Promotion::Queen => String::from("q"),
Promotion::Rook => String::from("r"),
Promotion::Bishop => String::from("b"),
Promotion::Knight => String::from("n"),
```

Add a regression test asserting `send_move` output is exactly `e7e8q`.

### 2.2 Knight promotions from the GUI are silently mis-parsed ⚠️ **critical**

`src/helper/position_to_usize.rs:43`:

```rust
let valid_move_regex = Regex::new(r"\A[abcdefgh][1-8][abcdefgh][1-8]([qrbkQrbK]?)").unwrap();
```

The character class accepts `k`/`K` for knight but **not `n`** — which is what every GUI actually
sends. So `position ... moves e7e8n` matches the first four characters, capture group 1 is empty,
`promoted_to` becomes `None`, and `move_figure` moves a **pawn onto the 8th rank**. The internal
board is now permanently desynced from the real game: wrong material, wrong hash, wrong (and
eventually illegal) moves for the rest of the game. Uppercase `R`/`B` are rejected too.

**Fix:** use `([qrbnQRBN]?)` and map `n|N → Knight` in `convert_input_string_to_promotion`
(keep `k|K` as a tolerated alias if you want backwards-compat with your own output).
Guard it in `update_position_from_uci_input`: if the regex fails to match a token, **log loudly**
instead of silently ignoring it.

### 2.3 `position fen ...` is not supported at all

`src/main.rs:149-171` — `update_board` unconditionally calls `board.set_to_default()` and then
feeds every remaining token to `update_position_from_uci_input`. The tokens `fen`, `rnbq.../8/...`,
`w`, `KQkq`, `-`, `0`, `1` all fail the move regex and are discarded. Result: the engine plays
**from the standard starting position** while the GUI believes it is in the given FEN.

This affects every tournament setup that uses opening books/EPD start positions (cutechess-cli,
fastchess, Lichess "from position" games) — i.e. exactly the tooling you need for measuring Elo.

**Fix:**

```rust
match move_vec.get(1) {
    Some(&"fen") => {
        let fen = move_vec[2..].iter().take_while(|t| **t != "moves")
                               .cloned().collect::<Vec<_>>().join(" ");
        board.create_position_from_input_string(fen);
    }
    _ => board.set_to_default(),
}
// then replay everything after the "moves" token
```

### 2.4 Unknown / routine UCI commands kill the process

`src/main.rs:124-134`:

```rust
let differentiation: &str = commands.first().unwrap_or(&"stop");
match differentiation {
    ...
    _ => quit(String::from("Unknown Command!")),   // quit() == panic!()
}
```

Every one of these terminates the engine with a panic:

* an **empty line** (`first()` → `None` → `"stop"` → `_` branch),
* `stop` — the standard way a GUI ends an infinite search,
* `setoption name ... value ...` — sent by Arena/cutechess right after `uci`,
* `ponderhit`, `register`, `go infinite`+`stop`, `ucinewgame` variants, stray whitespace.

**Fix:** ignore unknown commands (log them), handle `stop` by setting the search's abort flag,
accept and ignore `setoption` until options exist, and make `quit` a clean
`std::process::exit(0)` rather than `panic!`.

### 2.5 `ucinewgame` replies `isready`

`src/main.rs:266` prints `isready` — that is a *GUI→engine* command. `ucinewgame` must produce no
output; only `isready` may answer, and only with `readyok`. Some GUIs will choke on this.

### 2.6 Castling rights survive a rook captured by promotion

`src/board/board.rs:209-222` — `update_figure_to_promoted_one` removes the captured piece but,
unlike `move_white_figure`/`move_black_figure` (which handle `to == 0/7/56/63`), it never clears
the opponent's castling rights or the corresponding Zobrist bits.

After e.g. `b2xa1=Q` capturing the h1/a1 rook, `white_castle_long` stays `true`. The engine can
then generate `e1c1` — `is_possible_castle` only checks the rights flag and the empty squares,
never that a rook is actually on a1 (`src/figures/king.rs:94-108`). `white_castle(2)` then calls
`move_white_figure(0, 3)` on an empty square, which is a no-op, so the king teleports to c1 with
**no rook move at all** — an illegal move sent to the GUI, plus a corrupted Zobrist key.

**Fix:** factor the rook-capture rights update into one helper and call it from the promotion path
too. Additionally make `is_possible_castle` verify the rook bitboard as a belt-and-braces check.

### 2.7 The engine can send `bestmove a1a1`

`src/engine/engine.rs:103-107` initialises `best_move` to `MoveWithRating::default()`
(`from: 0, to: 0`). If the timer fires before the first depth-2 iteration completes — low time,
`movetime` small, a heavy position — nothing is ever received on the channel and
`send_move(0, 0, None)` prints `bestmove a1a1`. Illegal → forfeit.

**Fix:** after generating root moves, immediately set `best_move` to the first legal move, before
any search starts. Never send a move that didn't come from the move generator.

### 2.8 Root search panics when there are no legal moves

`src/engine/engine.rs:156`: `let first_move = valid_moves.remove(0);` panics on an empty list.
If the GUI asks the engine to move in a checkmated/stalemated position (happens with analysis
GUIs and some arbiters), the process dies.

**Fix:** early-return with `bestmove (none)` / `0000` and a `info string mated` line.

### 2.9 `movetime` under 100 ms underflows

`src/main.rs:211`: `return exact_movetime - 100;` — with `go movetime 50`, `u64` underflow in
release mode wraps to ~1.8×10¹⁹ ms. The engine never moves and loses on time.

**Fix:** `exact_movetime.saturating_sub(100).max(10)`.

---

## 3. Tier 1 — Search bugs costing real Elo

### 3.1 Transposition-table flags are always `Upperbound` ⚠️ **high impact**

`src/engine/engine.rs:339-350`:

```rust
alpha = alpha.max(adjusted_evaluation);   // alpha raised inside the move loop
...
if best_move_rating <= alpha {            // <-- compared against the RAISED alpha
    transposition_flag = Flag::Upperbound;
} else if best_move_rating >= beta {
    transposition_flag = Flag::Lowerbound;
}
```

After the loop, `alpha >= best_move_rating` is a tautology (alpha was raised by every candidate).
So the first branch *always* wins: **`Flag::Exact` and `Flag::Lowerbound` are never stored.**

Two consequences:

1. `get_entry` (`table.rs:26-32`) only ever cuts on the `Upperbound && eval <= alpha` path, so the
   TT delivers a small fraction of the cutoffs it should.
2. Far worse — a node that **failed high** (`best >= beta`, a true *lower* bound) is stored as an
   *upper* bound. Later probes treat "this is at least X" as "this is at most X" and cut on it.
   That returns **wrong scores**, which is exactly how an engine blunders a piece at depth 6 in a
   position it "solved" at depth 4.

**Fix:**

```rust
let alpha_orig = alpha;              // capture BEFORE the loop
...
let flag = if best_move_rating <= alpha_orig { Flag::Upperbound }
           else if best_move_rating >= beta  { Flag::Lowerbound }
           else                              { Flag::Exact };
```

Estimated gain: **+100 to +250 Elo.** This is the single highest-value fix in the file.

### 3.2 Aborted searches poison the transposition table ⚠️ **high impact**

`src/engine/engine.rs:240-247`: when the timer fires, `calculate` returns a static eval. But the
**parent** frame keeps iterating — each remaining child returns instantly with a static eval — and
then reaches line 352 and **inserts that garbage into the TT, stamped with the full `depth_to_end`**.

The table is only cleared on `ucinewgame` (`main.rs:264`), so every subsequent move in the game
probes these fake "deep" entries and trusts them.

**Fix:** propagate abortion explicitly and discard the node:

```rust
if timer.load(Ordering::Relaxed) { return ABORTED; }
...
if !aborted && calculate_all_moves { TRANSPOSITION_TABLE.insert(...) }
```

Also check the timer every ~2048 nodes rather than at every node entry (an atomic load per node is
itself a measurable cost).

Estimated gain: **+50 to +150 Elo.**

### 3.3 Iterative deepening never actually re-orders the root

The comment at `engine.rs:155` says *"calculate prev. best move sequential to get baseline alpha"*,
and move ordering relies on `add_prev_best_move_as_first_move` reading the TT for
`board.zobrist_key`. But **the root position is never written to the TT** — `calculate` is only
called on *children* of the root, and only children insert entries.

So on every ID iteration the root move list comes back in raw generation order (captures first,
then quiets). The whole point of iterative deepening — that iteration *n−1* tells iteration *n*
which move to try first — is lost at the node where it matters most.

**Fix:** have `iterative_deepening` keep the previous iteration's best move in a local variable and
hoist it to index 0 itself (and/or insert the root into the TT after each iteration).

Estimated gain: **+50 to +120 Elo** (better root ordering ⇒ far better alpha ⇒ much smaller tree).

### 3.4 Half of all search effort is thrown away

`engine.rs:146` loops `for max_depth in 2..=100` (step 1) but `engine.rs:212-215` discards every
odd iteration:

```rust
if max_depth % 2 != 0 { continue; }   // "we only want calculations ending on opponent moves"
```

The odd iteration is fully searched and only its TT side-effects are kept. That's close to a
**2× waste** of wall-clock (an odd-depth search is cheaper than the even one above it, but far
from free).

The underlying premise — that a search must end on an opponent move to avoid odd/even effects — is
what **quiescence search** exists to solve. With a proper quiescence (§4.1) the parity problem
disappears.

**Fix:** step by 1 and report/use every iteration; or, if you want to keep the parity rule as a
stopgap, iterate `for max_depth in (2..=100).step_by(2)` so you stop paying for the discarded ones.

Estimated gain: **+40 to +80 Elo** (roughly one extra ply of effective depth).

### 3.5 Root parallelism searches every sibling with a wide-open window

`engine.rs:173-198`: all root siblings are searched in parallel with `(-beta, -alpha)` where
`beta == PLACEHOLDER_RATING == 5000`. From the child's perspective the window is
`(-5000, -alpha)` — effectively unbounded on one side — and **`alpha` is never updated as siblings
finish**. Move 2 cannot benefit from move 7's refutation.

This is a textbook case where naive root splitting is *slower than sequential search* at equal node
counts. A sequential PVS over the same moves would search a far smaller tree.

**Fix (in order of effort):**

1. **PVS**: search move 1 with the full window, every other move with a null window
   `(-alpha-1, -alpha)`, and re-search only on a fail-high.
2. Share alpha between threads via `AtomicU32` (bit-cast the score) so siblings prune against the
   current best.
3. Longer term, replace root-splitting with **Lazy SMP**: N threads search the *same* root with
   slightly different depths/orderings, communicating only through the TT. It is far simpler than
   YBWC and scales better.

Estimated gain: **+80 to +150 Elo** on a multicore machine.

### 3.6 The TT move is injected without verifying it is legal

`src/engine/moves.rs:271-289`:

```rust
if let Some(pos) = moves.iter().position(|single| ...) {
    moves.remove(pos);
}
moves.insert(0, prev_best_move);   // inserted even when NOT found in the legal list
```

If the stored move isn't in the generated list, it is pushed in anyway and will be played.
Normally the Zobrist key guarantees the same position, but it does **not** here, because:

* 64-bit collisions exist (3.2M entries in flight), and
* the "verification" in `table.rs:8-14` (`transposition.hash == board_hash`) is a tautology — the
  hash *is* the `DashMap` key.

You already have empirical evidence of this path misbehaving: three perft tests are `#[ignore]`d
with the comment *"this test fails if it is run after the other tests for some reason I don't want
to debug"* (`board.rs:872`, `886`). That is exactly this bug — `count_moves` calls
`get_valid_moves_in_position`, which reads the **global** TT populated by the engine tests running
in the same process, and the injected move inflates the perft count.

**Fix:** only hoist the TT move if `position()` found it; never `insert` an unverified move.
That makes perft deterministic again and lets you un-ignore those tests.

### 3.7 Mate scores are stored in the TT un-normalised

`lost_game(depth_to_end)` returns `-3000.0 - depth_to_end`. Those ply-relative scores are written
into the TT verbatim and read back at a different distance from the root, so "mate in 3" can
re-emerge as "mate in 7" or vice versa, producing mate-score drift and occasional failure to
convert a forced mate.

**Fix:** the standard normalisation — on *store*, convert to "mate distance from this node"
(`score + ply`); on *probe*, convert back (`score - ply`).

### 3.8 Draw detection is incomplete

* **No 50-move rule.** The halfmove clock is never parsed from FEN
  (`create_position_from_input_string` ignores it) and never tracked. The engine cannot avoid or
  claim a 50-move draw, and will happily shuffle away a winning position.
* **Threefold is approximated at the root only.** `twice_played_positions` is built in
  `update_board` from the actual game, which is correct, but repetitions occurring *inside* the
  search tree (position repeated twice within the PV) are not detected — only positions already
  played twice in the real game.

**Fix:** track `halfmove_clock` in `Chessboard` (reset on captures/pawn moves), return a draw score
at 100, and keep a per-search path stack so a repetition inside the tree returns 0.

---

## 4. Tier 2 — Missing search features (ordered by Elo-per-hour-of-work)

### 4.1 Replace the fixed-depth capture search with a real quiescence

Today (`engine.rs:249-268`): at `depth == max_depth`, the search restarts with `depth = 0` and runs
captures only until `depth == max_depth_takes` (3 or 4), then returns a static eval — **possibly in
the middle of an unresolved capture sequence.** Every horizon effect quiescence is supposed to kill
still exists, just four plies further out, and the fixed depth makes it expensive in quiet
positions and too shallow in sharp ones.

A real quiescence recurses until no captures remain:

```rust
fn quiescence(board, mut alpha, beta) -> f32 {
    let stand_pat = evaluate_for_own_color(board);
    if stand_pat >= beta { return stand_pat; }          // stand-pat cutoff — MISSING today
    if stand_pat > alpha { alpha = stand_pat; }
    for mv in captures_ordered_by_mvv_lva(board) {
        if stand_pat + see_gain(mv) + 2.0 < alpha { continue; }   // delta pruning
        if see(board, mv) < 0.0 { continue; }                     // skip losing captures
        let score = -quiescence(&make(board, mv), -beta, -alpha);
        if score >= beta { return score; }
        alpha = alpha.max(score);
    }
    alpha
}
```

Note the **stand-pat cutoff is absent today** — `init_best_move` computes the stand-pat score
(`engine.rs:86-91`) but never uses it to raise alpha or to return early, so the capture search
explores far more than it needs to.

Estimated gain: **+80 to +150 Elo.**

### 4.2 MVV-LVA capture ordering + SEE

Captures are currently ordered by *which generator ran first* (pawn, rook, bishop, queen, knight,
king — `moves.rs:217-236`), i.e. essentially arbitrary. Sorting captures by
**Most Valuable Victim − Least Valuable Attacker** is ~30 lines and is the cheapest large win in
move ordering. Add Static Exchange Evaluation to skip losing captures in quiescence.

Estimated gain: **+60 to +120 Elo.**

### 4.3 Killer moves and history heuristic

Both are listed in the README as future work. Two killers per ply plus a
`[color][from][to] -> i32` history table typically give **+70 to +130 Elo** combined, and they are
the main thing that makes quiet-move ordering work once the TT move and captures are exhausted.

### 4.4 Null-move pruning

With a verification search and the usual zugzwang guard (skip when the side to move has only king
and pawns), R=2–3 null move is worth **+60 to +100 Elo** and is maybe 20 lines inside `calculate`.

### 4.5 Late Move Reductions

Once ordering is good (4.2 + 4.3), reducing late quiet moves by 1–2 plies is the single biggest
depth multiplier available: **+80 to +150 Elo**.

### 4.6 Check extensions

Extend by one ply when the side to move is in check. `get_valid_moves_in_position` already returns
`is_in_check` for free, so this is a two-line change. **+20 to +40 Elo.**

### 4.7 Aspiration windows

Start iteration *n* with `(prev_score - 0.25, prev_score + 0.25)` and widen on fail. Pairs naturally
with the PVS change in §3.5. **+20 to +40 Elo.**

### 4.8 En passant is missing from the capture generator

`get_possible_pawn_takes_and_promotion` (`pawn.rs:239-252`) handles captures and push-promotions but
**not en passant** — ep is only generated in `get_possible_pawn_moves`, the *quiet* generator, which
is skipped when `calculate_all_moves == false`. So the quiescence search can never see an ep
capture. Small, but it's a genuine blind spot in tactical positions. **(unverified by perft)**

---

## 5. Tier 3 — Evaluation

### 5.1 King-safety bonus is computed from the wrong side's pieces 🐛

`src/evaluation/mod.rs:147`:

```rust
let fields = KING_SAFETY_FIELDS[color as usize][king_position as usize].board
             & own_positions.board;
```

`own_positions` is passed in at line 208 as `board.get_positions_by_current_player()` — the
**side to move**, not `color`. `get_position_weight` is called once for White and once for Black
with the *same* bitboard, so one of the two colors always scores its pawn shield against the
opponent's occupancy. Half of every evaluation's king-safety term is noise.

**Fix:** pass `&board.used_positions[color as usize]`.

### 5.2 No tapered evaluation — the eval jumps discontinuously

`get_king_weight` switches from `EARLY_GAME_KING_RATE_*` to `LATE_GAME_KING_RATE` the moment the
opponent's material drops to 15.0 (`mod.rs:142-144`), and `get_opponent_king_bonus` flips at the
same threshold. A single capture can swing the evaluation by a full pawn for reasons that have
nothing to do with the position. The search will then happily steer toward/away from that cliff.

**Fix:** the standard phase interpolation:

```rust
let phase = ((total_npm - ENDGAME_LIMIT) / (MIDGAME_LIMIT - ENDGAME_LIMIT)).clamp(0.0, 1.0);
let king_score = phase * mg_table[sq] + (1.0 - phase) * eg_table[sq];
```

Apply it to all PSQTs, not just the king. **+30 to +60 Elo.**

### 5.3 `score cp` is reported at roughly double scale

`engine.rs:132` prints `(received.rating * 100.0).round()` as centipawns. But the eval counts each
piece twice — once in `get_pieces_value` (pawn = 1.0) and once in the PSQT
(`PAWN_RATE_*` ≈ 1.0, `KNIGHT_RATE` ≈ 3.0, `ROOK_RATE` ≈ 5.0 …). A pawn is therefore worth ≈ 2.0
internal units, so "+1.00 pawn" is printed as `score cp 200`.

The *ratios* are consistent, so the search is unaffected — but every GUI, log and analysis you look
at is showing double. Also, forced mates should be reported as `score mate N`, not as `cp -3000`.

**Fix:** either halve at output, or (better) move to integer centipawns internally (§6.1) with the
PSQT containing the *full* piece value and `get_pieces_value` dropped entirely.

### 5.4 Missing evaluation terms

In rough order of value:

| Term | Note | Est. |
|---|---|---|
| **Bishop pair** | ~0.3–0.5 pawn; trivially cheap (`count_ones() == 2`) | +20–40 |
| **Passed-pawn scaling** | Currently linear `0.3 × rank` (`mod.rs:319`). Should be quadratic, and should scale with blockers, king distance, and rook-behind-passer | +20–40 |
| **Isolated / backward pawns** | Only doubled pawns are penalised today (`get_douplicate_pawn_tariff`) | +20–30 |
| **Rook on open / semi-open file** | Partially proxied by the mobility term, but a direct bonus is better | +15–30 |
| **King danger zone** | Count attackers/attack weight on the squares around the enemy king, rather than only counting your own pawns in front of your king | +30–60 |
| **Pawn hash table** | Pawn structure recomputed from scratch at every node; caching it is pure speed | — |

### 5.5 `get_douplicate_pawn_tariff` iterates 8 bitboards per color per node

`mod.rs:233-243` loops over `DOUPLICATE_PAWN_TARIFF` with a branch per file. Given this runs at
every leaf, a precomputed file-count via `popcount` per file mask, cached in a pawn hash, removes it
from the hot path entirely.

---

## 6. Tier 4 — Performance / NPS

NPS is not reported by the engine at all (see §8.3), so these are reasoned, not measured.

### 6.1 Floating-point evaluation

`f32` scores throughout. This costs you:

* FP compare/branch in the hottest loop instead of integer compare;
* `partial_cmp(...).unwrap()` at `engine.rs:218` — a **panic** if any score is ever `NaN`;
* non-deterministic tie-breaking between parallel threads, which makes reproducing a bug hard;
* no exact equality, so `>= beta` tests are subtly fuzzy.

**Fix:** switch to `i32` centipawns. Mechanical change, meaningful speedup, removes a panic path.

### 6.2 `DashMap` as the transposition table ⚠️ **large**

`main.rs:41`: `DashMap<u64, Transposition>` with 3.2M capacity.

Per node this does a hash, a shard lock acquisition, and a pointer-chase into a heap bucket — and
the search performs **at least two** of these per node (`get_entry_without_check` inside move
generation at `moves.rs:238`, plus `get_entry`/`insert` in `calculate`). Under `rayon` with 8+
threads the shard locks also contend.

Every serious engine uses a **fixed-size, power-of-two, lock-free array** indexed by
`hash & (size-1)`, with the full hash stored in the entry for verification and a depth-preferred
replacement policy. No allocation, no locking, one cache line per probe.

```rust
#[repr(align(64))]
struct TtEntry { key: AtomicU64, data: AtomicU64 }   // Hyatt lockless XOR trick
static TT: [TtEntry; 1 << 22] = ...;                  // 4M entries ≈ 64 MB
```

This also fixes **unbounded memory growth** (the map only grows; it is cleared solely on
`ucinewgame` — the cause of the leak addressed in commit `918ed5c`) and lets you expose a standard
`setoption name Hash value N`.

Estimated: **2–5× NPS**, which is worth **+100 to +200 Elo** on its own.

### 6.3 `MOVES_BY_FIELD: FxHashMap<usize, MoveInEveryDirection>` in the hottest path

Used by `ray.rs` (pin detection), `checked.rs` (check resolution) and `pawn.rs` (ep legality).
Each lookup is a hash + pointer-chase to a struct of **eight `Vec<usize>`s** — eight separate heap
allocations, iterated element-by-element with `board.positions.field_is_used(x)` per square.

This is the classic pre-bitboard ray scan running *alongside* your magic bitboards, which already
compute the same information branchlessly.

**Fix:** replace with `[MoveInEveryDirection; 64]` at minimum (removes the hash and one
indirection), and ideally with precomputed `RAY[64][8]` / `BETWEEN[64][64]` **bitboards** so pin
and check detection become a handful of `&`/`popcount` operations. Pin detection in particular
becomes the standard "xray the king square, intersect with enemy sliders" trick.

### 6.4 `pinned_pieces.insert(pinned_piece, moves.clone())` allocates per node

`ray.rs:81` clones an entire direction `Vec<usize>` for **every pinned piece, at every node**, into
a freshly allocated `FxHashMap` (`ray.rs:15`). Then `moves.rs:302-309` does a hash lookup plus a
**linear `Vec::contains`** for every generated move.

**Fix:** with the `BETWEEN`/`RAY` bitboards from §6.3 this becomes a single
`pinned: Bitboard` plus `allowed: [u64; 64]`, zero allocations, O(1) filtering:

```rust
if pinned.field_is_used(mv.from) && !allowed_ray[mv.from].field_is_used(mv.to) { continue; }
```

### 6.5 Full opponent attack map recomputed at every node

`get_all_threatened_fields` (`moves.rs:65-126`) iterates all six piece types of the opponent at
every single node to answer "am I in check?". That is the price of the current legal-move
generation design.

A cheaper design: generate *pseudo-legal* moves, and detect check with a single
`square_attacked_by(king_sq, opponent)` query that walks only the attack sets *from the king
square outward* (knight-attacks ∩ enemy knights, rook-attacks-from-king ∩ enemy rooks+queens, …).
That's ~6 bitboard operations instead of a loop over every enemy piece.

### 6.6 Board copy instead of make/unmake

`let mut new_board = *board;` at every node copies ~136 bytes. This is a legitimate design choice
(copy-make is simpler and often competitive), but combined with §6.5 it means each node does both a
full board copy *and* a full attack-map rebuild. Worth benchmarking against an unmake-based
alternative once the above are done.

### 6.7 `Chessboard::empty()` builds the start position first

`board.rs:71-78` constructs `Default` (which parses the full start FEN, char by char, with
`to_uppercase().to_string()` **allocating a String per piece**) and then immediately wipes it.
Only used in tests, so harmless for strength — but `get_figure_from_char` allocating a `String`
per character is also on the FEN path used by the opening book loader (1.7 MB file).

---

## 7. Tier 5 — Code quality

**Strengths worth keeping:** clear module boundaries (`board` / `figures` / `engine` / `evaluation`
/ `helper`), descriptive names, comments that explain *why* (e.g. the "remove own king so the king
can't step backwards along the attacking ray" trick at `sliding_moves.rs:36`), real perft tests
against the Chess Programming Wiki, and CI on push.

### 7.1 Non-idiomatic Rust

| Pattern | Where | Better |
|---|---|---|
| `&Vec<usize>` parameters | `ray.rs:71`, `checked.rs:149`, `pawn.rs:108` | `&[usize]` |
| `x.is_none()` then `x.unwrap()` | `moves.rs:275-280`, `promotion.rs:22-24`, `main.rs:218-219` | `if let` / `let ... else` |
| `panic!` as control flow | `main.rs:257` (`quit`), `opening.rs:41` | `std::process::exit` / `Result` |
| `String` return from conversions | `sender.rs:5-17` | `&'static str` or `char` |
| `.unwrap()` on user input | `position_to_usize.rs:58-62`, `board.rs:517` | propagate `Option`/`Result` |
| `Regex::new` inside the call | `position_to_usize.rs:43` | `lazy_static!`/`OnceLock` |
| Magic numbers (`15.0`, `3000.0`, `5000.0`, `0.02`, `0.3`) | `evaluation/mod.rs`, `engine.rs` | named `const`s — these are your **tuning parameters**, they should be visible and ideally runtime-settable |

### 7.2 Duplication

`get_passed_pawn_bonus_white` / `_black` (`mod.rs:305-345`), `white_castle`/`black_castle`
(`king.rs:88-132`), `move_white_figure`/`move_black_figure` (`board.rs:392-452`),
`possible_white_en_passant`/`possible_black_en_passant`, and the eight near-identical direction
blocks in `checked.rs:49-144` and `ray.rs:16-57`.

All of these are the same code with a `Color`-dependent sign/offset. Generic over `Color` (or over a
direction constant) they'd be a third the size — and bugs like §5.1 would be structurally
impossible. Note that `move_white_figure` handles `from == 4` (king moved ⇒ lose both rights) but
`move_black_figure`'s equivalent is `from == 60`: correct, but only verifiable by reading both.

### 7.3 Test suite

* Three perft tests are `#[ignore]`d with *"fails if it is run after the other tests"*
  (`board.rs:870-895`). That is **not** flakiness — it is §3.6 (global TT leaking into move
  generation). Fix that and these become reliable, and the move generator gets properly covered.
* The CI workflow runs plain `cargo test` — so the `--ignored` perft tests, the thing that actually
  proves move-generation correctness, **never run in CI**. Add a second step with
  `cargo test --release -- --ignored`.
* No `cargo clippy -- -D warnings` or `cargo fmt --check` in CI.
* The engine tests (`engine.rs:390-447`) run a 5-second search each and assert on the chosen move.
  They're useful as regression smoke tests but they're slow, non-deterministic (parallel + `f32`),
  and will randomly fail on slower CI hardware. Consider moving them behind `--ignored` and adding
  a fast, deterministic EPD tactical suite instead.

### 7.4 Observability

* `info` lines carry only `depth`, `time`, `score cp` — no `nodes`, `nps`, `pv`, `hashfull`,
  `seldepth`. Without `nodes`/`nps` you cannot tell whether a change made the search *smarter* or
  merely *faster*, which makes every optimisation below unmeasurable.
* `count_moves` `println!`s debug output (`count.rs:19-21`, `34-39`) onto **stdout**, the same
  channel as UCI. Harmless today (only reachable via the `debug` command) but it's a trap — debug
  output belongs on stderr or behind `info string`.

### 7.5 Minor

* Typos throughout, including in public identifiers: `douplicate` (× 6), `cound_of_checks`,
  `beeing`, `prunning`, `convertion`, `knigh`, `itter_direction`, `rochade`/`lange rutsche`
  (mixed German/English comments).
* `src/evaluation/mod.rs:1`: `use std::{u64, usize};` — deprecated no-op imports.
* `Transposition::hash` is redundant with the `DashMap` key (it becomes essential once you move to
  the array TT in §6.2 — keep the field, it's the verification you'll need).
* Opening book: `create_opening_map` **panics** if `openings.txt` is missing (`opening.rs:41`), so
  the engine cannot start without it. README acknowledges this. Degrade gracefully to "no book"
  instead — the `panic!` happens inside `lazy_static` initialisation, which means it fires at a
  surprising moment (first `go`), not at startup.
* Book keys and en passant: the book is keyed by a Zobrist that includes the ep square, while
  `possible_future_en_passant` sets the ep key after **every** double push, even when no enemy pawn
  can capture. If the book FENs use the "only when capturable" convention, those positions silently
  miss. The comment at `opening.rs:17` suggests you've already hit this. Fix both sides by only
  XOR-ing the ep key when an enemy pawn actually attacks the square — this also increases TT hit
  rate across the board.

---

## 8. Suggested roadmap

Work top-down; each block is independently shippable and testable.

### Sprint 1 — stop losing games you've already won (hours, not days)

1. §2.1 lowercase promotion output
2. §2.2 accept `n` in the promotion regex
3. §2.7 never emit `a1a1`
4. §2.4 don't panic on `stop` / `setoption` / blank lines
5. §2.9 `saturating_sub` on `movetime`
6. §2.6 clear castling rights on promotion-captures
7. §2.3 support `position fen`
8. §2.8 handle a mated root

> No Elo "gain" here in the engine sense — but these are forfeits and corrupted games, which cost
> more rating than any search feature will win back.

### Sprint 2 — fix the search you already have (~+250 to +500 Elo)

9. §3.1 correct TT flags against `alpha_orig` ← **start here**
10. §3.2 never store aborted nodes
11. §3.6 don't inject unverified TT moves → un-ignore the perft tests → **run them**
12. §3.3 real root move ordering across ID iterations
13. §3.4 stop discarding odd iterations
14. §5.1 king safety uses the right color's bitboard
15. §7.4 report `nodes` / `nps` / `pv` so everything after this is measurable

### Sprint 3 — the standard toolkit (~+300 to +600 Elo)

16. §4.1 real quiescence with stand-pat + delta pruning
17. §4.2 MVV-LVA, then SEE
18. §4.3 killers + history
19. §3.5 PVS + aspiration windows (§4.7)
20. §4.4 null-move pruning
21. §4.6 check extensions
22. §4.5 LMR (only after 17 + 18 are in — LMR with bad ordering *loses* Elo)

### Sprint 4 — speed (~+150 to +250 Elo)

23. §6.2 lock-free array TT with depth-preferred replacement + `setoption name Hash`
24. §6.1 `i32` centipawns
25. §6.3/§6.4 bitboard rays, `BETWEEN[64][64]`, allocation-free pin detection
26. §6.5 cheap `square_attacked_by` check detection
27. §5.5 pawn hash table

### Sprint 5 — evaluation (~+100 to +200 Elo)

28. §5.2 tapered eval
29. §5.4 bishop pair, isolated/backward pawns, rook on open file, king danger zone
30. §3.8 50-move rule + in-tree repetition detection
31. Texel tuning of all PSQT/term weights against a quiet-position dataset — once the terms exist,
    tuning typically finds another 50–100 Elo that hand-tuning cannot.

---

## 9. How to measure this properly

Right now there is no way to tell whether a change helped. Before Sprint 2, set this up:

**Correctness — perft.** You already have the positions. Make them deterministic (§3.6), run them
in CI at `--release`, and extend to depth 6 on the standard six positions. A move generator that
passes perft 6 everywhere is *proven* correct; nothing else gives you that.

**Strength — SPRT.** Use [fastchess](https://github.com/Disservin/fastchess) or `cutechess-cli`:

```bash
fastchess -engine cmd=./rustyrook_new name=new \
          -engine cmd=./rustyrook_old name=old \
          -each tc=10+0.1 proto=uci -rounds 2000 -repeat -concurrency 8 \
          -openings file=8moves_v3.pgn format=pgn order=random \
          -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05
```

Every change in Sprints 2–5 should pass an SPRT against the previous build. Note this *requires*
§2.3 (`position fen`) and §2.4 (no panic on `setoption`) — which is why they're in Sprint 1.

**Tactics — EPD suites.** WAC (300 positions, expect a strong engine to solve ~290) and ERET for
positional understanding. Fast, deterministic, and they catch search regressions that SPRT would
need thousands of games to notice.

**Node-count regression.** Record nodes-to-depth-8 on a fixed set of 20 positions. Search
*efficiency* improvements (ordering, TT, pruning) show up here instantly, while NPS changes don't —
which is exactly how you tell the two apart.

---

## 10. Summary

RustyRook is a well-organised, genuinely working chess engine with the right architectural bones:
magic bitboards, Zobrist hashing, iterative deepening, a transposition table, and a thoughtful
hand-built evaluation. The ~1800 Lichess blitz rating is real and earned.

The gap to 2200+ is not a rewrite. It is, in order:

1. **Eight UCI-layer bugs** that corrupt state or emit illegal moves (§2) — these are costing
   rating points that have nothing to do with chess.
2. **One wrong comparison** in the transposition table flag logic (§3.1) that makes the TT return
   incorrect scores.
3. **A quiescence search that isn't one** (§4.1), plus the standard move-ordering toolkit
   (§4.2–4.3) that every engine above 2000 has.
4. **`DashMap` in the hot path** (§6.2) suppressing NPS by an estimated 2–5×.

Items 1 and 2 are an afternoon's work each and should happen before anything else.
