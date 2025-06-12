use log::info;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use crate::board::{board::Chessboard, promotion::Promotion};
use crate::engine::transposition::transposition::Flag;
use crate::evaluation::evaluate_for_own_color;
use crate::TRANSPOSITION_TABLE;

use super::transposition::table::get_entry;
use super::{
    moves::get_valid_moves_in_position, sender::send_move,
    transposition::transposition::Transposition,
};

const PLACEHOLDER_RATING: f32 = 5000.0;

#[derive(Debug, Clone, Copy, Default)]
pub struct PossibleMove {
    pub from: usize,
    pub to: usize,
    pub promoted_to: Option<Promotion>,
}

#[derive(Debug, Clone, Copy)]
pub struct MoveWithRating {
    from: usize,
    to: usize,
    promoted_to: Option<Promotion>,
    rating: f32,
}

impl Default for MoveWithRating {
    fn default() -> MoveWithRating {
        MoveWithRating {
            from: 0,
            to: 0,
            promoted_to: None,
            rating: 0.0,
        }
    }
}

pub fn search_for_best_move(
    time_for_move: u64,
    board: &Chessboard,
    repetition_is_possible: bool,
    twice_played_moved: &[u64],
) {
    let (best_move, depth) = calculate_root_level(
        time_for_move,
        *board,
        repetition_is_possible,
        twice_played_moved.to_owned(),
    );
    println!(
        "Calculated Positions to depth {} and took {:?}ms - Net Rating: {}",
        depth, time_for_move, best_move.rating
    );
    info!(
        "Calculated Positions to depth {} and took {:?}ms - Net Rating: {}",
        depth, time_for_move, best_move.rating
    );
    send_move(best_move.from, best_move.to, best_move.promoted_to);
}

fn lost_game(depth_to_end: u8) -> MoveWithRating {
    MoveWithRating {
        // m8 in 2 > m8 in 5
        rating: -3000.0 - depth_to_end as f32,
        ..Default::default()
    }
}

fn draw() -> MoveWithRating {
    MoveWithRating {
        rating: 0.0,
        ..Default::default()
    }
}

fn init_best_move(board: &Chessboard, calculate_all_moves: bool) -> f32 {
    if !calculate_all_moves {
        return evaluate_for_own_color(board);
    }
    -PLACEHOLDER_RATING
}

fn calculate_root_level(
    time_for_move: u64,
    board: Chessboard,
    repetition_is_possible: bool,
    twice_played_moved: Vec<u64>,
) -> (MoveWithRating, u8) {
    let (tx, rx) = mpsc::channel();
    let now = SystemTime::now();
    let timer = Arc::new(AtomicBool::new(false));
    let mut depth = 0;
    let best_move_rating = -PLACEHOLDER_RATING;
    let mut best_move: MoveWithRating = MoveWithRating {
        rating: best_move_rating,
        ..Default::default()
    };

    // iterative deepening
    let timer_clone = Arc::clone(&timer);
    thread::spawn(move || {
        iterative_deepening(
            board,
            repetition_is_possible,
            twice_played_moved.clone(),
            tx,
            timer_clone,
        )
    });
    // stop deepening after given time
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(time_for_move));
        timer.store(true, Ordering::SeqCst);
    });

    for received in rx {
        depth += 2;
        println!(
            "info depth {} time {} score cp {}",
            depth,
            now.elapsed().unwrap_or(Duration::new(0, 0)).as_millis(),
            (received.rating * 100.0).round()
        );
        best_move = received;
    }
    (best_move, depth)
}

