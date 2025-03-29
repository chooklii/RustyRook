use std::collections::HashMap;
use std::hash::Hash;

use crate::board::board::Chessboard;
use crate::figures::{knight::Knight, pawn::Pawn, queen::Queen, rook::Rook, bishop::Bishop, king::King};
use crate::helper::moves_by_field::{self, MoveInEveryDirection};

#[derive(Clone)]
pub enum Figure {
    Pawn(Pawn),
    Rook(Rook),
    Bishop(Bishop),
    Knight(Knight),
    Queen(Queen),
    King(King)
}

impl Figure {
    pub fn set_moved(&mut self) {
        match self {
            Figure::Pawn(pawn) => pawn.set_moved(),
            Figure::Rook(rook) => rook.set_moved(),
            Figure::King(king) => king.set_moved(),
            // bishop, queen and knight dont care
            _ => (),
        }
    }

    pub fn has_moved(&self) -> bool{
        match self {
            Figure::Rook(rook) => rook.has_moved,
            Figure::King(king) => king.has_moved,
            Figure::Pawn(pawn) => pawn.has_moved,
            _ => false
        }   
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize, opponent_moves: &Vec<usize>, moves_by_field: &HashMap<usize, MoveInEveryDirection>) -> Vec<usize> {
        match self {
            Figure::Pawn(pawn) => pawn.possible_moves(board, own_position, &moves_by_field),
            Figure::Rook(rook) => rook.possible_moves(board, own_position, &moves_by_field),
            Figure::Bishop(bishop) => bishop.possible_moves(board, own_position, &moves_by_field),
            Figure::Knight(knight) => knight.possible_moves(board, own_position, &moves_by_field),
            Figure::Queen(queen) => queen.possible_moves(board, own_position, &moves_by_field),
            Figure::King(king) => king.possible_moves(board, own_position, &opponent_moves)
        }
    }

    pub fn threatened_fields(&self, board: &Chessboard, own_position: &usize, moves_by_field: &HashMap<usize, MoveInEveryDirection>) -> Vec<usize> {
        match self {
            Figure::Pawn(pawn) => pawn.threatened_fields(&own_position),
            Figure::Rook(rook) => rook.threatened_fields(board, &own_position, &moves_by_field),
            Figure::Bishop(bishop) => bishop.threatened_fields(board, &own_position, &moves_by_field),
            Figure::Knight(knight) => knight.threatened_fields(&own_position, &moves_by_field),
            Figure::Queen(queen) => queen.threatened_fields(board, &own_position, &moves_by_field),
            Figure::King(king) => king.threatened_fields(&own_position)
        }
    } 

    pub fn get_weight(&self) -> u8{
        match self{
            Figure::Bishop(_) => 3,
            Figure::King(_) => 100,
            Figure::Queen(_) => 9,
            Figure::Knight(_) => 3,
            Figure::Pawn(_) => 1,
            Figure::Rook(_) => 5
        }
    }

    pub fn get_name(&self) -> String{
        match self{
            Figure::Bishop(_) => String::from("B"),
            Figure::King(_) => String::from("K"),
            Figure::Queen(_) => String::from("Q"),
            Figure::Knight(_) => String::from("H"), // Horse
            Figure::Pawn(_) => String::from("P"),
            Figure::Rook(_) => String::from("R")
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
            Figure::Rook(_) => true,
            _ => false
        }
    }

    pub fn is_queen(&self) -> bool{
        match self{
            Figure::Queen(_) => true,
            _ => false
        }
    }

    pub fn is_bishop(&self) -> bool{
        match self{
            Figure::Bishop(_) => true,
            _ => false
        }
    }

    pub fn is_pawn(&self) -> bool{
        match self{
            Figure::Pawn(_) => true,
            _ => false
        }
    }

    pub fn is_knight(&self) -> bool{
        match self{
            Figure::Knight(_) => true,
            _ => false
        }
    }
}
