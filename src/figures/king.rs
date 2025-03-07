use crate::board::board::Chessboard;

use super::color::Color;

#[derive(Default)]
pub struct King {
    pub color: Color,
    pub has_moved: bool,
}

impl King {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    fn check_move(
        &self,
        board: &Chessboard,
        next_position: usize,
        possible_moves: &mut Vec<usize>,
    ) {
        if board.positions.get(next_position) {
            if board
                .get_opponents(&self.color)
                .contains_key(&next_position)
            {
                possible_moves.push(next_position);
            }
        } else {
            possible_moves.push(next_position);
        }
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();

        let can_move_backward = board.figure_can_move_backward(own_position);
        let can_move_left = board.figure_can_move_left(own_position);
        let can_move_right = board.figure_can_move_right(own_position);
        let can_move_forward = board.figure_can_move_forward(own_position);

        if can_move_backward {
            self.check_move(board, own_position - 8, &mut possible_moves);
            if can_move_left {
                self.check_move(board, own_position - 9, &mut possible_moves);
            }
            if can_move_right {
                self.check_move(board, own_position - 7, &mut possible_moves);
            }
        }
        if can_move_forward {
            self.check_move(board, own_position + 8, &mut possible_moves);
            if can_move_left {
                self.check_move(board, own_position + 7, &mut possible_moves);
            }
            if can_move_right {
                self.check_move(board, own_position + 9, &mut possible_moves);
            }
        }
        if can_move_left {
            self.check_move(board, own_position - 1, &mut possible_moves);
        }
        if can_move_right {
            self.check_move(board, own_position + 1, &mut possible_moves);
        }

        // castle missing

        possible_moves
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use bitmaps::Bitmap;
    use super::*;


    #[test]
    fn move_empty_board(){
        let figure = King {
            color: Color::Black,
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
        };

        let moves = figure.possible_moves(&board, &10);
        assert_eq!(8, moves.len());

        let moves = figure.possible_moves(&board, &0);
        assert_eq!(3, moves.len());

        let moves = figure.possible_moves(&board, &31);
        assert_eq!(5, moves.len());

    }
}