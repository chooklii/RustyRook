use rustc_hash::FxHashMap;

use crate::board::board::Chessboard;
use crate::helper::moves_by_field::MoveInEveryDirection;

use super::figures::SingleMove;
use super::bishop_and_rook_moves::{get_fields_threatened_by_rook, get_rook_moves, get_takes_rook};

#[derive(Default, Clone)]
pub struct Rook {
    pub has_moved: bool,
}

impl Rook {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn possible_moves(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    ) -> Vec<SingleMove> {
        get_rook_moves(&board, &own_position, &moves_by_field)
    }

    pub fn threatened_fields(    
        &self,
        board: &Chessboard,
        position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
        king_position: &usize
    ) -> Vec<usize>{
        get_fields_threatened_by_rook(&board, &position, &moves_by_field, &king_position)
    }

    pub fn possible_takes(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    ) -> Vec<SingleMove> {
        get_takes_rook(&board, &own_position, &moves_by_field)
    }
}

#[cfg(test)]
mod tests {
    use bitmaps::Bitmap;

    use crate::helper::moves_by_field::get_moves_for_each_field;

    use super::*;

    #[test]
    fn test_move_forward() {
        let mut positions = Bitmap::<64>::new();
        let possible_moves = get_moves_for_each_field();

        positions.set(24, true);
        positions.set(1, true);
        let figure = Rook {
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let possible_moves = figure.possible_moves(&board, &0, &possible_moves);
        assert_eq!(2, possible_moves.len())
    }

    #[test]
    fn test_move_backward() {
        let mut positions = Bitmap::<64>::new();
        let possible_moves = get_moves_for_each_field();

        positions.set(18, true);
        positions.set(25, true);
        positions.set(27, true);
        let figure = Rook {
            ..Default::default()
        };
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let possible_moves = figure.possible_moves(&board, &26, &possible_moves);
        assert_eq!(4, possible_moves.len())
    }

    #[test]
    fn test_movement_on_empty_board() {
        let possible_moves = get_moves_for_each_field();

        let figure = Rook {
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            ..Default::default()
        };

        let possible_moves = figure.possible_moves(&board, &11, &possible_moves);
        assert_eq!(14, possible_moves.len())
    }
}
