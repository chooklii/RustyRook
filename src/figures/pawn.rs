use std::collections::HashMap;

use crate::board::board::Chessboard;
use crate::figures::color::Color;
use crate::helper::moves_by_field::MoveInEveryDirection;

#[derive(Default, Clone)]
pub struct Pawn {
    pub color: Color,
    pub has_moved: bool,
}

impl Pawn {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    fn take_left_position(&self, one_step_forward: &usize) -> usize {
        match self.color {
            Color::White => one_step_forward - 1,
            Color::Black => one_step_forward + 1,
        }
    }

    fn take_right_position(&self, one_step_forward: &usize) -> usize {
        match self.color {
            Color::White => one_step_forward + 1,
            Color::Black => one_step_forward - 1,
        }
    }

    fn en_passant_position_left(&self, own_position: &usize) -> usize {
        match self.color {
            Color::White => own_position - 1,
            Color::Black => own_position + 1,
        }
    }

    fn en_passant_position_right(&self, own_position: &usize) -> usize {
        match self.color {
            Color::White => own_position + 1,
            Color::Black => own_position - 1,
        }
    }

    fn figure_can_move_left(&self, field: &usize, color: &Color) -> bool {
        match color {
            Color::White => field % 8 != 0,
            Color::Black => field % 8 != 7,
        }
    }

    fn figure_can_move_right(&self, field: &usize, color: &Color) -> bool {
        match color {
            Color::White => field % 8 != 7,
            Color::Black => field % 8 != 0,
        }
    }

    fn figure_can_move_forward(&self, field: &usize, color: &Color) -> bool {
        match color {
            Color::White => field <= &55,
            Color::Black => field >= &8,
        }
    }

    fn calculate_forward_position(&self, own_position: &usize, movement: usize) -> usize {
        match self.color {
            Color::Black => return own_position - movement,
            Color::White => return own_position + movement,
        }
    }

    fn check_taking(&self, board: &Chessboard, position: usize) -> Option<usize> {
        if board.positions.get(position) {
            if board.get_opponents().contains_key(&position) {
                return Some(position);
            }
        }
        None
    }

    // check if en passant would put our king into check (not captures by pinned peaces, as there are two between R/Q and K)
    fn en_passant_no_check(
        &self,
        board: &Chessboard,
        own_position: &usize,
        en_passanted: &usize,
        moves_by_field: &HashMap<usize, MoveInEveryDirection>,
    ) -> bool {
        if let Some((king_position, _)) = board
            .get_next_player_figures()
            .iter()
            .find(|(_, fig)| fig.is_king())
        {
            if let Some(moves) = moves_by_field.get(king_position) {
                if moves.left.contains(own_position) && moves.left.contains(en_passanted) {
                    return self.check_if_other_figure_in_between(
                        &board,
                        &moves.left,
                        &en_passanted,
                        &own_position
                    );
                }
                if moves.right.contains(own_position) && moves.right.contains(en_passanted) {
                    return self.check_if_other_figure_in_between(
                        board,
                        &moves.right,
                        &en_passanted,
                        &own_position
                    );
                }
            }
        }
        true
    }

    fn check_if_other_figure_in_between(
        &self,
        board: &Chessboard,
        moves: &Vec<usize>,
        en_passanted: &usize,
        own_position: &usize
    ) -> bool {
        for single in moves {
            // ignore both pawns involved in en passant
            if single != en_passanted && single != own_position {
                if board.positions.get(*single) {
                    if let Some(opponent) = board.get_opponents().get(single) {
                        return !opponent.is_queen() && !opponent.is_rook();
                    }
                    return false;
                }
            }
        }
        return true;
    }

    pub fn possible_moves(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &HashMap<usize, MoveInEveryDirection>,
    ) -> Vec<usize> {
        let mut possible_moves = Vec::new();
        // if pawn is not able to move one field it cant move anywhere (it is on last row) - can be removed with promotion?
        if !self.figure_can_move_forward(&own_position, &self.color) {
            return possible_moves;
        }

        let one_step_forward = self.calculate_forward_position(own_position, 8);

        if self.figure_can_move_left(own_position, &self.color) {
            let take_left_position = self.take_left_position(&one_step_forward);
            // regular take left
            if let Some(id) = self.check_taking(board, take_left_position) {
                possible_moves.push(id);
            }
            // en passant left
            else if let Some(possible_en_passant) = board.en_passant {
                if self.en_passant_no_check(
                    &board,
                    &own_position,
                    &possible_en_passant,
                    &moves_by_field,
                ) {
                    if self.en_passant_position_left(own_position) == possible_en_passant {
                        possible_moves.push(take_left_position)
                    }
                }
            }
        }
        if self.figure_can_move_right(own_position, &self.color) {
            let take_right_position = self.take_right_position(&one_step_forward);
            // regular take right
            if let Some(id) = self.check_taking(board, take_right_position) {
                possible_moves.push(id);
            }
            // en passant right
            else if let Some(possible_en_passant) = board.en_passant {
                if self.en_passant_no_check(
                    &board,
                    &own_position,
                    &possible_en_passant,
                    &moves_by_field,
                ) {
                    if self.en_passant_position_right(own_position) == possible_en_passant {
                        possible_moves.push(take_right_position)
                    }
                }
            }
        }
        // one field forward
        if !board.positions.get(one_step_forward) {
            possible_moves.push(one_step_forward);

            // two fields forward
            if !self.has_moved {
                let two_steps_forward = self.calculate_forward_position(own_position, 16);

                if !board.positions.get(one_step_forward) && !board.positions.get(two_steps_forward)
                {
                    possible_moves.push(two_steps_forward);
                }
            }
        }
        possible_moves
    }

