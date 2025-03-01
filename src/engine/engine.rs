
use crate::board::{board::Chessboard};


pub fn calculate_move(board: &Chessboard ){
    get_all_possible_moves(&board);
}

fn get_all_possible_moves(board: &Chessboard){
    for (key, val) in board.figures.iter() { 
        println!("{}{:?}", key, val.possible_moves(board, &key));
    }
}