use crate::board::board::Chessboard;
use crate::figures::pawn::Pawn;

pub enum Figure {
    Pawn(Pawn)
} 

impl Figure {

    pub fn set_moved(&mut self){
        match self{
            Figure::Pawn(pawn) => pawn.set_moved(),
        }
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize>{
        match self{
            Figure::Pawn(pawn) => pawn.possible_moves(board, own_position)
        }
    }
}
