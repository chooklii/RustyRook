use crate::board::board::Chessboard;

use super::color::Color;

#[derive(Default)]
pub struct Knight {
    pub color: Color
}

impl Knight {
    fn add_as_move(&self, board: &Chessboard, position: usize, possible_moves: &mut Vec<usize>) {
        if board.positions.get(position) {
            if board.get_opponents(&self.color).contains_key(&position) {
                possible_moves.push(position);
            }
        } else {
            possible_moves.push(position);
        }
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();

        let can_move_one_left = board.figure_can_move_left(own_position);
        let can_move_one_right = board.figure_can_move_right(own_position);
        let can_move_one_backward = board.figure_can_move_backward(own_position);
        let can_move_one_forward = board.figure_can_move_forward(own_position);

        let can_move_two_left = can_move_one_left && own_position % 8 != 1;
        let can_move_two_right = can_move_one_right && own_position % 8 != 6;
        let can_move_two_backward = can_move_one_backward && own_position >= &16;
        let can_move_two_forward = can_move_one_forward && own_position <= &47;

        if can_move_two_left {
            if can_move_one_backward {
                self.add_as_move(board, own_position - 10, &mut possible_moves);
            }
            if can_move_one_forward {
                self.add_as_move(board, own_position +6, &mut possible_moves);
            }
        }
        if can_move_two_right {
            if can_move_one_backward {
                self.add_as_move(board, own_position - 6, &mut possible_moves);
            }
            if can_move_one_forward {
                self.add_as_move(board, own_position + 10, &mut possible_moves);
            }
        }
        if can_move_two_backward {
            if can_move_one_left {
                self.add_as_move(board, own_position - 17, &mut possible_moves);
            }
            if can_move_one_right {
                self.add_as_move(board, own_position - 15, &mut possible_moves);
            }
        }
        if can_move_two_forward {
            if can_move_one_left {
                self.add_as_move(board, own_position + 15, &mut possible_moves);
            }
            if can_move_one_right {
                self.add_as_move(board, own_position +17, &mut possible_moves);
            }
        }
        possible_moves
    }
}


#[cfg(test)]
mod tests{
    use std::collections::HashMap;

    use bitmaps::Bitmap;

    use super::*;

    #[test]
    fn test_empty_board(){
        let figure = Knight {
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
        };

        let moves = figure.possible_moves(&board, &27);
        assert_eq!(8, moves.len());

        let moves = figure.possible_moves(&board, &0);
        assert_eq!(2, moves.len());

        let moves = figure.possible_moves(&board, &54);
        assert_eq!(4, moves.len());

    }
}