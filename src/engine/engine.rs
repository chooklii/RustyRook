use std::cmp::Reverse;

use crate::{
    board::board::Chessboard,
    evaluation::{evaluate, Evaluation},
    figures::color::Color,
};

use super::sender::send_move;

#[derive(Debug)]
pub struct PossibleMove {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct MoveWithRating {
    from: usize,
    to: usize,
    rating: Evaluation,
}

pub fn search_for_best_move(board: &Chessboard) {
    let max_depth: u8 = 5;
    if let Some(best_move) = calculate_move(board, max_depth, 1) {
        send_move(&best_move.from, &best_move.to);
    }
}

fn calculate_move(board: &Chessboard, max_depth: u8, depth: u8) -> Option<MoveWithRating> {
    let moves: Vec<PossibleMove> = get_all_possible_moves(&board);
    let mut ratings_for_each_move = Vec::new();

    for single in moves.iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to);
        // evaluate - if own check kick
        if depth < max_depth {
            if let Some(move_evaluation) = calculate_move(&new_board, max_depth, depth + 1) {
                ratings_for_each_move.push(MoveWithRating {
                    from: single.from,
                    to: single.to,
                    rating: move_evaluation.rating,
                });
            }
        } else {
            ratings_for_each_move.push(MoveWithRating{
                from: single.from,
                to: single.to,
                rating: evaluate(&new_board)}
            );
        }
    }
    get_best_move(ratings_for_each_move, &board.current_move)
}

fn get_best_move(mut moves: Vec<MoveWithRating>, turn: &Color) -> Option<MoveWithRating> {
    moves.sort_by_key(|d| d.rating);
    let best_move = match turn {
        Color::White => moves.first(),
        Color::Black => moves.last()
    };
    best_move.cloned()
}

fn get_all_possible_moves(board: &Chessboard) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in board.get_next_player_figures().iter() {
        val.possible_moves(board, &key)
            .iter()
            .for_each(|single_move| {
                moves.push(PossibleMove {
                    from: key.clone(),
                    to: single_move.clone(),
                })
            });
    }
    moves
}
