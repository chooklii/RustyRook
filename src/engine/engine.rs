use log::info;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;

use crate::engine::transposition::transposition::Flag;
use crate::TRANSPOSITION_TABLE;
use crate::{
    board::{board::Chessboard, promotion::Promotion},
    evaluation::evaluate,
    figures::color::Color,
};

use super::transposition::table::get_entry;
use super::{
    moves::get_valid_moves_in_position,
    sender::send_move,
    transposition::{transposition::Transposition},
};

#[derive(Debug, Clone, Copy)]
pub struct PossibleMove {
    pub from: usize,
    pub to: usize,
    pub promoted_to: Option<Promotion>,
}

impl Default for PossibleMove {
    fn default() -> PossibleMove {
        PossibleMove {
            from: 0,
            to: 0,
            promoted_to: None,
        }
    }
}

#[derive(Debug, Clone)]
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

const MAX_DEPTH: u8 = 4;
const MAX_DEPTH_TAKES: u8 = 4;

pub fn search_for_best_move(
    board: &Chessboard,
    repetition_is_possible: bool,
    twice_played_moved: &Vec<u64>,
) {
    let now = SystemTime::now();

    let best_move = calculate_root_level(
        board.clone(),
        true,
        repetition_is_possible,
        twice_played_moved.clone(),
    );
    println!(
        "Calculated Positions to depth {} and took {:?} - Net Rating: {}",
        MAX_DEPTH,
        now.elapsed(),
        best_move.rating
    );
    info!(
        "Calculated Positions to depth {} and took {:?} - Net Rating: {}",
        MAX_DEPTH,
        now.elapsed(),
        best_move.rating
    );
    send_move(&best_move.from, &best_move.to, &best_move.promoted_to);
}

// depth to go for m8 in 1 instead of m8 in 3
fn lost_game_evaluation(color: &Color, depth: u8) -> f32 {
    return match color {
        Color::White => -3000.0 + depth as f32,
        Color::Black => 3000.0 - depth as f32,
    };
}

fn lost_game(color: &Color, depth: u8) -> MoveWithRating {
    return MoveWithRating {
        rating: lost_game_evaluation(&color, depth),
        ..Default::default()
    };
}

fn draw() -> MoveWithRating {
    return MoveWithRating {
        rating: 0.0,
        ..Default::default()
    };
}

fn init_best_move(board: &Chessboard, calculate_all_moves: bool) -> f32 {
    if !calculate_all_moves {
        return evaluate(&board);
    }
    match board.current_move {
        Color::White => -3001.0,
        Color::Black => 3001.0,
    }
}

fn calculate_root_level(
    board: Chessboard,
    calculate_all_moves: bool,
    repetition_is_possible: bool,
    twice_played_moved: Vec<u64>,
) -> MoveWithRating {
    let (tx, rx) = mpsc::channel();

    let maximizing = board.current_move.eq(&Color::White);
    let alpha = -3000.0;
    let beta = 3000.0;
    let best_move_rating = match board.current_move {
        Color::White => -3001.0,
        Color::Black => 3001.0,
    };
    let mut best_move: MoveWithRating = MoveWithRating {
        rating: best_move_rating,
        ..Default::default()
    };
    
    let (valid_moves, _) =
        get_valid_moves_in_position(&board, calculate_all_moves);


    thread::spawn(move|| {
        valid_moves.par_iter().for_each(|single| {
            let mut new_board = board.clone();
            new_board.move_figure(single.from, single.to, single.promoted_to);
            let root_move = calculate(
                &new_board,
                !maximizing,
                alpha,
                beta,
                1,
                calculate_all_moves,
                repetition_is_possible,
                &twice_played_moved,
            );
            let _ = tx.send(MoveWithRating { from: single.from, to: single.to, promoted_to: single.promoted_to, rating: root_move.rating });
        });
    });

    for received in rx {
        if maximizing && received.rating > best_move.rating{
            best_move = received;
        }
        else if !maximizing && received.rating < best_move.rating{
            best_move = received;
        }
    };
    return best_move;
}

