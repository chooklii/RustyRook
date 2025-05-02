use crate::{board::{bitboard::Bitboard, board::Chessboard}, engine::engine::PossibleMove, KNIGHT_MOVES};

pub fn get_possible_knight_moves(
    board: &Chessboard,
    own_position: usize
) -> Vec<PossibleMove> {
    let mut possible_moves = Vec::new();
    if let Some(moves) = KNIGHT_MOVES.get(own_position) {
        let own_positions = board.get_positions_by_current_player();
        let movement = Bitboard{board: moves.board & !own_positions.board};
        movement.iterate_board(|position| possible_moves.push(PossibleMove { from: own_position, to: position, promoted_to: None }));      
    }
    possible_moves
}

pub fn get_fields_threatened_by_knight(
    own_position: usize
) -> Bitboard {
    // if field is not defined we want to know and fail
    return *KNIGHT_MOVES.get(own_position).unwrap();
}

pub fn get_possible_knight_takes(
    board: &Chessboard,
    own_position: usize,
    possible_takes: &mut Vec<PossibleMove>
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

        let moves = get_possible_knight_moves(&board, 27);
        assert_eq!(8, moves.len());

        let moves = get_possible_knight_moves(&board, 0);
        assert_eq!(2, moves.len());

        let moves = get_possible_knight_moves(&board, 54);
        assert_eq!(4, moves.len());
    }

    #[test]
    fn test_takes_default_board() {
        let board = Chessboard {
            ..Default::default()
        };

        let mut moves: Vec<PossibleMove> = Vec::new();
        get_possible_knight_takes(&board, 1, &mut moves);
        assert_eq!(0, moves.len());

        let mut moves: Vec<PossibleMove> = Vec::new();
        get_possible_knight_takes(&board, 33, &mut moves);
        // 48, 50
        assert_eq!(2, moves.len());
    }
}
