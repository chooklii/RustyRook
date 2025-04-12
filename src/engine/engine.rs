use rustc_hash::FxHashMap;
use std::{char::MAX, time::SystemTime};

use crate::{
    board::{board::Chessboard, promotion::Promotion},
    evaluation::{self, evaluate, Evaluation},
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
    rating: Evaluation,
}

impl Default for MoveWithRating {
    fn default() -> MoveWithRating {
        MoveWithRating {
            from: 0,
            to: 0,
            rating: Evaluation {
                white_pieces_value: 0.0,
                black_pieces_value: 0.0,
                net_rating: 0.0,
            },
        }
    }
}

const MAX_DEPTH: u8 = 4;
const MAX_TAKES_DEPTH: u8 = 4;

pub fn search_for_best_move(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) {
    let now = SystemTime::now();
    if let (Some(best_move), calculations) = calculate(&board, &moves_by_field, -3000.0, 3000.0, 0)
    {
        println!(
            "Calculated Positions {} and took {:?}",
            calculations,
            now.elapsed(),
        );
        println!("Best Move Net Rating {:?}", &best_move.rating);
        send_move(&best_move.from, &best_move.to);
    }
}

fn lost_game(best_move_rating: f32) -> Option<MoveWithRating> {
    return Some(MoveWithRating {
        from: 0,
        to: 0,
        rating: Evaluation {
            net_rating: best_move_rating,
            ..Default::default()
        },
    });
}

fn draw() -> Option<MoveWithRating> {
    return Some(MoveWithRating {
        from: 0,
        to: 0,
        rating: Evaluation {
            net_rating: 0.0,
            ..Default::default()
        },
    });
}

fn calculate(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
) -> (Option<MoveWithRating>, u64) {
    if depth == MAX_DEPTH{
        return calculate_takes_only(&board, &moves_by_field, alpha, beta, 0);
    }

    let mut best_move_rating = init_best_move(&board.current_move);
    let mut best_move: Option<MoveWithRating> = None;
    let mut calculated_positions: u64 = 0;

    let (valid_moves, is_in_check) = get_valid_moves_in_position(&board, &moves_by_field);

    if is_in_check && valid_moves.is_empty() {
        return (lost_game(best_move_rating), 1);
    } else if valid_moves.is_empty() && !is_in_check {
        return (draw(), 1);
    }

    let white_to_play = board.current_move.eq(&Color::White);
    for single in valid_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if white_to_play {
            // max
            if let (Some(evaluation), calculated_moves) =
                calculate(&new_board, &moves_by_field, alpha, beta, depth + 1)
            {
                calculated_positions += calculated_moves;

                if best_move_rating < evaluation.rating.net_rating {
                    best_move_rating = evaluation.rating.net_rating;
                    best_move = Some(MoveWithRating {
                        from: single.from,
                        to: single.to,
                        rating: evaluation.rating,
                    })
                }
                alpha = alpha.max(evaluation.rating.net_rating);

                if beta <= alpha {
                    break;
                }
            }
        } else {
            // mini
            if let (Some(evaluation), calculated_moves) =
                calculate(&new_board, &moves_by_field, alpha, beta, depth + 1)
            {
                calculated_positions += calculated_moves;

                if best_move_rating > evaluation.rating.net_rating {
                    best_move_rating = evaluation.rating.net_rating;
                    best_move = Some(MoveWithRating {
                        from: single.from,
                        to: single.to,
                        rating: evaluation.rating,
                    })
                }
                beta = beta.min(evaluation.rating.net_rating);
                if beta <= alpha {
                    break;
                }
            }
        }
    }
    return (best_move, calculated_positions);
}

fn calculate_takes_only(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
) -> (Option<MoveWithRating>, u64) {
    if depth == MAX_TAKES_DEPTH{
        return (Some(MoveWithRating { rating: evaluate(&board), ..Default::default()}),1)
    }

    let mut best_move_rating = init_best_move(&board.current_move);
    let mut best_move: Option<MoveWithRating> = Some(MoveWithRating { rating: evaluate(&board), ..Default::default()});
    let mut calculated_positions: u64 = 0;

    let (takes_moves, is_in_check) = get_takes_in_position(&board, &moves_by_field);
    
    if is_in_check && takes_moves.is_empty() {
        return (lost_game(best_move_rating), 1);
    } else if takes_moves.is_empty() && !is_in_check {
        // in this case no draw just no takes left to be played
        return (best_move,1)
    }

    let white_to_play = board.current_move.eq(&Color::White);
    for single in takes_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if white_to_play {
            // max
            if let (Some(evaluation), calculated_moves) =
                calculate_takes_only(&new_board, &moves_by_field, alpha, beta, depth + 1)
            {
                calculated_positions += calculated_moves;

                if best_move_rating < evaluation.rating.net_rating {
                    best_move_rating = evaluation.rating.net_rating;
                    best_move = Some(MoveWithRating {
                        from: single.from,
                        to: single.to,
                        rating: evaluation.rating,
                    })
                }
                alpha = alpha.max(evaluation.rating.net_rating);

                if beta <= alpha {
                    break;
                }
            }
        } else {
            // mini
            if let (Some(evaluation), calculated_moves) =
                calculate_takes_only(&new_board, &moves_by_field, alpha, beta, depth + 1)
            {
                calculated_positions += calculated_moves;

                if best_move_rating > evaluation.rating.net_rating {
                    best_move_rating = evaluation.rating.net_rating;
                    best_move = Some(MoveWithRating {
                        from: single.from,
                        to: single.to,
                        rating: evaluation.rating,
                    })
                }
                beta = beta.min(evaluation.rating.net_rating);
                if beta <= alpha {
                    break;
                }
            }
        }
    }
    return (best_move, calculated_positions);
}

fn init_best_move(turn: &Color) -> f32 {
    match turn {
        Color::White => -300.0,
        Color::Black => 300.0,
    }
}

