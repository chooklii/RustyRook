use std::collections::HashMap;

use crate::board::board::Chessboard;
use crate::helper::moves_by_field::MoveInEveryDirection;

#[derive(Default, Clone)]
pub struct Rook {
    pub has_moved: bool,
}

pub fn get_rook_threatened_fields(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> Vec<usize> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_threatened_one_direction(&board,  &movement.left, &mut possible_moves);
        get_threatened_one_direction(&board,  &movement.right, &mut possible_moves);
        get_threatened_one_direction(&board,  &movement.forward, &mut possible_moves);
        get_threatened_one_direction(&board,  &movement.back, &mut possible_moves);
    }
    possible_moves
}

fn get_threatened_one_direction(
    board: &Chessboard,
    direction_moves: &Vec<usize>,
    positions: &mut Vec<usize>,
) {
    for &field in direction_moves {
        if board.positions.get(field) {
            positions.push(field);
            return;
        }
        positions.push(field);
    }
}

// the Rooooook is also part of the Queen
pub fn get_rook_moves(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> Vec<usize> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_moves_one_direction(&board , &movement.left, &mut possible_moves);
        get_moves_one_direction(&board , &movement.right, &mut possible_moves);
        get_moves_one_direction(&board , &movement.forward, &mut possible_moves);
        get_moves_one_direction(&board, &movement.back, &mut possible_moves);
    }
    possible_moves
}

fn get_moves_one_direction(
    board: &Chessboard,
    direction_moves: &Vec<usize>,
    positions: &mut Vec<usize>,
) {
    for &field in direction_moves {
        if board.positions.get(field) {
            // field is opponent - add it as well!
            if board.get_opponents().contains_key(&field) {
                positions.push(field)
            }
            return;
        }
        positions.push(field);
    }
}

impl Rook {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    pub fn possible_moves(
        &self,
        board: &Chessboard,
        own_position: &usize,
        moves_by_field: &HashMap<usize, MoveInEveryDirection>,
    ) -> Vec<usize> {
        get_rook_moves(board, own_position, moves_by_field)
    }

    pub fn threatened_fields(    
        &self,
        board: &Chessboard,
        position: &usize,
        moves_by_field: &HashMap<usize, MoveInEveryDirection>
    ) -> Vec<usize>{
        get_rook_threatened_fields(board, position, moves_by_field)
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