    pub fn threatened_fields(&self, own_position: &usize) -> Vec<usize> {
        let mut possible_moves = Vec::new();

        // if pawn is not able to move one field it cant move anywhere (it is on last row) - can be removed with promotion?
        if !self.figure_can_move_forward(&own_position, &self.color) {
            return possible_moves;
        }
        let one_step_forward = self.calculate_forward_position(own_position, 8);
        if self.figure_can_move_left(own_position, &self.color) {
            // take left
            possible_moves.push(self.take_left_position(&one_step_forward));
        }
        if self.figure_can_move_right(own_position, &self.color) {
            // regular take right
            possible_moves.push(self.take_right_position(&one_step_forward));
        }
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bitmaps::Bitmap;

    use crate::{figures::{figures::Figure, king::King, rook::Rook}, helper::moves_by_field::{self, get_moves_for_each_field}};

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
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &12, &HashMap::new());

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
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &16, &HashMap::new());

        // should not be able to take from Field 16(A3) to 23(H3)
        assert_eq!(0, moves.len());
    }

    // Black H Pawn is not working - test driven fix
    #[test]
    fn test_black_h_pawn_with_free_path() {
        let mut board = Chessboard {
            current_move: Color::Black,
            ..Default::default()
        };
        board.set_to_default();

        let figure = Pawn {
            color: Color::Black,
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &55, &HashMap::new());

        assert_eq!(2, moves.len());
    }

    #[test]
    fn test_en_passant_left() {
        let mut positions = Bitmap::<64>::new();
        let moves_by_field = get_moves_for_each_field();

        positions.set(35, true);

        let figure = Pawn {
            ..Default::default()
        };

        let mut opponents: HashMap<usize, Figure> = HashMap::new();
        opponents.insert(
            34,
            Figure::Pawn(Pawn {
                ..Default::default()
            }),
        );

        let board = Chessboard {
            positions,
            black_figures: opponents,
            en_passant: Some(34),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &35, &moves_by_field);
        assert_eq!(true, moves.contains(&42));
    }

    #[test]
    fn test_en_passant_right() {
        let mut positions = Bitmap::<64>::new();
        let moves_by_field = get_moves_for_each_field();
        positions.set(26, true);

        let figure = Pawn {
            color: Color::Black,
            ..Default::default()
        };

        let mut opponents: HashMap<usize, Figure> = HashMap::new();
        opponents.insert(
            27,
            Figure::Pawn(Pawn {
                ..Default::default()
            }),
        );

        let board = Chessboard {
            positions,
            white_figures: opponents,
            current_move: Color::Black,
            en_passant: Some(27),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &26, &moves_by_field);
        assert_eq!(true, moves.contains(&19));
    }

    #[test]
    fn test_en_passant_right_not_possible_as_it_would_put_us_in_check() {
        let mut positions = Bitmap::<64>::new();
        let moves_by_field = get_moves_for_each_field();

        positions.set(26, true);
        positions.set(27, true);
        positions.set(31, true);

        let figure = Pawn {
            color: Color::Black,
            ..Default::default()
        };

        let mut opponents: HashMap<usize, Figure> = HashMap::new();
        opponents.insert(
            27,
            Figure::Pawn(Pawn {
                ..Default::default()
            }),
        );
        opponents.insert(
            31,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );

        let mut own_figures: HashMap<usize, Figure> = HashMap::new();
        own_figures.insert(
            24,
            Figure::King(King {
                ..Default::default()
            }),
        );

        let board = Chessboard {
            positions,
            white_figures: opponents,
            black_figures: own_figures,
            current_move: Color::Black,
            en_passant: Some(27),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &26, &moves_by_field);
        println!("Moves {:?}", moves);
        assert_eq!(false, moves.contains(&19));
    }
}
