use rustc_hash::FxHashMap;

use crate::{
    board::board::Chessboard,
    helper::moves_by_field::MoveInEveryDirection, MOVES_BY_FIELD,
};

// File Contains Logic to check for possible pinned Figures

// Opposite Ray-Directions
pub fn get_pinned_pieces_and_possible_moves(
    board: &Chessboard,
    king_position: &usize,
) -> FxHashMap<usize, Vec<usize>> {
    let mut pinned_pieces: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    if let Some(moves) = MOVES_BY_FIELD.get(&king_position) {
        // Rook
        check_and_add_pin_one_direction(&board, &moves.left, &mut pinned_pieces, is_rook_movement_figure);
        check_and_add_pin_one_direction(
            &board,
            &moves.right,
            &mut pinned_pieces,
            is_rook_movement_figure,
        );
        check_and_add_pin_one_direction(
            &board,
            &moves.forward,
            &mut pinned_pieces,
            is_rook_movement_figure,
        );
        check_and_add_pin_one_direction(&board, &moves.back, &mut pinned_pieces, is_rook_movement_figure);
        // Bishop
        check_and_add_pin_one_direction(
            &board,
            &moves.left_back,
            &mut pinned_pieces,
            is_bishop_movement_figure,
        );
        check_and_add_pin_one_direction(
            &board,
            &moves.left_forward,
            &mut pinned_pieces,
            is_bishop_movement_figure,
        );
        check_and_add_pin_one_direction(
            &board,
            &moves.right_back,
            &mut pinned_pieces,
            is_bishop_movement_figure,
        );
        check_and_add_pin_one_direction(
            &board,
            &moves.right_forward,
            &mut pinned_pieces,
            is_bishop_movement_figure,
        );
    }
    pinned_pieces
}

fn is_rook_movement_figure(board: &Chessboard,position: usize) -> bool {
    return board.is_queen_or_rook(board.get_opponent_color(), position);
}

fn is_bishop_movement_figure(board: &Chessboard,position: usize) -> bool {
    return board.is_queen_or_bishop(board.get_opponent_color(), position);
}

fn check_and_add_pin_one_direction(
    board: &Chessboard,
    moves: &Vec<usize>,
    pinned_pices: &mut FxHashMap<usize, Vec<usize>>,
    figure_has_correct_movement: fn(&Chessboard, usize) -> bool,
) {
    let mut possible_pinned_piece: Option<usize> = None;
    for &single in moves {
        if board.positions.field_is_used(single) {
            if board.get_opponents().field_is_used(single) {
                if figure_has_correct_movement(&board, single) {
                    if let Some(pinned_piece) = possible_pinned_piece {
                        pinned_pices.insert(pinned_piece, moves.clone());
                    }
                    return;
                }
                // field is opponent but not one that can pin
                return;
            }
            // we do not need to check if there is our figure on the field - is confirmed by the two ifs prior
            // two pieces from player -> no pin for a single one
            if possible_pinned_piece.is_some() {
                return;
            }
            possible_pinned_piece = Some(single);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::helper::moves_by_field::get_moves_for_each_field;

    use super::*;

    #[test]
    fn check_pinned_piece() {
        let mut board = Chessboard {
            ..Default::default()
        };
        // move white queen up on diagonal
        board.move_figure(3, 33, None);
        // move black pawn forward
        board.move_figure(51, 35, None);
        // white rook to pin center
        board.move_figure(0, 20, None);
        // black knight into pin of queen
        board.move_figure(57, 42, None);
        // white dummy move to give the move to black
        board.move_figure(8, 16, None);

        let pinned = get_pinned_pieces_and_possible_moves(&board, &60);
        // e pawn and knight on 42
        assert_eq!(2, pinned.len());
    }

    #[test]
    fn no_pinned_piece() {
        let board = Chessboard {
            ..Default::default()
        };

        let pinned = get_pinned_pieces_and_possible_moves(&board, &4);
        assert_eq!(0, pinned.len());
    }

    #[test]
    fn not_pinned_as_there_in_a_pawn_in_between() {
        let mut board = Chessboard {
            ..Default::default()
        };
        board.move_figure(4, 19, None);
        let pinned = get_pinned_pieces_and_possible_moves(&board, &19);
        assert_eq!(0, pinned.len());
    }
}
