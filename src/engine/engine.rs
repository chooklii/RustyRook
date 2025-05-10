use std::time::SystemTime;

use log::info;

use crate::transposition::transposition::Flag;
use crate::{
    board::{board::Chessboard, promotion::Promotion},
    evaluation::evaluate,
    figures::color::Color,
};

use super::{
    moves::get_valid_moves_in_position,
    sender::send_move,
    transposition::{table::TranspositionTable, transposition::Transposition},
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
    transposition: &mut TranspositionTable,
    repetition_is_possible: bool,
    twice_played_moved: &Vec<u64>,
) {
    let now = SystemTime::now();

    let maximizing = board.current_move.eq(&Color::White);
    let alpha = -3000.0;
    let beta = 3000.0;
    let (best_move, calculations) = calculate(
        &board,
        transposition,
        maximizing,
        alpha,
        beta,
        0,
        true,
        repetition_is_possible,
        twice_played_moved,
    );
    println!(
        "Calculated Positions {} and took {:?} - Net Rating: {}",
        calculations,
        now.elapsed(),
        best_move.rating
    );
    info!(
        "Calculated Positions {} and took {:?} - Net Rating: {}",
        calculations,
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

fn calculate(
    board: &Chessboard,
    transposition: &mut TranspositionTable,
    maximizing: bool,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
    calculate_all_moves: bool,
    repetition_is_possible: bool,
    twice_played_moved: &Vec<u64>,
) -> (MoveWithRating, u64) {
    if depth == MAX_DEPTH && calculate_all_moves {
        let (move_with_rating, calculated_moves) = calculate(
            &board,
            transposition,
            maximizing,
            alpha,
            beta,
            0,
            false,
            repetition_is_possible,
            twice_played_moved,
        );
        return (move_with_rating, calculated_moves);
    }
    if depth == MAX_DEPTH_TAKES && !calculate_all_moves {
        let evaluation = evaluate(&board);
        // init without a best move is no issue as long as we calculate more than depth = 1
        return (
            MoveWithRating {
                rating: evaluation,
                ..Default::default()
            },
            1,
        );
    }
    let depth_to_end = if calculate_all_moves {
        MAX_DEPTH + MAX_DEPTH_TAKES - depth
    } else {
        MAX_DEPTH_TAKES - depth
    };
    // transposition has values - no need to calculate again!
    if depth != 0 || !calculate_all_moves {
        // we dont want to repeat over and over the same move - better to calculate again if we are at depth 0
        if let Some(val) = transposition.get_entry(board.zobrist_key, depth_to_end, alpha, beta) {
            return (
                MoveWithRating {
                    from: val.best_move.from,
                    to: val.best_move.to,
                    promoted_to: val.best_move.promoted_to,
                    rating: val.evaluation,
                },
                0,
            );
        }
    }
    let mut calculated_positions: u64 = 0;
    let mut best_move_rating = init_best_move(&board, calculate_all_moves);
    let (valid_moves, is_in_check) =
        get_valid_moves_in_position(&board, &transposition, calculate_all_moves);
    if is_in_check && valid_moves.is_empty() {
        return (lost_game(&board.current_move, depth), 1);
    } else if calculate_all_moves && valid_moves.is_empty() && !is_in_check {
        return (draw(), 1);
    } else if valid_moves.is_empty() && !is_in_check {
        // in this case no draw just no takes left to be checked
        return (
            MoveWithRating {
                rating: best_move_rating,
                ..Default::default()
            },
            1,
        );
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
            let (evaluation, calculated_moves) =
                if repetition_is_possible && twice_played_moved.contains(&new_board.zobrist_key) {
                    (
                        MoveWithRating {
                            rating: 0.0,
                            ..Default::default()
                        },
                        1,
                    )
                } else {
                    calculate(
                        &new_board,
                        transposition,
                        !maximizing,
                        alpha,
                        beta,
                        depth + 1,
                        calculate_all_moves,
                        repetition_is_possible,
                        twice_played_moved,
                    )
                };

            calculated_positions += calculated_moves;
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
            let (evaluation, calculated_moves) =
                if repetition_is_possible && twice_played_moved.contains(&new_board.zobrist_key) {
                    (
                        MoveWithRating {
                            rating: 0.0,
                            ..Default::default()
                        },
                        1,
                    )
                } else {
                    calculate(
                        &new_board,
                        transposition,
                        !maximizing,
                        alpha,
                        beta,
                        depth + 1,
                        calculate_all_moves,
                        repetition_is_possible,
                        twice_played_moved,
                    )
                };
            calculated_positions += calculated_moves;

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
        transposition.save_entry(Transposition {
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
    return (best_move, calculated_positions);
}