fn iterative_deepening(
    board: Chessboard,
    repetition_is_possible: bool,
    twice_played_moved: Vec<u64>,
    tx: Sender<MoveWithRating>,
    timer_clone: Arc<AtomicBool>,
) {
    for max_depth in 2..=100 {
        let beta = PLACEHOLDER_RATING;
        if timer_clone.load(Ordering::Relaxed) {
            break;
        }
        let (mut valid_moves, _) = get_valid_moves_in_position(&board, true);

        // on odd numbers (we dont really care about, as they end with our move) calculate odd takes to end on opponent move
        let max_depth_takes = if max_depth % 2 == 0 { 4 } else { 3 };
        // calculate prev. best move sequential to get baseline alpha
        let first_move = valid_moves.remove(0);
        let mut new_board = board;
        new_board.move_figure(first_move.from, first_move.to, first_move.promoted_to);
        let first_move_calculation = calculate(
            &new_board,
            -PLACEHOLDER_RATING,
            beta,
            1,
            max_depth,
            max_depth_takes,
            true,
            repetition_is_possible,
            &twice_played_moved,
            &timer_clone,
            false
        );
        let alpha = -first_move_calculation.rating;
        let mut moves_with_rating: Vec<MoveWithRating> = valid_moves
            .par_iter()
            .map(|single| {
                let mut new_board = board;
                new_board.move_figure(single.from, single.to, single.promoted_to);
                let move_with_rating = calculate(
                    &new_board,
                    -beta,
                    -alpha,
                    1,
                    max_depth,
                    max_depth_takes,
                    true,
                    repetition_is_possible,
                    &twice_played_moved,
                    &timer_clone,
                    false
                );
                MoveWithRating {
                    from: single.from,
                    to: single.to,
                    promoted_to: single.promoted_to,
                    rating: -move_with_rating.rating,
                }
            })
            .collect();

        // add back best move
        moves_with_rating.push(MoveWithRating {
            from: first_move.from,
            to: first_move.to,
            promoted_to: first_move.promoted_to,
            rating: -first_move_calculation.rating,
        });

        // prevent sending not calculated moves
        if timer_clone.load(Ordering::Relaxed) {
            break;
        }
        if max_depth % 2 != 0 {
            // we only want calculations ending on opponent moves
            continue;
        }
        let depth_best_move_opt = moves_with_rating
            .iter()
            .max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());
        if let Some(depth_best_move) = depth_best_move_opt {
            let _ = tx.send(*depth_best_move);
        }
    }
    drop(tx)
}

fn calculate(
    board: &Chessboard,
    mut alpha: f32,
    beta: f32,
    depth: u8,
    max_depth: u8,
    max_depth_takes: u8,
    calculate_all_moves: bool,
    repetition_is_possible: bool,
    twice_played_moved: &Vec<u64>,
    timer: &AtomicBool,
    use_transposition: bool,
) -> MoveWithRating {
    // todo: check if timer can be removed from calculation as it is dropped in other position anyways?
    if timer.load(Ordering::Relaxed) || (depth == max_depth_takes && !calculate_all_moves) {
        let evaluation = evaluate_for_own_color(board);
        // init without a best move is no issue as long as we calculate more than depth = 1
        return MoveWithRating {
            rating: evaluation,
            ..Default::default()
        };
    }
    // calculate only takes
    if depth == max_depth && calculate_all_moves {
        return calculate(
            board,
            alpha,
            beta,
            0,
            max_depth,
            max_depth_takes,
            false,
            repetition_is_possible,
            twice_played_moved,
            timer,
            true,
        );
    }
    let depth_to_end = if calculate_all_moves {
        max_depth + max_depth_takes - depth
    } else {
        max_depth_takes - depth
    };
    if use_transposition {
        if let Some(val) = get_entry(board.zobrist_key, depth_to_end, alpha, beta) {
            // only use value from transposition if it does not result in a repetition
            if !(repetition_is_possible
                && results_in_repetition(val, &mut board.clone(), twice_played_moved))
            {
                return MoveWithRating {
                    from: val.best_move.from,
                    to: val.best_move.to,
                    promoted_to: val.best_move.promoted_to,
                    rating: val.evaluation,
                };
            }
        }
    }

    let mut best_move_rating = init_best_move(board, calculate_all_moves);
    let (valid_moves, is_in_check) = get_valid_moves_in_position(board, calculate_all_moves);
    if is_in_check && valid_moves.is_empty() {
        return lost_game(depth_to_end);
    } else if calculate_all_moves && valid_moves.is_empty() && !is_in_check {
        return draw();
    } else if valid_moves.is_empty() && !is_in_check {
        // in this case no draw just no takes left to be checked
        return MoveWithRating {
            rating: best_move_rating,
            ..Default::default()
        };
    }
    let mut transposition_flag = Flag::Exact;
    let mut best_move: MoveWithRating = MoveWithRating {
        rating: best_move_rating,
        ..Default::default()
    };
    for single in valid_moves.into_iter() {
        let mut new_board = *board;
        new_board.move_figure(single.from, single.to, single.promoted_to);

        // check for repetition
        let move_with_rating =
            if repetition_is_possible && twice_played_moved.contains(&new_board.zobrist_key) {
                MoveWithRating {
                    rating: 0.0,
                    ..Default::default()
                }
            } else {
                calculate(
                    &new_board,
                    -beta,
                    -alpha,
                    depth + 1,
                    max_depth,
                    max_depth_takes,
                    calculate_all_moves,
                    repetition_is_possible,
                    twice_played_moved,
                    timer,
                    true
                )
            };
        let adjusted_evaluation = -move_with_rating.rating;
        if best_move_rating < adjusted_evaluation {
            best_move_rating = adjusted_evaluation;
            best_move = MoveWithRating {
                from: single.from,
                to: single.to,
                promoted_to: single.promoted_to,
                rating: adjusted_evaluation,
            }
        }
        alpha = alpha.max(adjusted_evaluation);
        if alpha >= beta {
            break;
        }
    }
    // dont save best move from only takes in transposition table
    if calculate_all_moves {
        if best_move_rating <= alpha {
            transposition_flag = Flag::Upperbound;
        } else if best_move_rating >= beta {
            transposition_flag = Flag::Lowerbound;
        }

        if best_move.from == 0 && best_move.to == 0{
            // is going to stay here for some time to make sure there is no bug
            info!("Panic as bestMove is 0 -> 0");
            panic!("BestMove From and to was 0 - Should not happen!")
        }

        TRANSPOSITION_TABLE.insert(
            board.zobrist_key,
            Transposition {
                hash: board.zobrist_key,
                depth: depth_to_end,
                evaluation: best_move_rating,
                best_move: PossibleMove {
                    from: best_move.from,
                    to: best_move.to,
                    promoted_to: best_move.promoted_to,
                },
                flag: transposition_flag,
            },
        );
    }
    best_move
}

