use crate::{
    board::{bitboard::Bitboard, board::Chessboard}, engine::engine::PossibleMove, KING_MOVES
};

use super::color::Color;

pub fn get_possible_king_takes(
    board: &Chessboard,
    own_position: usize,
    opponent_moves: Bitboard,
    possible_moves: &mut Vec<PossibleMove>
){
    if let Some(moves_by_field) = KING_MOVES.get(own_position){
        let takes = Bitboard{board: moves_by_field.board & !opponent_moves.board & board.get_opponents().board};
        takes.iterate_board(|position| possible_moves.push(PossibleMove { from: own_position, to: position, promoted_to: None }));
    }
}

// does not include castle and should only be used when in check where castle is not possible
pub fn get_all_king_moves_in_check(
    board: &Chessboard,
    own_position: usize,
    opponent_moves: Bitboard,
    possible_moves: &mut Vec<PossibleMove>){
        if let Some(move_by_field) = KING_MOVES.get(own_position){
            let normal_movement = Bitboard{board: 
                move_by_field.board 
                & !board.get_positions_by_current_player().board
                & !opponent_moves.board
            };
            normal_movement.iterate_board(|pos| possible_moves.push(PossibleMove { from: own_position, to: pos, promoted_to: None }));
        }
    }

pub fn get_possible_king_moves(
    board: &Chessboard,
    own_position: usize,
    own_color: Color,
    opponent_moves: Bitboard,
    possible_moves: &mut Vec<PossibleMove>
){
    if let Some(move_by_field) = KING_MOVES.get(own_position){
        let normal_movement = Bitboard{board: 
            move_by_field.board 
            & !board.positions.board
            & !opponent_moves.board
        };
        normal_movement.iterate_board(|pos| possible_moves.push(PossibleMove { from: own_position, to: pos, promoted_to: None }));
    }

    // castle
    if board.castle.can_castle(own_color) && !opponent_moves.field_is_used(own_position) {
        match own_color {
            Color::White => white_castle(&board, opponent_moves, possible_moves),
            Color::Black => black_castle(&board, opponent_moves, possible_moves),
        }
    }
}

pub fn get_fields_threatened_by_king(own_position: usize) -> Bitboard {
    // fail is okay - we need to know as it would be a big bug
    return *KING_MOVES.get(own_position).unwrap();
}

fn is_possible_castle(
    board: &Chessboard,
    opponent_moves: Bitboard,
    new_king_position: usize,
    field_between: usize,
    // opt. field we need to empty check for long rochade
    long_rochade_free_field: Option<usize>,
) -> bool {

    if let Some(extra_field) = long_rochade_free_field {
        if board.positions.field_is_used(extra_field) {
            return false;
        }
    }

    return !(opponent_moves.field_is_used(field_between)
        || opponent_moves.field_is_used(new_king_position)
        || board.positions.field_is_used(field_between)
        || board.positions.field_is_used(new_king_position));
}

fn white_castle(
    board: &Chessboard,
    opponent_moves: Bitboard,
    possible_moves: &mut Vec<PossibleMove>,
) {
    // short
    if board.castle.white_castle_short && is_possible_castle(board, opponent_moves, 6, 5, None) {
        possible_moves.push(PossibleMove {
            to: 6,
            from: 4,
            promoted_to: None,
        });
    }
    // long
    if board.castle.white_castle_long && is_possible_castle(board, opponent_moves,  2, 3, Some(1)) {
        possible_moves.push(PossibleMove {
            to: 2,
            from: 4,
            promoted_to: None,
        });
    }
}

