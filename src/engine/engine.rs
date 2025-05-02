use std::time::SystemTime;

use rustc_hash::FxHashMap;

use crate::{
    board::{board::Chessboard, promotion::Promotion},
    evaluation::evaluate,
    figures::color::Color,
};
use crate::transposition::transposition::Flag;

use super::{
    moves::{get_takes_in_position, get_valid_moves_in_position},
    sender::send_move, transposition::{self, table::TranspositionTable, transposition::Transposition},
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
            rating: 0.0
        }
    }
}

const MAX_DEPTH: u8 = 4;
const MAX_DEPTH_TAKES: u8 = 4;

pub fn search_for_best_move(
    board: &Chessboard,
    transposition: &mut TranspositionTable
) {
    let now = SystemTime::now();

    let maximizing = board.current_move.eq(&Color::White);
    let (best_move, calculations) = calculate(&board, transposition, maximizing,-3000.0, 3000.0, 0);
    println!(
        "Calculated Positions {} and took {:?}",
        calculations,
        now.elapsed(),
    );
    println!("Best Move Net Rating {:?}", &best_move.rating);
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

fn init_best_move(turn: &Color) -> f32 {
    match turn {
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
) -> (MoveWithRating, u64) {
    if depth == MAX_DEPTH {
        let (evaluation, calculated_moves) =
        // works as long as MAX_DEPTH_TAKES != 0 :D -- prob. needs to be improved
            calculate_takes_only(&board,transposition, maximizing, PossibleMove { ..Default::default() }, alpha, beta, 0);
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
    let (valid_moves, is_in_check) = get_valid_moves_in_position(&board);
    if is_in_check && valid_moves.is_empty() {
        return (lost_game(&board.current_move, depth), 1);
    } else if valid_moves.is_empty() && !is_in_check {
        return (draw(), 1);
    }
    let depth_to_end = MAX_DEPTH + MAX_DEPTH_TAKES - depth;
    for single in valid_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);
        // check transposition table
        if let Some(val) = transposition.get_entry(board.zobrist_key, depth_to_end, alpha, beta){
            if maximizing{
                alpha = alpha.max(val.evaluation);
                if best_move_rating < val.evaluation{
                    best_move = MoveWithRating {
                        from: single.from,
                        to: single.to,
                        promoted_to: single.promoted_to,
                        rating: val.evaluation
                    };
                    best_move_rating = val.evaluation;
                }
            } 
            if !maximizing{
                beta = beta.min(val.evaluation);
                if best_move_rating > val.evaluation{
                    best_move = MoveWithRating {
                        from: single.from,
                        to: single.to,
                        promoted_to: single.promoted_to,
                        rating: val.evaluation
                    };
                    best_move_rating = val.evaluation;
                }
            }
        }
        else if maximizing {
            // white
            let (evaluation, calculated_moves) =
                calculate(&new_board,transposition,!maximizing, alpha, beta, depth + 1);
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
                transposition.save_entry(Transposition { 
                    hash: new_board.zobrist_key, 
                    depth: depth_to_end, 
                    evaluation: evaluation.rating, 
                    best_move: PossibleMove { from: single.from, to: single.to, promoted_to: single.promoted_to }, 
                    flag: Flag::Upperbound });
                break;
            }
        } else {
            // black
            let (evaluation, calculated_moves) =
                calculate(&new_board,transposition,!maximizing, alpha, beta, depth + 1);
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
                transposition.save_entry(Transposition { 
                    hash: new_board.zobrist_key, 
                    depth: depth_to_end, 
                    evaluation: evaluation.rating, 
                    best_move: PossibleMove { from: single.from, to: single.to, promoted_to: single.promoted_to }, 
                    flag: Flag::Lowerbound });
                break;
            }
        }
    }
    if best_move.from != 0 || best_move.to != 0{
        transposition.save_entry(Transposition { 
            hash: board.zobrist_key, 
            depth: depth_to_end, 
            evaluation: best_move_rating, 
            best_move: PossibleMove { from: best_move.from, to: best_move.to, promoted_to: best_move.promoted_to }, 
            flag: Flag::Exact });
    }

    return (best_move, calculated_positions);
}

fn calculate_takes_only(
    board: &Chessboard,
    transposition: &mut TranspositionTable,
    maximizing: bool,
    last_move: PossibleMove,
    mut alpha: f32,
    mut beta: f32,
    depth: u8,
    // net rating, calculated_moves
) -> (f32, u64) {
    // no need to calculate 20 takes deep
    if depth == MAX_DEPTH_TAKES {
        let evaluation = evaluate(&board);
        transposition.save_entry(Transposition { 
            hash: board.zobrist_key, 
            depth: MAX_DEPTH_TAKES - depth, 
            evaluation, 
            best_move: last_move, 
            flag: Flag::Lowerbound });
        return (evaluation, 1);
    }
    let mut best_move_evaluation = evaluate(&board);
    let mut best_move: PossibleMove = PossibleMove {
        ..Default::default()
    };
    let mut calculated_positions: u64 = 0;

    let (takes_moves, is_in_check) = get_takes_in_position(&board);
    if is_in_check && takes_moves.is_empty() {
        return (
            lost_game_evaluation(&board.current_move, MAX_DEPTH_TAKES + depth),
            1,
        );
    } else if takes_moves.is_empty() && !is_in_check {
        // in this case no draw just no takes left to be checked
        return (best_move_evaluation, 1);
    }
    let depth_till_end = MAX_DEPTH_TAKES - depth;
    for single in takes_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if let Some(val) = transposition.get_entry(new_board.zobrist_key, depth_till_end, alpha, beta){
            if maximizing{
                alpha = alpha.max(val.evaluation);
                if best_move_evaluation < val.evaluation{
                    best_move_evaluation = val.evaluation;
                    best_move = val.best_move;
                }
            } 
            if !maximizing{
                beta = beta.min(val.evaluation);
                if best_move_evaluation > val.evaluation{
                    best_move_evaluation = val.evaluation;
                    best_move = val.best_move;
                }
            }
        }
        else if maximizing {
            // white
            let (evaluation, calculated_moves) =
                calculate_takes_only(&new_board,transposition,!maximizing, single, alpha, beta, depth + 1);
            calculated_positions += calculated_moves;

            if best_move_evaluation < evaluation {
                best_move_evaluation = evaluation;
                best_move = single;
            }
            alpha = alpha.max(evaluation);

            if beta <= alpha {
                transposition.save_entry(Transposition { 
                    hash: new_board.zobrist_key, 
                    depth: depth_till_end, 
                    evaluation, 
                    best_move: single, 
                    flag: Flag::Upperbound });
                break;
            }
        } else {
            // black
            let (evaluation, calculated_moves) =
                calculate_takes_only(&new_board,transposition,!maximizing, single,alpha, beta, depth + 1);
            calculated_positions += calculated_moves;
            if best_move_evaluation > evaluation {
                best_move_evaluation = evaluation;
                best_move = single;
            }
            beta = beta.min(evaluation);
            if beta <= alpha {
                transposition.save_entry(Transposition { 
                    hash: new_board.zobrist_key, 
                    depth: depth_till_end, 
                    evaluation, 
                    best_move: single, 
                    flag: Flag::Lowerbound });
                break;
            }
        }
    }
    if best_move.from != 0 || best_move.to != 0{
    // this is not really exact as it is only takes - maybe need to add 4th flag
    transposition.save_entry(Transposition { 
        hash: board.zobrist_key, 
        depth: depth_till_end, 
        evaluation: best_move_evaluation, 
        best_move, 
        flag: Flag::Exact });
    }

    return (best_move_evaluation, calculated_positions);
}