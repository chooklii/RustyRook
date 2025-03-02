use crate::board::board::Chessboard;
use crate::figures::color::Color;
use crate::figures::position::calculate_next_position;

#[derive(Default)]
pub struct Pawn {
    pub color: Color,
    pub has_moved: bool,
}

impl Pawn {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn take_left_position(&self, one_step_forward: &usize) -> usize {
        match self.color {
            Color::White => one_step_forward - 1,
            Color::Black => one_step_forward + 1,
        }
    }

    pub fn take_right_position(&self, one_step_forward: &usize) -> usize {
        match self.color {
            Color::White => one_step_forward + 1,
            Color::Black => one_step_forward - 1,
        }
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();
        let one_step_forward = calculate_next_position(&self.color, own_position, 8);

        // en passant is missing

        // if pawn is not able to move one field it cant move anywhere (it is on last row) - can be removed with promotion?
        if !board.figure_can_move_forward(&one_step_forward, &self.color){
            return possible_moves
        }

        if board.figure_can_move_left(own_position, &self.color) {
            if let Some(id) = self.check_taking(board, self.take_left_position(&one_step_forward)) {
                possible_moves.push(id);
            }
        }
        if board.figure_can_move_right(own_position, &self.color) {
            if let Some(id) = self.check_taking(board, self.take_right_position(&one_step_forward))
            {
                possible_moves.push(id);
            }
        }

        // one field forward
        if !board.positions.get(one_step_forward) {
            possible_moves.push(one_step_forward);

            // two fields forward
            let two_steps_forward = calculate_next_position(&self.color, own_position, 16);

            if !self.has_moved
                && !board.positions.get(one_step_forward)
                && !board.positions.get(two_steps_forward)
            {
                possible_moves.push(two_steps_forward);
            }
        }

        possible_moves
    }

    fn check_taking(&self, board: &Chessboard, position: usize) -> Option<usize> {
        if board.positions.get(position) {
            if board.get_opponents(&self.color).contains_key(&position) {
                return Some(position);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bitmaps::Bitmap;

    use crate::figures::figures::Figure;

    use super::*;

    #[test]
    fn test_normal_move() {
        let mut positions = Bitmap::<64>::new();

        positions.set(12, true);

        let figure = Pawn {
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
        };

        let moves = figure.possible_moves(&board, &12);

        assert_eq!(2, moves.len());
    }

    #[test]
    fn test_take_from_a_to_h() {
        let mut positions = Bitmap::<64>::new();
        let mut white_figures: HashMap<usize, Figure> = HashMap::new();

        white_figures.insert(
            23,
            Figure::Pawn(Pawn {
                color: Color::Black,
                has_moved: false,
            }),
        );

        positions.set(16, true);
        positions.set(23, true);
        positions.set(24, true);

        let figure = Pawn {
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            white_figures,
            black_figures: HashMap::new(),
            current_move: Color::White,
        };

        let moves = figure.possible_moves(&board, &16);

        // should not be able to take from Field 16(A3) to 23(H3)
        assert_eq!(0, moves.len());
    }
}
