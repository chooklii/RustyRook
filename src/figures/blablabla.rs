use crate::board::board::Chessboard;
use crate::figures::position::calculate_next_position;
use crate::figures::color::Color;

#[derive(Default)]
pub struct Pawn {
    pub color: Color,
    pub has_moved: bool,
}

impl Pawn {
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();
        let one_step_forward = calculate_next_position(&self.color, own_position, 8);

        // all rows, but a row are able to take on the left
        if !board.is_in_a_row(own_position) {
            if let Some(id) = self.check_taking(board, one_step_forward - 1) {
                possible_moves.push(id);
            }
        }
        // all rows, but h row are able to take on the right
        if !board.is_in_h_row(own_position) {
            if let Some(id) = self.check_taking(board, one_step_forward + 1) {
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
            let field = board.figures.get(&position);
            let field_color = field.unwrap().get_color();
            if field_color != self.get_color() {
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
            figures: HashMap::new(),
        };

        let moves = figure.possible_moves(&board, &12);

        assert_eq!(2, moves.len());
    }

    #[test]
    fn test_take_from_a_to_h() {
        let mut positions = Bitmap::<64>::new();
        let mut figures: HashMap<usize, Figure> = HashMap::new();

        figures.insert(
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
        let board = Chessboard { positions, figures };

        let moves = figure.possible_moves(&board, &16);

        // should not be able to take from Field 16(A3) to 23(H3)
        assert_eq!(0, moves.len());
    }
}
