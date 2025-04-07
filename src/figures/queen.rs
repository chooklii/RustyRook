use rustc_hash::FxHashMap;
use crate::{board::board::Chessboard, helper::moves_by_field::MoveInEveryDirection};

use super::figures::SingleMove;
use super::BishopAndRookMoves::{get_bishop_moves, get_fields_threatened_by_bishop, get_fields_threatened_by_rook, get_rook_moves, get_takes_bishop, get_takes_rook};

#[derive(Default, Clone)]
pub struct Queen {}

impl Queen {
    pub fn possible_moves(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    ) -> Vec<SingleMove> {
        // Queen is mix of Rook and Bishop
        let mut bishop = get_bishop_moves(board, &own_position, &moves_by_field);
        let mut rook = get_rook_moves(board, &own_position, &moves_by_field);
        bishop.append(&mut rook);

        bishop
    }

    pub fn threatened_fields(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
        king_position: &usize
    ) -> Vec<usize> {
        // Queen is mix of Rook and Bishop
        let mut bishop = get_fields_threatened_by_bishop(board, &own_position, &moves_by_field, &king_position);
        let mut rook = get_fields_threatened_by_rook(board, &own_position, &moves_by_field, &king_position);
        bishop.append(&mut rook);

        bishop
    }

    pub fn possible_takes(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    ) -> Vec<SingleMove> {
        // Queen is mix of Rook and Bishop
        let mut bishop = get_takes_bishop(board, &own_position, &moves_by_field);
        let mut rook = get_takes_rook(board, &own_position, &moves_by_field);
        bishop.append(&mut rook);

        bishop
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

        let figure = Queen {
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &0, &possible_moves);
        assert_eq!(21, moves.len());

        let moves = figure.possible_moves(&board, &19, &possible_moves);
        assert_eq!(25, moves.len());
    }
}
