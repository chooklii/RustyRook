use crate::board::board::Chessboard;
use crate::figures::color::Color;
use crate::helper::movement::{figure_can_move_backward, figure_can_move_forward, figure_can_move_left, figure_can_move_right};

#[derive(Default, Clone)]
pub struct Rook {
    pub color: Color,
    pub has_moved: bool,
}

// the Rooooook is also part of the Queen
pub fn get_rook_moves(board: &Chessboard, color: &Color,  position: &usize) -> Vec<usize> {
    let mut possible_moves = Vec::new();

    move_forward(board,color, position, &mut possible_moves);
    move_backward(board, color,position, &mut possible_moves);
    move_left(board, color,position, &mut possible_moves);
    move_right(board, color,position, &mut possible_moves);
    possible_moves
}

fn move_forward(board: &Chessboard, color: &Color, own_position: &usize, positions: &mut Vec<usize>) {
    if figure_can_move_forward(own_position) {
        let next_position: usize = own_position + 8;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                positions.push(next_position);
            }
        } else {
            positions.push(next_position);
            move_forward(board, color, &next_position, positions);
        }
    }
}

fn move_backward(board: &Chessboard,color: &Color, own_position: &usize, positions: &mut Vec<usize>) {
    if figure_can_move_backward(own_position) {
        let next_position: usize = own_position - 8;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                positions.push(next_position);
            }
        } else {
            positions.push(next_position);
            move_backward(board, color, &next_position, positions);
        }
    }
}

fn move_left(board: &Chessboard,color: &Color, own_position: &usize, positions: &mut Vec<usize>) {
    if figure_can_move_left(own_position) {
        let next_position: usize = own_position - 1;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                positions.push(next_position);
            }
        } else {
            positions.push(next_position);
            move_left(board, color, &next_position, positions);
        }
    }
}

fn move_right(board: &Chessboard,color: &Color, own_position: &usize, positions: &mut Vec<usize>) {
    if figure_can_move_right(own_position) {
        let next_position: usize = own_position + 1;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                positions.push(next_position);
            }
        } else {
            positions.push(next_position);
            move_right(board, color, &next_position, positions);
        }
    }
}

impl Rook {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        get_rook_moves(board, &self.color, own_position)
    }

}

#[cfg(test)]
mod tests {
    use bitmaps::Bitmap;

    use super::*;

    #[test]
    fn test_move_forward() {
        let mut positions = Bitmap::<64>::new();

        positions.set(24, true);
        positions.set(1, true);
        let figure = Rook {
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let possible_moves = figure.possible_moves(&board, &0);
        assert_eq!(2, possible_moves.len())
    }

    #[test]
    fn test_move_backward() {
        let mut positions = Bitmap::<64>::new();

        positions.set(18, true);
        positions.set(25, true);
        positions.set(27, true);
        let figure = Rook {
            color: Color::Black,
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let possible_moves = figure.possible_moves(&board, &26);
        assert_eq!(4, possible_moves.len())
    }

    #[test]
    fn test_movement_on_empty_board() {
        let figure = Rook {
            color: Color::Black,
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            ..Default::default()
        };

        let possible_moves = figure.possible_moves(&board, &11);
        assert_eq!(14, possible_moves.len())
    }
}
