# RustyRook
Chess Engine written in Rust

This Engine has no UI - It implements the UCI Chess Protocol. Use the [Lichess Bot](https://lichess.org/@/RustyRookChessBot) or any other Chess Software to play against it. 

use 
```
cargo build 
or
cargo build --release
```

to create an executable to use with your local chess software.

To verify the correctness of changes run the Unit-Tests

```
cargo test
```

Advanced Unit-Tests based on [Chess Programming Wiki](https://www.chessprogramming.org/Perft_Results) to verify the correctness of the move generation can be used with

```
cargo test -- --ignored
```

## Chess Programming
Rusty Rook implements Basic Chess Engine Algorithms like

- Minimax
- Alpha Beta Prunning
- Magic Bitboards
- Parallel Iterative Deepening with prev. best Move sequentially
- Transpositional Table
- Some Move Ordering

___

Possible Improvements for the Future

- Refactor Pinned Pieces
- Refactor En-Passant (Does not effect performance)
- Improve Move-Ordering
- Add more Stuff like Killer Moves & Null Moves
- Improve convertion from UCI-Position to Position usize (a1 -> 0)
