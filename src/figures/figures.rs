use crate::board::board::Chessboard;
use crate::figures::blablabla::Pawn;

use super::color::Color;

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

    pub fn get_color(&self) -> &Color{
        match self{
            Figure::Pawn(pawn) => pawn.get_color()
        }
    }
}
