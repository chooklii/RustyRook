use rustc_hash::FxHashMap;

use crate::board::board::Chessboard;
use crate::helper::moves_by_field::MoveInEveryDirection;

use super::figures::SingleMove;
use super::BishopAndRookMoves::{get_bishop_moves, get_fields_threatened_by_bishop, get_takes_bishop};

#[derive(Default, Clone)]
pub struct Bishop {}

impl Bishop {
    pub fn possible_moves(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    ) -> Vec<SingleMove> {
        get_bishop_moves(board, &own_position, &moves_by_field)
    }

    pub fn threatened_fields(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
        king_position: &usize
    ) -> Vec<usize> {
        get_fields_threatened_by_bishop(&board, &own_position, &moves_by_field, &king_position)
    }

    pub fn possible_takes(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    ) -> Vec<SingleMove> {
        get_takes_bishop(&board, &own_position, &moves_by_field)
    }
}

#[cfg(test)]
mod tests {
    use bitmaps::Bitmap;

    use crate::helper::moves_by_field::get_moves_for_each_field;

    use super::*;

    #[test]
    fn move_empty_board() {
        let possible_moves = get_moves_for_each_field();
        let figure = Bishop {
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &27, &possible_moves);
        assert_eq!(13, moves.len());

        let moves = figure.possible_moves(&board, &0, &possible_moves);
        assert_eq!(7, moves.len());
    }

    #[test]
    fn not_able_to_move() {
        let possible_moves = get_moves_for_each_field();

        let figure = Bishop {
            ..Default::default()
        };
        let mut positions = Bitmap::<64>::new();
        positions.set(9, true);
        positions.set(11, true);
        positions.set(27, true);
        positions.set(25, true);
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &18, &possible_moves);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn able_to_move_in_two_directions() {
        let possible_moves = get_moves_for_each_field();
        let figure = Bishop {
            ..Default::default()
        };
        let mut positions = Bitmap::<64>::new();
        positions.set(29, true);
        positions.set(13, true);
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &20, &possible_moves);
        assert_eq!(6, moves.len());
    }
}
