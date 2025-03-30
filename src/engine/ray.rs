use std::collections::HashMap;

use crate::{
    board::board::Chessboard, figures::figures::Figure,
    helper::moves_by_field::MoveInEveryDirection,
};

// File Contains Logic to check for possible pinned Figures

// Opposite Ray-Directions
pub fn get_pinned_pieces_and_possible_moves(
    board: &Chessboard,
    king_position: &usize,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> HashMap<usize, Vec<usize>> {
    let mut pinned_pieces: HashMap<usize, Vec<usize>> = HashMap::new();

    if let Some(moves) = moves_by_field.get(&king_position) {
        // Rook
        check_and_add_pin_one_direction(&board, &moves.left, &mut pinned_pieces, has_rook_movement);
        check_and_add_pin_one_direction(&board, &moves.right, &mut pinned_pieces, has_rook_movement);
        check_and_add_pin_one_direction(&board, &moves.forward, &mut pinned_pieces, has_rook_movement);
        check_and_add_pin_one_direction(&board, &moves.back, &mut pinned_pieces, has_rook_movement);
        // Bishop
        check_and_add_pin_one_direction(&board, &moves.left_back, &mut pinned_pieces, has_bishop_movement);
        check_and_add_pin_one_direction(&board, &moves.left_forward, &mut pinned_pieces, has_bishop_movement);
        check_and_add_pin_one_direction(&board, &moves.right_back, &mut pinned_pieces, has_bishop_movement);
        check_and_add_pin_one_direction(&board, &moves.right_forward, &mut pinned_pieces, has_bishop_movement);
    }
    pinned_pieces
}

fn has_rook_movement(figure: &Figure) -> bool {
    figure.is_rook() || figure.is_queen()
}

fn has_bishop_movement(figure: &Figure) -> bool {
    figure.is_bishop() || figure.is_queen()
}

fn check_and_add_pin_one_direction(
    board: &Chessboard,
    moves: &Vec<usize>,
    pinned_pices: &mut HashMap<usize, Vec<usize>>,
    figure_has_correct_movement: fn(&Figure) -> bool,
) {
    let mut possible_pinned_piece: Option<usize> = None;
    for &single in moves {
        if let Some(opponent) = board.get_opponents().get(&single) {
            if figure_has_correct_movement(&opponent) {
                if let Some(pinned_piece) = possible_pinned_piece {
                    pinned_pices.insert(pinned_piece, moves.clone());
                }
                return
            }
            // field is opponent but not one that can pin
            return
        }
        if board.get_next_player_figures().contains_key(&single) {
            // two pieces from player -> no pin for a single one
            if possible_pinned_piece.is_some() {
                return
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
        let moves = get_moves_for_each_field();
        let mut board = Chessboard {
            ..Default::default()
        };
        // move white queen up on diagonal
        board.move_figure(3, 33);
        // move black pawn forward
        board.move_figure(51, 35);
        // white rook to pin center
        board.move_figure(0, 20);
        // black knight into pin of queen
        board.move_figure(57, 42);
        // white dummy move to give the move to black
        board.move_figure(8, 16);

        let pinned = get_pinned_pieces_and_possible_moves(&board, &60, &moves);
        // e pawn and knight on 42
        assert_eq!(2, pinned.len());
    }

    #[test]
    fn no_pinned_piece() {
        let moves = get_moves_for_each_field();
        let board = Chessboard {
            ..Default::default()
        };

        let pinned = get_pinned_pieces_and_possible_moves(&board, &4, &moves);
        assert_eq!(0, pinned.len());
    }

    #[test]
    fn not_pinned_as_there_in_a_pawn_in_between() {
        let moves = get_moves_for_each_field();
        let mut board = Chessboard {
            ..Default::default()
        };
        board.move_figure(4, 19);
        let pinned = get_pinned_pieces_and_possible_moves(&board, &19, &moves);
        assert_eq!(0, pinned.len());
    }
}
