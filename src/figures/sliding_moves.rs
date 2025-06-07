use crate::{board::{bitboard::Bitboard, board::Chessboard}, engine::engine::PossibleMove, helper::magic_bitboards::helper::get_magic_index, BISHOP_MAGIC_BITBOARDS, BISHOP_MAGIC_POSITIONS, ROOK_MAGIC_BITBOARDS, ROOK_MAGIC_POSITIONS};

pub fn get_fields_threatened_by_queen(
    board: &Chessboard,
    position: usize,
    king_position: usize
) -> Bitboard {
    let bishop_threats = get_fields_threatened_by_bishop(&board, position, king_position);
    let rook_threats = get_fields_threatened_by_rook(&board, position, king_position);
    Bitboard { board: bishop_threats.board | rook_threats.board }
}


pub fn get_fields_threatened_by_bishop(
    board: &Chessboard,
    position: usize,
    king_position: usize
) -> Bitboard {
    let mut board_without_king = board.positions;
    board_without_king.remove_field(king_position);

    let move_options = &BISHOP_MAGIC_POSITIONS[position];
    let magic_options = &BISHOP_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board_without_king, &magic_options)];

    Bitboard{board:magic_moves.board}
}

pub fn get_fields_threatened_by_rook(
    board: &Chessboard,
    position: usize,
    king_position: usize
) -> Bitboard {
    // remove own king to prevent bug where king moves on same line as attacker "as field is not attacked" (he is blocking it himself)
    let mut board_without_king = board.positions;
    board_without_king.remove_field(king_position);

    let move_options = &ROOK_MAGIC_POSITIONS[position];
    let magic_options = &ROOK_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board_without_king, &magic_options)];

    Bitboard{board:magic_moves.board}
}

pub fn get_possible_queen_moves(
    board: &Chessboard,
    position: usize,
    possible_moves: &mut Vec<PossibleMove>
) {
    get_possible_bishop_moves(&board, position, possible_moves);
    get_possible_rook_moves(&board, position, possible_moves);

}

pub fn get_possible_bishop_moves(
    board: &Chessboard,
    position: usize,
    possible_moves: &mut Vec<PossibleMove>
){
    let move_options = &BISHOP_MAGIC_POSITIONS[position];
    let magic_options = &BISHOP_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board.positions, &magic_options)];
    let moves = Bitboard{board:magic_moves.board & !board.positions.board};
    moves.iterate_board(|mov| possible_moves.push(PossibleMove { from: position, to: mov, promoted_to: None }));
}

pub fn get_possible_rook_moves(
    board: &Chessboard,
    position: usize,
    possible_moves: &mut Vec<PossibleMove>
) {
    let move_options = &ROOK_MAGIC_POSITIONS[position];
    let magic_options = &ROOK_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board.positions, &magic_options)];
    let moves = Bitboard{board:magic_moves.board & !board.positions.board};
    moves.iterate_board(|mov| possible_moves.push(PossibleMove { from: position, to: mov, promoted_to: None }));
}

pub fn get_possible_rook_moves_to_prevent_check(
    board: &Chessboard,
    position: usize,
    prevent_check_fields: Bitboard,
    possible_moves: &mut Vec<PossibleMove>,
){
    let move_options = &ROOK_MAGIC_POSITIONS[position];
    let magic_options = &ROOK_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board.positions, &magic_options)];

    let moves = Bitboard{board:magic_moves.board & prevent_check_fields.board};
    moves.iterate_board(|mov| possible_moves.push(PossibleMove { from: position, to: mov, promoted_to: None }));
}

pub fn get_possible_bishop_takes(
    board: &Chessboard,
    position: usize,
    possible_moves: &mut Vec<PossibleMove>
){
    let move_options = &BISHOP_MAGIC_POSITIONS[position];
    let magic_options = &BISHOP_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board.positions, &magic_options)];

    let moves = Bitboard{board:magic_moves.board & board.get_opponents().board};

    moves.iterate_board(|mov| possible_moves.push(PossibleMove { from: position, to: mov, promoted_to: None }));
}

pub fn get_possible_bishop_moves_to_prevent_check(
    board: &Chessboard,
    position: usize,
    prevent_check_fields: Bitboard,
    possible_moves: &mut Vec<PossibleMove>,
){
    let move_options = &BISHOP_MAGIC_POSITIONS[position];
    let magic_options = &BISHOP_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board.positions, &magic_options)];

    let moves = Bitboard{board:magic_moves.board & prevent_check_fields.board};
    moves.iterate_board(|mov| possible_moves.push(PossibleMove { from: position, to: mov, promoted_to: None }));
}

