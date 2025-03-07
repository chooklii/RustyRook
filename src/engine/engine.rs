use crate::{board::board::Chessboard, evaluation::{evaluate, Evaluation}};

use super::sender::send_move;

struct PossibleMove {
    from: usize,
    to: usize,
}

pub fn search_for_best_move(board: &Chessboard){
    let max_depth:u8 = 3;
    calculate_move(board, 1);
    evaluate(board);
}

pub fn calculate_move(board: &Chessboard, depth: u8) {
    let moves: Vec<PossibleMove> = get_all_possible_moves(&board);       
    let mut ratings_for_each_move = Vec::new();
    for single in moves.iter(){
        let mut board = board.clone();
        board.move_figure(single.from, single.to);
        ratings_for_each_move.push((single, evaluate(&board)));
        if depth < 3 {
            calculate_move(&board, depth + 1);
        }
    }


}
    




fn get_all_possible_moves(board: &Chessboard) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in board.get_next_player_figures().iter() {
        val.possible_moves(board, &key).iter().for_each(|single_move| {
            moves.push(PossibleMove {
                from: key.clone(),
                to: single_move.clone(),
            })
        });
    }
    moves
}
