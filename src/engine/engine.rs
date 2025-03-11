use std::{collections::{HashMap, HashSet}, time::SystemTime};

use crate::{
    board::board::Chessboard,
    evaluation::{evaluate, Evaluation},
    figures::{color::Color, figures::Figure},
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
    let now = SystemTime::now();
    let mut checked_positions:HashSet<String> = HashSet::new();
    if let (Some(best_move), calculations) = calculate_move(board, &mut checked_positions, max_depth, 1) {
        println!("Calculated Positions {} and took {:?}", calculations, now.elapsed());
        println!("Best Move Net Rating {:?}", &best_move.rating);
        send_move(&best_move.from, &best_move.to);
    }
}

fn calculate_move(
    board: &Chessboard, 
    checked_positions: &mut HashSet<String>, 
    max_depth: u8, 
    depth: u8) -> (Option<MoveWithRating>, u64) {

    let moves: Vec<PossibleMove> = get_all_possible_moves(&board, board.get_next_player_figures());
    let mut best_move_rating: i16  = init_best_move(&board.current_move);
    let mut best_move: Option<MoveWithRating> = None;
    let mut calculated_positions: u64 = 0;

    for single in moves.iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to);
        if !check_if_position_should_be_calculated(&new_board, checked_positions){
            break;
        }
        // evaluate - if own check kick
        if depth < max_depth {
            if let (Some(move_evaluation), x) = calculate_move(&new_board, checked_positions, max_depth, depth + 1) {
                calculated_positions+=x;
                if check_if_is_better_move(&board.current_move, best_move_rating, move_evaluation.rating.net_rating){
                    best_move_rating = move_evaluation.rating.net_rating;
                    best_move = Some(MoveWithRating{from: single.from, to: single.to, rating: move_evaluation.rating});
                }
            }
        } else {
            let evaluation = evaluate(&new_board);
            calculated_positions +=1;
            if check_if_is_better_move(&board.current_move, best_move_rating, evaluation.net_rating){
                best_move_rating = evaluation.net_rating;
                best_move = Some(MoveWithRating{from: single.from, to: single.to, rating: evaluation});
            }
        }
    }
    return (best_move, calculated_positions)
}

fn init_best_move(turn: &Color) -> i16 {
    match turn{
        Color::White => -30000,
        Color::Black => 30000
    }
}

fn check_if_is_better_move(turn: &Color, prev: i16, new: i16) -> bool{
    match turn{
        Color::White => new > prev,
        Color::Black => new < prev
    }
}

fn check_if_position_should_be_calculated(board: &Chessboard, calculated_positions: &mut HashSet<String>) -> bool{
    let position_key = board.position_key();
    if calculated_positions.contains(&position_key){
        return false
    }

    let opponent_moves: Vec<usize> = get_all_possible_moves(&board, board.get_opponents(&board.current_move)).iter().map(|x| x.to).collect();
    if let Some((position, _)) = board.get_next_player_figures().iter().find(|entry| entry.1.is_king()){
        // we put ourself in "check" with the move we made 
        if opponent_moves.contains(position){
            return false
        }
    }
    calculated_positions.insert(position_key);
    return true;


}

fn get_all_possible_moves(board: &Chessboard, figures: &HashMap<usize, Figure>) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in figures.iter() {
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
