use crate::board::board::Chessboard;
use crate::figures::color::Color;

#[derive(Default)]
pub struct Rock {
    pub color: Color,
    pub has_moved: bool,
}

impl Rock {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();

        self.move_forward(board, own_position, &mut possible_moves);
        self.move_backward(board, own_position, &mut possible_moves);
        self.move_left(board, own_position, &mut possible_moves);
        self.move_right(board, own_position, &mut possible_moves);
        possible_moves
    }

    fn move_forward(&self, board: &Chessboard, own_position: &usize, positions: &mut Vec<usize>) {
        if board.figure_can_move_forward(own_position) {
            let next_position: usize = own_position + 8;
            if board.positions.get(next_position) {
                if board
                    .get_opponents(&self.color)
                    .contains_key(&next_position)
                {
                    positions.push(next_position);
                }
            } else {
                positions.push(next_position);
                return self.move_forward(board, &next_position, positions);
            }
        }
    }

    fn move_backward(&self, board: &Chessboard, own_position: &usize, positions: &mut Vec<usize>) {
        if board.figure_can_move_backward(own_position) {
            let next_position: usize = own_position - 8;
            if board.positions.get(next_position) {
                if board
                    .get_opponents(&self.color)
                    .contains_key(&next_position)
                {
                    positions.push(next_position);
                }
            } else {
                positions.push(next_position);
                return self.move_backward(board, &next_position, positions);
            }
        }
    }

    fn move_left(&self, board: &Chessboard, own_position: &usize, positions: &mut Vec<usize>) {
        if board.figure_can_move_left(own_position) {
            let next_position: usize = own_position - 1;
            if board.positions.get(next_position) {
                if board
                    .get_opponents(&self.color)
                    .contains_key(&next_position)
                {
                    positions.push(next_position);
                }
            } else {
                positions.push(next_position);
                return self.move_left(board, &next_position, positions);
            }
        }
    }

    fn move_right(&self, board: &Chessboard, own_position: &usize, positions: &mut Vec<usize>) {
        if board.figure_can_move_right(own_position) {
            let next_position: usize = own_position + 1;
            if board.positions.get(next_position) {
                if board
                    .get_opponents(&self.color)
                    .contains_key(&next_position)
                {
                    positions.push(next_position);
                }
            } else {
                positions.push(next_position);
                return self.move_right(board, &next_position, positions);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bitmaps::Bitmap;

    use super::*;

    #[test]
    fn test_move_forward() {
        let mut positions = Bitmap::<64>::new();

        positions.set(24, true);
        positions.set(1, true);
        let figure = Rock {
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
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
        let figure = Rock {
            color: Color::Black,
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
        };

        let possible_moves = figure.possible_moves(&board, &26);
        assert_eq!(4, possible_moves.len())
    }

    #[test]
    fn test_movement_on_empty_board() {
        let figure = Rock {
            color: Color::Black,
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
        };

        let possible_moves = figure.possible_moves(&board, &11);
        println!("{:?}", possible_moves);
        assert_eq!(14, possible_moves.len())
    }
}