// if repetition is possible make move and check if it is a repetition
fn results_in_repetition(
    transposition: Transposition,
    board: &mut Chessboard,
    twice_played_moved: &Vec<u64>,
) -> bool {
    board.move_figure(
        transposition.best_move.from,
        transposition.best_move.to,
        transposition.best_move.promoted_to,
    );
    twice_played_moved.contains(&board.zobrist_key)
}

// test all kinds of positions which made problems during development

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sacking_queen() {
        // sacked queen by moving knight
        let position =
            String::from("r3k2r/pppq1pp1/2n1p2p/3p1b2/1b1PnP2/2N1P1B1/PPPQB1PP/R4KNR w kq - 8 11");

        let mut board = Chessboard {
            ..Default::default()
        };
        board.create_position_from_input_string(position);

        let (best_move, _) = calculate_root_level(5000, board, false, Vec::new());
        assert_ne!(best_move.from, 18);
    }

    #[test]
    fn test_not_taking_figure() {
        // does not take +3 figure
        let position =
            String::from("2r1kb1r/pppq1ppp/4p3/3pPb2/4NB2/4P3/PPPQBPPP/R3K2R b KQk - 0 11");

        let mut board = Chessboard {
            ..Default::default()
        };
        board.create_position_from_input_string(position);

        let (best_move, _) = calculate_root_level(5000, board, false, Vec::new());
        assert_eq!(best_move.to, 28);
    }

    #[test]
    fn test_sacking_rook() {
        // was sacking rook at d4
        let position = String::from("8/5ppp/2ppk3/P2p4/3Pr1b1/4B1R1/1r5P/2R3K1 b - - 5 45");

        let mut board = Chessboard {
            ..Default::default()
        };
        board.create_position_from_input_string(position);

        let (best_move, _) = calculate_root_level(5000, board, false, Vec::new());
        assert_ne!(best_move.to, 27);
    }

    #[test]
    fn test_sacking_knight() {
        // was sacking knight on a2
        let position =
            String::from("r2qkb1r/pppbpp1p/5np1/1B1p4/1n1P1B2/2N1P3/PPP1QPPP/R3K1NR b KQkq - 3 7");

        let mut board = Chessboard {
            ..Default::default()
        };
        board.create_position_from_input_string(position);

        let (best_move, _) = calculate_root_level(5000, board, false, Vec::new());
        assert_ne!(best_move.to, 8);
    }
}
