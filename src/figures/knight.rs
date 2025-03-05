use crate::board::board::Chessboard;

use super::color::Color;



#[derive(Default)]
pub struct Knight{
    pub color: Color
}

impl Knight {

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();

        let can_move_two_left = own_position % 8 != 1;
        let can_move_two_right = own_position % 8 != 6;
        let can_move_two_backward = own_position >= &16;
        let can_move_two_forward = own_position <= &47;

        let can_move_one_left = board.figure_can_move_left(own_position);
        let can_move_one_right = board.figure_can_move_right(own_position);
        let can_move_one_backward = board.figure_can_move_backward(own_position);
        let can_move_one_forward = board.figure_can_move_forward(own_position);

        if can_move_two_left {
            if can_move_one_backward {
                possible_moves.push(own_position - 10);
            }
            if can_move_one_forward {
                possible_moves.push(own_position + 6);
            }
        }
        if can_move_two_right {
            if can_move_one_backward {
                possible_moves.push(own_position - 6);
            }
            if can_move_one_forward {
                possible_moves.push(own_position + 10);
            }
        }
        if can_move_two_backward {
            if can_move_one_left {
                possible_moves.push(own_position - 17);
            }
            if can_move_one_right {
                possible_moves.push(own_position - 15);
            }
        }
        if can_move_two_forward {
            if can_move_one_left {
                possible_moves.push(own_position + 15);
            }
            if can_move_one_right {
                possible_moves.push(own_position + 17);
            }
        }
        possible_moves
    }
}