use crate::board::board::Chessboard;

use super::{color::Color, rock::get_rook_moves, bishop::get_bishop_moves};



#[derive(Default, Clone)]
pub struct Queen{
    pub color: Color
}

impl Queen{

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize>{
        // Queen is mix of Rook and Bishop
        let mut bishop =  get_bishop_moves(board, &self.color, own_position);
        let mut rook = get_rook_moves(board, &self.color, own_position);
        bishop.append(&mut rook);

        bishop
    }
}

#[cfg(test)]
mod tests{

    use std::collections::HashMap;

    use bitmaps::Bitmap;

    use super::*;

    #[test]
    fn move_empty_board(){
        let figure = Queen {
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
        };

        let moves = figure.possible_moves(&board, &0);
        assert_eq!(21, moves.len());

        let moves = figure.possible_moves(&board, &19);
        assert_eq!(25, moves.len());

    }
}