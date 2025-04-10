use std::time::SystemTime;
use std::cmp;
use rustc_hash::FxHashMap;

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

const MAX_DEPTH: u8 = 4;
const MAX_TAKES_DEPTH: u8 = 4;

pub fn search_for_best_move(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) {
    let now = SystemTime::now();
    if let (Some(best_move), calculations) = calculate(&board, &moves_by_field,  -1000,  1000, 1) {
        println!(
            "Calculated Positions {} and took {:?}",
            calculations,
            now.elapsed(),
        );
        println!("Best Move Net Rating {:?}", &best_move.rating);
        send_move(&best_move.from, &best_move.to);
    }
}

fn lost_game(best_move_rating: i16) -> Option<MoveWithRating> {
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
            net_rating: 0,
            ..Default::default()
        },
    });
}

fn calculate(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    mut alpha: i16,
    mut beta: i16,
    depth: u8,
) -> (Option<MoveWithRating>, u64) {
    let mut best_move_rating: i16 = init_best_move(&board.current_move);
    let mut best_move: Option<MoveWithRating> = None;
    let mut calculated_positions: u64 = 0;

    let (valid_moves, is_in_check) = get_valid_moves_in_position(&board, &moves_by_field);

    if is_in_check && valid_moves.is_empty() {
        return (lost_game(best_move_rating), 1);
    } else if valid_moves.is_empty() && !is_in_check {
        return (draw(), 1);
    }

    for single in valid_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if depth < MAX_DEPTH {
            if let (Some(move_evaluation), calculated_moves) =
                calculate(&new_board, &moves_by_field, alpha, beta, depth + 1)
            {
                calculated_positions += calculated_moves;

                match board.current_move{
                    Color::Black => {
                        if beta > move_evaluation.rating.net_rating{
                            beta = move_evaluation.rating.net_rating;
                        }
                        beta = cmp::min(beta, move_evaluation.rating.net_rating);
                    },
                    Color::White => {
                        alpha = cmp::max(alpha, move_evaluation.rating.net_rating);
                    }
                }

                let breaking = match board.current_move{
                    Color::White => move_evaluation.rating.net_rating > beta,
                    Color::Black => move_evaluation.rating.net_rating < alpha
                };

                if breaking{
                    break;
                }

                if check_if_is_better_move(
                    &board.current_move,
                    best_move_rating,
                    move_evaluation.rating.net_rating,
                ) {
                    best_move_rating = move_evaluation.rating.net_rating;
                    best_move = Some(MoveWithRating {
                        from: single.from,
                        to: single.to,
                        rating: move_evaluation.rating,
                    });
                }
            }
        } else {
            let deeper_evaluation = calculate_takes_only(&board, &moves_by_field, alpha, beta, 1);
            let evaluation = deeper_evaluation.unwrap_or_else(|| evaluate(&new_board));
            calculated_positions += 1;
            if check_if_is_better_move(&board.current_move, best_move_rating, evaluation.net_rating)
            {
                best_move_rating = evaluation.net_rating;
                best_move = Some(MoveWithRating {
                    from: single.from,
                    to: single.to,
                    rating: evaluation,
                });
            }
        }
    }

    return (best_move, calculated_positions);
}

fn calculate_takes_only(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    mut alpha: i16,
    mut beta: i16,
    depth: u8,
) -> Option<Evaluation> {
    let mut best_move_rating: i16 = init_best_move(&board.current_move);
    let mut best_move: Option<Evaluation> = None;
    let (valid_moves, is_in_check) = get_takes_in_position(&board, &moves_by_field);

    if is_in_check && valid_moves.is_empty() {
        return Some(Evaluation {
            net_rating: best_move_rating,
            ..Default::default()
        });
    } else if valid_moves.is_empty() && !is_in_check {
        return Some(Evaluation {
            net_rating: 0,
            ..Default::default()
        });
    }

    for single in valid_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if depth < MAX_TAKES_DEPTH {
            if let Some(move_evaluation) =
                calculate_takes_only(&new_board, &moves_by_field, alpha, beta,depth + 1)
            {

                match board.current_move{
                    Color::Black => {
                        if beta > move_evaluation.net_rating{
                            beta = move_evaluation.net_rating;
                        }
                        beta = cmp::min(beta, move_evaluation.net_rating);
                    },
                    Color::White => {
                        alpha = cmp::max(alpha, move_evaluation.net_rating);
                    }
                }

                let breaking = match board.current_move{
                    Color::White => move_evaluation.net_rating > beta,
                    Color::Black => move_evaluation.net_rating < alpha
                };

                if breaking{
                    break;
                }

                if check_if_is_better_move(
                    &board.current_move,
                    best_move_rating,
                    move_evaluation.net_rating,
                ) {
                    best_move_rating = move_evaluation.net_rating;
                    best_move = Some(move_evaluation);
                }
            }
        }
        // if maximum has completed return
        let evaluation = evaluate(&new_board);
        if check_if_is_better_move(&board.current_move, best_move_rating, evaluation.net_rating) {
            best_move_rating = evaluation.net_rating;
            best_move = Some(evaluation);
        }
    }
    return best_move;
}

fn init_best_move(turn: &Color) -> i16 {
    match turn {
        Color::White => -30000,
        Color::Black => 30000,
    }
}

fn check_if_is_better_move(turn: &Color, prev: i16, new: i16) -> bool {
    match turn {
        Color::White => new > prev,
        Color::Black => new < prev,
    }
}
