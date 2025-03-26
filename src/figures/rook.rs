use std::collections::HashMap;

use crate::board::board::Chessboard;
use crate::figures::color::Color;
use crate::helper::moves_by_field::MoveInEveryDirection;

#[derive(Default, Clone)]
pub struct Rook {
    pub color: Color,
    pub has_moved: bool,
}

// the Rooooook is also part of the Queen
pub fn get_rook_moves(
    board: &Chessboard,
    color: &Color,
    position: &usize,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> Vec<usize> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_moves_one_direction(
            &board,
            &color,
            movement.left,
            &position,
            &mut possible_moves,
            calculate_left_field
        );
        get_moves_one_direction(
            &board,
            &color,
            movement.right,
            &position,
            &mut possible_moves,
            calculate_right_field
        );
        get_moves_one_direction(
            &board,
            &color,
            movement.forward,
            &position,
            &mut possible_moves,
            calculate_forward_field
        );
        get_moves_one_direction(
            &board,
            &color,
            movement.back,
            &position,
            &mut possible_moves,
            calculate_backward_field
        );
    }
    possible_moves
}

fn calculate_left_field(rook_position: &usize, movement: usize) -> usize{
    return rook_position - movement;
}

fn calculate_right_field(rook_position: &usize, movement: usize) -> usize{
    return rook_position + movement;
}

fn calculate_forward_field(rook_position: &usize, movement: usize) -> usize{
    return rook_position + (movement*8);
}

fn calculate_backward_field(rook_position: &usize, movement: usize) -> usize{
    return rook_position - (movement*8);
}


fn get_moves_one_direction(
    board: &Chessboard,
    color: &Color,
    direction_moves: usize,
    rook_position: &usize,
    positions: &mut Vec<usize>,
    calculate_position: fn(&usize, usize) -> usize
) {
    for movement in 1_usize..=direction_moves {
        // next field is full
        let field = calculate_position(rook_position, movement);
        if board.positions.get(field) {
            // field is opponent - add it as well!
            if board.get_opponents(color).contains_key(&field) {
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
        get_rook_moves(board, &self.color, own_position, moves_by_field)
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
            color: Color::Black,
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
            color: Color::Black,
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
