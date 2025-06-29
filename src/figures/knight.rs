use smallvec::SmallVec;

use crate::{board::{bitboard::Bitboard, board::Chessboard}, engine::engine::PossibleMove, KNIGHT_MOVES};

pub fn get_possible_knight_moves(
    board: &Chessboard,
    own_position: usize,
    possible_moves: &mut SmallVec<[PossibleMove; 64]>
){
    if let Some(moves) = KNIGHT_MOVES.get(own_position) {
        let movement = Bitboard{board: moves.board & !board.positions.board};
        movement.iterate_board(|position| possible_moves.push(PossibleMove { from: own_position, to: position, promoted_to: None }));      
    }
}

pub fn get_fields_threatened_by_knight(
    own_position: usize
) -> Bitboard {
    // if field is not defined we want to know and fail
    *KNIGHT_MOVES.get(own_position).unwrap()
}

pub fn get_possible_knight_moves_to_prevent_check(
    own_position: usize,
    prevent_check_fields: Bitboard,
    possible_moves: &mut SmallVec<[PossibleMove; 64]>
){
    let moves = Bitboard{board: KNIGHT_MOVES[own_position].board & prevent_check_fields.board};
    moves.iterate_board(|single| possible_moves.push(PossibleMove { from:own_position, to: single, promoted_to: None }));
}

pub fn get_possible_knight_takes(
    board: &Chessboard,
    own_position: usize,
    possible_takes: &mut SmallVec<[PossibleMove; 64]>
){
    if let Some(moves) = KNIGHT_MOVES.get(own_position) {
        let movement = Bitboard{board: moves.board & board.get_opponents().board};
        movement.iterate_board(|position| possible_takes.push(PossibleMove { to: position, from: own_position, promoted_to: None }));
    }
}

#[cfg(test)]
mod tests {

    use crate::figures::color::Color;

    use super::*;

    #[test]
    fn test_empty_board() {
        let board = Chessboard::empty(Color::White);

        let mut moves = SmallVec::new();
        get_possible_knight_moves(&board, 27, &mut moves);
        assert_eq!(8, moves.len());

        let mut moves = SmallVec::new();
        get_possible_knight_moves(&board, 0, &mut moves);
        assert_eq!(2, moves.len());

        let mut moves = SmallVec::new();
        get_possible_knight_moves(&board, 54, &mut moves);
        assert_eq!(4, moves.len());
    }

    #[test]
    fn test_takes_default_board() {
        let board = Chessboard {
            ..Default::default()
        };

        let mut moves = SmallVec::new();
        get_possible_knight_takes(&board, 1, &mut moves);
        assert_eq!(0, moves.len());

        let mut moves = SmallVec::new();
        get_possible_knight_takes(&board, 33, &mut moves);
        // 48, 50
        assert_eq!(2, moves.len());
    }
}