fn calculate(
    board: &Chessboard,
    maximizing: bool,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
    calculate_all_moves: bool,
    repetition_is_possible: bool,
    twice_played_moved: &Vec<u64>,
) -> MoveWithRating {
    if depth == MAX_DEPTH && calculate_all_moves {
        return calculate(
            &board,
            maximizing,
            alpha,
            beta,
            0,
            false,
            repetition_is_possible,
            twice_played_moved,
        );
    }
    if depth == MAX_DEPTH_TAKES && !calculate_all_moves {
        let evaluation = evaluate(&board);
        // init without a best move is no issue as long as we calculate more than depth = 1
        return MoveWithRating {
            rating: evaluation,
            ..Default::default()
        };
    }
    let depth_to_end = if calculate_all_moves {
        MAX_DEPTH + MAX_DEPTH_TAKES - depth
    } else {
        MAX_DEPTH_TAKES - depth
    };
    // we dont want to repeat over and over the same move - better to calculate again if we are at depth 0
    // transposition has values - no need to calculate again!
    if depth != 0 || !calculate_all_moves {
        if let Some(val) = get_entry(board.zobrist_key, depth_to_end, alpha, beta) {
            return MoveWithRating {
                from: val.best_move.from,
                to: val.best_move.to,
                promoted_to: val.best_move.promoted_to,
                rating: val.evaluation,
            };
        }
    }
    let mut best_move_rating = init_best_move(&board, calculate_all_moves);
    let (valid_moves, is_in_check) =
        get_valid_moves_in_position(&board, calculate_all_moves);
    if is_in_check && valid_moves.is_empty() {
        return lost_game(&board.current_move, depth);
    } else if calculate_all_moves && valid_moves.is_empty() && !is_in_check {
        return draw();
    } else if valid_moves.is_empty() && !is_in_check {
        // in this case no draw just no takes left to be checked
        return MoveWithRating {
            rating: best_move_rating,
            ..Default::default()
        };
    }
    // track if it was cut or if calculation is exact
    let mut transposition_flag = Flag::Exact;
    let mut best_move: MoveWithRating = MoveWithRating {
        rating: best_move_rating,
        ..Default::default()
    };
    for single in valid_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);
        // white
        if maximizing {
            // check for repetition
            let evaluation =
                if repetition_is_possible && twice_played_moved.contains(&new_board.zobrist_key) {
                    MoveWithRating {
                        rating: 0.0,
                        ..Default::default()
                    }
                } else {
                    calculate(
                        &new_board,
                        !maximizing,
                        alpha,
                        beta,
                        depth + 1,
                        calculate_all_moves,
                        repetition_is_possible,
                        twice_played_moved,
                    )
                };
            if best_move_rating < evaluation.rating {
                best_move_rating = evaluation.rating;
                best_move = MoveWithRating {
                    from: single.from,
                    to: single.to,
                    promoted_to: single.promoted_to,
                    rating: evaluation.rating,
                }
            }
            alpha = alpha.max(evaluation.rating);
            if beta <= alpha {
                transposition_flag = Flag::Upperbound;
                break;
            }
        // black
        } else {
            let evaluation =
                if repetition_is_possible && twice_played_moved.contains(&new_board.zobrist_key) {
                    MoveWithRating {
                        rating: 0.0,
                        ..Default::default()
                    }
                } else {
                    calculate(
                        &new_board,
                        !maximizing,
                        alpha,
                        beta,
                        depth + 1,
                        calculate_all_moves,
                        repetition_is_possible,
                        twice_played_moved,
                    )
                };

            if best_move_rating > evaluation.rating {
                best_move_rating = evaluation.rating;
                best_move = MoveWithRating {
                    from: single.from,
                    to: single.to,
                    promoted_to: single.promoted_to,
                    rating: evaluation.rating,
                }
            }
            beta = beta.min(evaluation.rating);
            if beta <= alpha {
                transposition_flag = Flag::Lowerbound;
                break;
            }
        }
    }
    // best move is 0 -> 0 if we only calculate some moves and none of them is good
    if best_move.from != 0 || best_move.to != 0 {
        TRANSPOSITION_TABLE.insert(board.zobrist_key, Transposition {
            hash: board.zobrist_key,
            depth: depth_to_end,
            evaluation: best_move_rating,
            best_move: PossibleMove {
                from: best_move.from,
                to: best_move.to,
                promoted_to: best_move.promoted_to,
            },
            flag: transposition_flag,
        });
    }
    return best_move;
}
