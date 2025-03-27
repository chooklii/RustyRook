use std::collections::HashMap;

use crate::board::board::Chessboard;
use crate::figures::color::Color;
use crate::helper::moves_by_field::MoveInEveryDirection;

#[derive(Default, Clone)]
pub struct Bishop {
    pub color: Color,
}

// Queen is a Bishop as well - reuse this
pub fn get_bishop_moves(board: &Chessboard,color: &Color, position: &usize, moves_by_field: &HashMap<usize, MoveInEveryDirection>) -> Vec<usize>{
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_moves_one_direction(
            &board,
            &color,
            &movement.left_forward,
            &mut possible_moves,
        );
        get_moves_one_direction(
            &board,
            &color,
            &movement.right_forward,
            &mut possible_moves,
        );
        get_moves_one_direction(
            &board,
            &color,
            &movement.left_back,
            &mut possible_moves,
        );
        get_moves_one_direction(
            &board,
            &color,
            &movement.right_back,
            &mut possible_moves,
        );
    }
    possible_moves
}


fn get_moves_one_direction(
    board: &Chessboard,
    color: &Color,
    direction_moves: &Vec<usize>,
    positions: &mut Vec<usize>,
) {
    for movement in direction_moves {
        // next field is full
        if board.positions.get(*movement) {
            // field is opponent - add it as well!
            if board.get_opponents(color).contains_key(movement) {
                positions.push(*movement)
            }
            return;
        }
        positions.push(*movement);
    }
}

impl Bishop {
    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize, moves_by_field: &HashMap<usize, MoveInEveryDirection>) -> Vec<usize> {
        get_bishop_moves(board, &self.color, &own_position, &moves_by_field)
    }
}

#[cfg(test)]
mod tests {
    use bitmaps::Bitmap;

    use crate::helper::moves_by_field::get_moves_for_each_field;

    use super::*;


    #[test]
    fn move_empty_board(){
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
    fn not_able_to_move(){
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
    fn able_to_move_in_two_directions(){
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