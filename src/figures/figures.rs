use crate::board::board::Chessboard;
use crate::figures::{pawn::Pawn, rock::Rock, Bishop::Bishop, knight::Knight};

pub enum Figure {
    Pawn(Pawn),
    Rock(Rock),
    Bishop(Bishop),
    Knight(Knight)
} 

impl Figure {

    pub fn set_moved(&mut self){
        match self{
            Figure::Pawn(pawn) => pawn.set_moved(),
            Figure::Rock(rock) => rock.set_moved(),
            // bishop, queen and knight dont care
            _ => ()
        }
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize>{
        match self{
            Figure::Pawn(pawn) => pawn.possible_moves(board, own_position),
            Figure::Rock(rock) => rock.possible_moves(board, own_position),
            Figure::Bishop(bishop) => bishop.possible_moves(board, own_position),
            Figure::Knight(knight) => knight.possible_moves(board, own_position)
        }
    }
}
