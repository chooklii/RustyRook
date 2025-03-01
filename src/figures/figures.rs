use crate::board::board::Chessboard;

use super::color::Color;

pub trait Figure {
    fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize>;

    fn get_color(&self) -> &Color;

    fn set_moved(&mut self);

    fn calculate_next_position(&self, own_position: &usize, movement: usize) -> usize {
        match self.get_color() {
            Color::Black => return own_position - movement,
            Color::White => return own_position + movement,
        }
    }

}