fn black_castle(
    board: &Chessboard,
    opponent_moves: Bitboard,
    possible_moves: &mut Vec<PossibleMove>
) {
    // short
    if board.castle.black_castle_short && is_possible_castle(board, opponent_moves, 62, 61, None) {
        possible_moves.push(PossibleMove {
            to: 62,
            from: 60,
            promoted_to: None,
        });
    }
    // long
    if board.castle.black_castle_long && is_possible_castle(board, opponent_moves, 58, 59, Some(57)) {
        possible_moves.push(PossibleMove {
            to: 58,
            from: 60,
            promoted_to: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        board::bitboard::Bitboard, figures::piece::Piece,
    };

    #[test]
    fn move_empty_board() {

        let mut board = Chessboard::empty(Color::White);
        board.castle.set_has_castled(Color::Black);

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 10, Color::Black,Bitboard::new(), &mut moves);
        assert_eq!(8, moves.len());

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 0,Color::Black, Bitboard::new(), &mut moves);
        assert_eq!(3, moves.len());

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 31,Color::Black, Bitboard::new(), &mut moves);
        assert_eq!(5, moves.len());
    }

    #[test]
    fn castle_on_empty_board() {
        let mut board = Chessboard::empty(Color::White);
        board.castle.white_castle_short = true;
        board.castle.white_castle_long = true;
        board.positions.set_field(0);
        board.positions.set_field(4);
        board.positions.set_field(7);
        
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(7);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(0);

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 4, Color::White,Bitboard::new(), &mut moves);

        let own_move_positions: Vec<usize> = moves.into_iter().map(|x| x.to).collect();
        // can castle left and right
        assert_eq!(7, own_move_positions.len());

        assert_eq!(true, own_move_positions.contains(&6));
        assert_eq!(true, own_move_positions.contains(&2));
    }

    #[test]
    fn not_able_to_castle_long() {
        let mut board = Chessboard::empty(Color::White);
        board.castle.white_castle_short = true;
        board.castle.white_castle_long = true;
        board.positions.set_field(0);
        board.positions.set_field(4);
        board.positions.set_field(7);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(7);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(0);

        let mut opponent_moves = Bitboard::new();
        opponent_moves.set_field(2);

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 4,Color::White, opponent_moves, &mut moves);
        let own_move_positions: Vec<usize> = moves.into_iter().map(|x| x.to).collect();
        assert_eq!(6, own_move_positions.len());
        assert_eq!(true, own_move_positions.contains(&6));
        assert_eq!(false, own_move_positions.contains(&2));
    }

    #[test]
    fn not_able_to_castle_long_as_extra_field_is_used() {
        let mut board = Chessboard::empty(Color::White);
        board.castle.white_castle_short = true;
        board.castle.white_castle_long = true;
        board.positions.set_field(0);
        board.positions.set_field(0);
        board.positions.set_field(4);
        board.positions.set_field(7);
        board.positions.set_field(1);

        board.figures[Color::White as usize][Piece::Rook as usize].set_field(7);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(0);

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 4,Color::White, Bitboard::new(), &mut moves);
        let own_move_positions: Vec<usize> = moves.into_iter().map(|x| x.to).collect();
        assert_eq!(6, own_move_positions.len());
        assert_eq!(true, own_move_positions.contains(&6));
        assert_eq!(false, own_move_positions.contains(&2));
    }

    #[test]
    fn not_able_to_castle() {


        let mut board = Chessboard::empty(Color::White);
        board.positions.set_field(0);
        board.positions.set_field(2);
        board.positions.set_field(4);
        board.positions.set_field(6);
        board.positions.set_field(7); 

        board.figures[Color::White as usize][Piece::Rook as usize].set_field(7);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(0);

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 4, Color::White,Bitboard::new(), &mut moves);
        let own_move_positions: Vec<usize> = moves.into_iter().map(|x| x.to).collect();

        // castle is not possible as there are figures in the way
        assert_eq!(5, own_move_positions.len());
        assert_eq!(false, own_move_positions.contains(&6));
        assert_eq!(false, own_move_positions.contains(&2));
    }

    #[test]
    fn not_beeing_able_to_move_as_all_fields_are_check() {
        let mut board = Chessboard {
            current_move: Color::Black,
            ..Default::default()
        };
        // king is on d8 and checked by queen on h8 - king cannot move as c7-e7 are full with pawns
        board.positions.set_field(50);
        board.positions.set_field(51);
        board.positions.set_field(52);
        board.positions.set_field(59);
        board.positions.set_field(63);
        // it should also not be able to move to c8
        board.figures[Color::White as usize][Piece::Queen as usize].set_field(63);
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(50);
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(51);
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(52);

        let mut opponent_moves = Bitboard::new();
        opponent_moves.set_field(62);
        opponent_moves.set_field(61);
        opponent_moves.set_field(60);
        opponent_moves.set_field(59);
        opponent_moves.set_field(58);
        opponent_moves.set_field(57);

        let mut moves = Vec::new();
        get_possible_king_moves(&board, 59,Color::Black, opponent_moves, &mut moves);
        assert_eq!(0, moves.len());
    }
}
