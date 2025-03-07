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

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        match self {
            Figure::Pawn(pawn) => pawn.possible_moves(board, own_position),
            Figure::Rock(rock) => rock.possible_moves(board, own_position),
            Figure::Bishop(bishop) => bishop.possible_moves(board, own_position),
            Figure::Knight(knight) => knight.possible_moves(board, own_position),
            Figure::Queen(queen) => queen.possible_moves(board, own_position),
            Figure::King(king) => king.possible_moves(board, own_position)
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
}
