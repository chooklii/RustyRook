use rustc_hash::FxHashMap;

use crate::{board::board::Chessboard, helper::moves_by_field::MoveInEveryDirection};

use super::figures::SingleMove;

pub fn get_fields_threatened_by_bishop(
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

pub fn get_fields_threatened_by_rook(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    king_position: &usize
) -> Vec<usize> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_threatened_one_direction(&board,  &movement.left, &mut possible_moves, &king_position);
        get_threatened_one_direction(&board,  &movement.right, &mut possible_moves, &king_position);
        get_threatened_one_direction(&board,  &movement.forward, &mut possible_moves, &king_position);
        get_threatened_one_direction(&board,  &movement.back, &mut possible_moves, &king_position);
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

pub fn get_rook_moves(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> Vec<SingleMove> {
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

pub fn get_takes_bishop(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> Vec<SingleMove> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_takes_one_direction(&board,  &movement.left_forward, &mut possible_moves);
        get_takes_one_direction(&board,  &movement.right_forward, &mut possible_moves);
        get_takes_one_direction(&board,  &movement.left_back, &mut possible_moves);
        get_takes_one_direction(&board,  &movement.right_back, &mut possible_moves);
    }
    possible_moves
}

pub fn get_takes_rook(
    board: &Chessboard,
    position: &usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> Vec<SingleMove> {
    let mut possible_moves = Vec::new();

    if let Some(movement) = moves_by_field.get(position) {
        get_takes_one_direction(&board , &movement.left, &mut possible_moves);
        get_takes_one_direction(&board , &movement.right, &mut possible_moves);
        get_takes_one_direction(&board , &movement.forward, &mut possible_moves);
        get_takes_one_direction(&board, &movement.back, &mut possible_moves);
    }
    possible_moves
}

fn get_takes_one_direction(
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
    }
}