pub fn get_possible_rook_takes(
    board: &Chessboard,
    position: usize,
    possible_moves: &mut Vec<PossibleMove>
){
    let move_options = &ROOK_MAGIC_POSITIONS[position];
    let magic_options = &ROOK_MAGIC_BITBOARDS[position];
    let magic_moves = move_options[get_magic_index(board.positions, &magic_options)];

    let moves = Bitboard{board:magic_moves.board & board.get_opponents().board};

    moves.iterate_board(|mov| possible_moves.push(PossibleMove { from: position, to: mov, promoted_to: None }));
}

pub fn get_possible_queen_takes(
    board: &Chessboard,
    position: usize,
    possible_moves: &mut Vec<PossibleMove>
){
    get_possible_rook_takes(board, position, possible_moves);
    get_possible_bishop_takes(board, position, possible_moves);
}

pub fn get_possible_queen_moves_to_prevent_check(
    board: &Chessboard,
    position: usize,
    prevent_check_fields: Bitboard,
    possible_moves: &mut Vec<PossibleMove>,
){
    get_possible_rook_moves_to_prevent_check(board, position, prevent_check_fields, possible_moves);
    get_possible_bishop_moves_to_prevent_check(board, position, prevent_check_fields, possible_moves);
}

#[cfg(test)]
mod tests {
    use crate::{board::bitboard::Bitboard, figures::color::Color};

    use super::*;

    #[test]
    fn move_bishop_empty_board() {
        let board = Chessboard::empty(Color::White);

        let mut moves = Vec::new();

        get_possible_bishop_moves(&board, 27, &mut moves);
        assert_eq!(13, moves.len());

        let mut moves = Vec::new();
        get_possible_bishop_moves(&board, 0, &mut moves);
        assert_eq!(7, moves.len());
    }

    #[test]
    fn bishop_not_able_to_move() {
        let mut board = Chessboard::empty(Color::White);
        board.used_positions[Color::White as usize].set_field(25);
        board.used_positions[Color::White as usize].set_field(27);
        board.used_positions[Color::White as usize].set_field(9);
        board.used_positions[Color::White as usize].set_field(11);

        board.positions.set_field(25);
        board.positions.set_field(27);
        board.positions.set_field(9);
        board.positions.set_field(11);

        let mut moves = Vec::new();
        get_possible_bishop_moves(&board, 18, &mut moves);
        assert_eq!(0, moves.len());
    }

    #[test]
    fn bishop_able_to_move_in_two_directions() {
        let mut positions = Bitboard::new();
        positions.set_field(29);
        positions.set_field(13);
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let mut moves = Vec::new();
        get_possible_bishop_moves(&board, 20, &mut moves);
        assert_eq!(6, moves.len());
    }

    #[test]
    fn queen_move_empty_board() {

        let mut moves = Vec::new();
        let board = Chessboard::empty(Color::White);
        get_possible_queen_moves(&board, 0, &mut moves);
        assert_eq!(21, moves.len());

        let mut moves = Vec::new();
        get_possible_queen_moves(&board, 19, &mut moves);
        assert_eq!(25, moves.len());
    }

    #[test]
    fn test_bishop_in_corner_empty_board(){
        let board = Chessboard::empty(Color::White);
        let mut moves = Vec::new();
        get_possible_bishop_moves(&board, 0, &mut moves);
        assert_eq!(7, moves.len());
    }

    #[test]
    fn test_queen_in_corner_empty_board(){
        let board = Chessboard::empty(Color::White);
        let mut moves = Vec::new();
        get_possible_queen_moves(&board, 0, &mut moves);
        assert_eq!(21, moves.len());
    }


    #[test]
    fn rook_test_move_forward() {
        let mut positions = Bitboard::new();

        positions.set_field(24);
        positions.set_field(1);
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let mut moves = Vec::new();
        get_possible_rook_moves(&board, 0, &mut moves);
        assert_eq!(2, moves.len())
    }

    #[test]
    fn rook_test_move_backward() {
        let mut board = Chessboard::empty(Color::White);

        board.positions.set_field(18);
        board.positions.set_field(25);
        board.positions.set_field(27);
        board.used_positions[Color::White as usize].set_field(18);
        board.used_positions[Color::White as usize].set_field(25);
        board.used_positions[Color::White as usize].set_field(27);

        let mut moves = Vec::new();
        get_possible_rook_moves(&board, 26, &mut moves);
        assert_eq!(4, moves.len())
    }

    #[test]
    fn rook_test_movement_on_empty_board() {
        let board = Chessboard::empty(Color::White);
        let mut moves = Vec::new();
        get_possible_rook_moves(&board, 11, &mut moves);
        assert_eq!(14, moves.len())
    }
}