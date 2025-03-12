use crate::board::board::Chessboard;
use crate::figures::{knight::Knight, pawn::Pawn, queen::Queen, rock::Rock, Bishop::Bishop, king::King};

use super::knight;

#[derive(Clone)]
pub enum Figure {
    Pawn(Pawn),
    Rock(Rock),
    Bishop(Bishop),
    Knight(Knight),
    Queen(Queen),
    King(King)
}

impl Figure {
    pub fn set_moved(&mut self) {
        match self {
            Figure::Pawn(pawn) => pawn.set_moved(),
            Figure::Rock(rock) => rock.set_moved(),
            Figure::King(king) => king.set_moved(),
            // bishop, queen and knight dont care
            _ => (),
        }
    }

    pub fn has_moved(&self) -> bool{
        match self {
            Figure::Rock(rock) => rock.has_moved,
            Figure::King(king) => king.has_moved,
            Figure::Pawn(pawn) => pawn.has_moved,
            _ => false
        }   
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize, opponent_moves: &Vec<usize>) -> Vec<usize> {
        match self {
            Figure::Pawn(pawn) => pawn.possible_moves(board, own_position),
            Figure::Rock(rock) => rock.possible_moves(board, own_position),
            Figure::Bishop(bishop) => bishop.possible_moves(board, own_position),
            Figure::Knight(knight) => knight.possible_moves(board, own_position),
            Figure::Queen(queen) => queen.possible_moves(board, own_position),
            Figure::King(king) => king.possible_moves(board, own_position, &opponent_moves)
        }
    }

    pub fn get_weight(&self) -> u8{
        match self{
            Figure::Bishop(_) => 3,
            Figure::King(_) => 100,
            Figure::Queen(_) => 9,
            Figure::Knight(_) => 3,
            Figure::Pawn(_) => 1,
            Figure::Rock(_) => 5
        }
    }

    pub fn get_name(&self) -> String{
        match self{
            Figure::Bishop(_) => String::from("B"),
            Figure::King(_) => String::from("K"),
            Figure::Queen(_) => String::from("Q"),
            Figure::Knight(_) => String::from("H"), // Horse
            Figure::Pawn(_) => String::from("P"),
            Figure::Rock(_) => String::from("R")
        }
    }

    // check for checks and castle
    pub fn is_king(&self) -> bool{
        match self{
            Figure::King(_) => true,
            _ => false
        }
    }

    pub fn is_rook(&self) -> bool{
        match self{
            Figure::Rock(_) => true,
            _ => false
        }
    }
}
