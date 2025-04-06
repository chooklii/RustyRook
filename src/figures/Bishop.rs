use rustc_hash::FxHashMap;

use crate::board::board::Chessboard;
use crate::helper::moves_by_field::MoveInEveryDirection;

use super::figures::SingleMove;

#[derive(Default, Clone)]
pub struct Bishop {}


pub fn get_threatened_fields_bishop(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    king_position: &usize
) -> Vec<usize> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_threatened_one_direction(&board, &movement.left_forward, &mut possible_moves, &king_position);
        get_threatened_one_direction(&board, &movement.right_forward, &mut possible_moves, &king_position);
        get_threatened_one_direction(&board, &movement.left_back, &mut possible_moves, &king_position);
        get_threatened_one_direction(&board, &movement.right_back, &mut possible_moves, &king_position);
    }
    possible_moves
}

fn get_threatened_one_direction(
    board: &Chessboard,
    direction_moves: &Vec<usize>,
    positions: &mut Vec<usize>,
    king_position: &usize
) {
    for &movement in direction_moves {
        if board.positions.get(movement) && movement != *king_position {
            positions.push(movement);
            return;
        }
        positions.push(movement);
    }
}

// Queen is a Bishop as well - reuse this
pub fn get_bishop_moves(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> Vec<SingleMove> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_moves_one_direction(&board,  &movement.left_forward, &mut possible_moves);
        get_moves_one_direction(&board,  &movement.right_forward, &mut possible_moves);
        get_moves_one_direction(&board,  &movement.left_back, &mut possible_moves);
        get_moves_one_direction(&board,  &movement.right_back, &mut possible_moves);
    }
    possible_moves
}

fn get_moves_one_direction(
    board: &Chessboard,
    direction_moves: &Vec<usize>,
    positions: &mut Vec<SingleMove>,
) {
    for &movement in direction_moves {
        // next field is full
        if board.positions.get(movement) {
            // field is opponent - add it as well!
            if board.get_opponents().contains_key(&movement) {
                positions.push(SingleMove{to: movement, promotion: None})
            }
            return;
        }
        positions.push(SingleMove{to: movement, promotion: None})
    }
}

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
        get_threatened_fields_bishop(&board, &own_position, &moves_by_field, &king_position)
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
