use rustc_hash::FxHashMap;
use std::time::SystemTime;

use crate::{
    board::{board::Chessboard, promotion::Promotion},
    evaluation::{evaluate, Evaluation},
    figures::color::Color,
    helper::moves_by_field::MoveInEveryDirection,
};

use super::{
    moves::{get_takes_in_position, get_valid_moves_in_position},
    sender::send_move,
};

#[derive(Debug)]
pub struct PossibleMove {
    pub from: usize,
    pub to: usize,
    pub promoted_to: Option<Promotion>,
}

#[derive(Debug, Clone)]
pub struct MoveWithRating {
    from: usize,
    to: usize,
    promoted_to: Option<Promotion>,
    rating: Evaluation,
}

impl Default for MoveWithRating {
    fn default() -> MoveWithRating {
        MoveWithRating {
            from: 0,
            to: 0,
            promoted_to: None,
            rating: Evaluation {
                white_pieces_value: 0.0,
                black_pieces_value: 0.0,
                net_rating: 0.0,
            },
        }
    }
}

const MAX_DEPTH: u8 = 4;
const MAX_DEPTH_TAKES: u8 = 4;

pub fn search_for_best_move(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) {
    let now = SystemTime::now();

    let maximizing = board.current_move.eq(&Color::White);
    let (best_move, calculations) = calculate(&board, &moves_by_field, maximizing, -3000.0, 3000.0, 0);
    println!(
        "Calculated Positions {} and took {:?}",
        calculations,
        now.elapsed(),
    );
    println!("Best Move Net Rating {:?}", &best_move.rating);
    send_move(&best_move.from, &best_move.to, &best_move.promoted_to);
}

// depth to go for m8 in 1 instead of m8 in 3
fn lost_game_evaluation(color: &Color, depth: u8) -> Evaluation {
    let rating = match color {
        Color::White => -3000.0 + depth as f32,
        Color::Black => 3000.0 - depth as f32,
    };
    Evaluation {
        net_rating: rating,
        ..Default::default()
    }
}

fn lost_game(color: &Color, depth: u8) -> MoveWithRating {
    return MoveWithRating {
        rating: lost_game_evaluation(&color, depth),
        ..Default::default()
    };
}

fn draw() -> MoveWithRating {
    return MoveWithRating {
        rating: Evaluation {
            net_rating: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };
}

fn calculate(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    maximizing: bool,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
) -> (MoveWithRating, u64) {
    if depth == MAX_DEPTH {
        let (evaluation, calculated_moves) =
            calculate_takes_only(&board, &moves_by_field,maximizing, alpha, beta, 0);
        return (
            MoveWithRating {
                rating: evaluation,
                ..Default::default()
            },
            calculated_moves,
        );
    }

    let mut calculated_positions: u64 = 0;
    let mut best_move_rating = init_best_move(&board.current_move);
    let mut best_move: MoveWithRating = MoveWithRating {
        ..Default::default()
    };
    let (valid_moves, is_in_check) = get_valid_moves_in_position(&board, &moves_by_field);

    if is_in_check && valid_moves.is_empty() {
        return (lost_game(&board.current_move, depth), 1);
    } else if valid_moves.is_empty() && !is_in_check {
        return (draw(), 1);
    }
    for single in valid_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if maximizing {
            // white
            let (evaluation, calculated_moves) =
                calculate(&new_board, &moves_by_field,!maximizing, alpha, beta, depth + 1);
            calculated_positions += calculated_moves;

            if best_move_rating < evaluation.rating.net_rating {
                best_move_rating = evaluation.rating.net_rating;
                best_move = MoveWithRating {
                    from: single.from,
                    to: single.to,
                    promoted_to: single.promoted_to,
                    rating: evaluation.rating,
                }
            }
            alpha = alpha.max(evaluation.rating.net_rating);

            if beta <= alpha {
                break;
            }
        } else {
            // black
            let (evaluation, calculated_moves) =
                calculate(&new_board, &moves_by_field,!maximizing, alpha, beta, depth + 1);
            calculated_positions += calculated_moves;

            if best_move_rating > evaluation.rating.net_rating {
                best_move_rating = evaluation.rating.net_rating;
                best_move = MoveWithRating {
                    from: single.from,
                    to: single.to,
                    promoted_to: single.promoted_to,
                    rating: evaluation.rating,
                }
            }
            beta = beta.min(evaluation.rating.net_rating);
            if beta <= alpha {
                break;
            }
        }
    }
    return (best_move, calculated_positions);
}

fn calculate_takes_only(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    maximizing: bool,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
) -> (Evaluation, u64) {
    // performance reasons
    if depth == MAX_DEPTH_TAKES {
        return (evaluate(&board, &moves_by_field), 1);
    }
    let mut best_move_evaluation: Evaluation = evaluate(&board, &moves_by_field);
    let mut best_move_rating = best_move_evaluation.net_rating;
    let mut calculated_positions: u64 = 0;

    let (takes_moves, is_in_check) = get_takes_in_position(&board, &moves_by_field);
    if is_in_check && takes_moves.is_empty() {
        return (
            lost_game_evaluation(&board.current_move, MAX_DEPTH_TAKES + depth),
            1,
        );
    } else if takes_moves.is_empty() && !is_in_check {
        // in this case no draw just no takes left to be checked
        return (best_move_evaluation, 1);
    }
    for single in takes_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if maximizing {
            // white
            let (evaluation, calculated_moves) =
                calculate_takes_only(&new_board, &moves_by_field,!maximizing, alpha, beta, depth + 1);
            calculated_positions += calculated_moves;

            if best_move_rating < evaluation.net_rating {
                best_move_rating = evaluation.net_rating;
                best_move_evaluation = evaluation;
            }
            alpha = alpha.max(evaluation.net_rating);

            if beta <= alpha {
                break;
            }
        } else {
            // black
            let (evaluation, calculated_moves) =
                calculate_takes_only(&new_board, &moves_by_field,!maximizing, alpha, beta, depth + 1);
            calculated_positions += calculated_moves;
            if best_move_rating > evaluation.net_rating {
                best_move_rating = evaluation.net_rating;
                best_move_evaluation = evaluation;
            }
            beta = beta.min(evaluation.net_rating);
            if beta <= alpha {
                break;
            }
        }
    }
    return (best_move_evaluation, calculated_positions);
}

fn init_best_move(turn: &Color) -> f32 {
    match turn {
        Color::White => -3001.0,
        Color::Black => 3001.0,
    }
}
