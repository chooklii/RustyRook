use crate::{
    board::{bitboard::Bitboard, board::Chessboard},
    figures::piece::Piece,
    KNIGHT_MOVES, MOVES_BY_FIELD, PAWN_THREATS,
};

// get all the fields we can place a figure (not king) to prevent a active check by the opponent
pub fn get_fields_to_prevent_check(
    board: &Chessboard,
    king_position: usize,
    opponent_moves: Bitboard
) -> Bitboard {
    let mut possible_fields = Bitboard::new();

    if let Some(rook_checking_field) = check_and_get_rook_movement_check_field(
        board,
        &king_position,
        opponent_moves,
    ){
        possible_fields.board |= rook_checking_field.board;
    }
    if let Some(bishop_checking_field) = check_and_get_bishop_movement_check_field(
        board,
        &king_position,
        opponent_moves,
    ) {
        possible_fields.board |= bishop_checking_field.board;
    } 
     if let Some(knight_check_field) =
        check_and_get_knight_check_field(board, king_position)
    {
        possible_fields.set_field(knight_check_field);
    } 
     if let Some(pawn_check_field) = check_and_get_pawn_check_field(board, king_position) {
        possible_fields.set_field(pawn_check_field);
    }

    possible_fields
}

fn is_rook_movement_figure(board: &Chessboard,position: usize) -> bool {
    return board.is_queen_or_rook(board.get_opponent_color(), position);
}

fn is_bishop_movement_figure(board: &Chessboard,position: usize) -> bool {
    return board.is_queen_or_bishop(board.get_opponent_color(), position);
}

fn check_and_get_bishop_movement_check_field(
    board: &Chessboard,
    king_position: &usize,
    opponent_moves: Bitboard
) -> Option<Bitboard> {
    if let Some(movement) = MOVES_BY_FIELD.get(king_position) {
        let left_fw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.left_forward,
            is_bishop_movement_figure,
        );

        if left_fw.is_some() {
            return left_fw;
        }
        let right_fw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.right_forward,
            is_bishop_movement_figure,
        );
        if right_fw.is_some() {
            return right_fw;
        }
        let left_bw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.left_back,
            is_bishop_movement_figure,
        );
        if left_bw.is_some() {
            return left_bw;
        }
        let right_bw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.right_back,
            is_bishop_movement_figure,
        );
        if right_bw.is_some() {
            return right_bw;
        }
    }
    return None;
}

fn check_and_get_rook_movement_check_field(
    board: &Chessboard,
    king_position: &usize,
    opponent_moves: Bitboard,
) -> Option<Bitboard> {
    if let Some(moves) = MOVES_BY_FIELD.get(king_position) {
        let left = check_single_direction_check(
            board,
            opponent_moves,
            &moves.left,
            is_rook_movement_figure,
        );
        if left.is_some() {
            return left;
        }
        // right
        let right = check_single_direction_check(
            board,
            opponent_moves,
            &moves.right,
            is_rook_movement_figure,
        );
        if right.is_some() {
            return right;
        }
        // forward
        let forward = check_single_direction_check(
            board,
            opponent_moves,
            &moves.forward,
            is_rook_movement_figure,
        );
        if forward.is_some() {
            return forward;
        }
        // backward
        let backward = check_single_direction_check(
            board,
            opponent_moves,
            &moves.back,
            is_rook_movement_figure,
        );
        if backward.is_some() {
            return backward;
        }
    }

    return None;
}

fn check_single_direction_check(
    board: &Chessboard,
    opponent_moves: Bitboard,
    moves: &Vec<usize>,
    figure_check: fn(&Chessboard, usize) -> bool
) -> Option<Bitboard> {
    let mut fields_to_prevent_check: Bitboard = Bitboard::new();
    for &movement in moves {
        if board.positions.field_is_used(movement) {
                if figure_check(&board, movement) {
                    fields_to_prevent_check.set_field(movement);
                    return Some(fields_to_prevent_check);
                }
            // field is used by our own figure or opponent not threatening us
            return None;
        } else if !opponent_moves.field_is_used(movement) {
            // opponent does not attack this field thus there can not be a attacker in this row
            return None;
        }
        fields_to_prevent_check.set_field(movement);
    }
    return None;
}

fn check_and_get_knight_check_field(
    board: &Chessboard,
    king_position: usize,
) -> Option<usize> {
    if let Some(moves) = KNIGHT_MOVES.get(king_position) {
        let checking_knigh_board = Bitboard{board: moves.board & board.get_opponent_piece(Piece::Knight).board};
        if checking_knigh_board.board != 0{
            return Some(checking_knigh_board.get_first_field());
        }
    }
    return None;
}

fn check_and_get_pawn_check_field(board: &Chessboard, position: usize) -> Option<usize> {
    let relevant_pawn_fields = PAWN_THREATS[board.current_move as usize][position];
    let possible_checks = Bitboard{board: relevant_pawn_fields.board & board.get_opponent_piece(Piece::Pawn).board};

    if possible_checks.board == 0{
        return None;
    }else{
        return Some(possible_checks.get_first_field());
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        board::bitboard::Bitboard, figures::color::Color
    };

    use super::*;

    #[test]
    fn test_rook_check() {

        let mut board = Chessboard {
            ..Default::default()
        };

        board.positions.set_field(19);
        board.positions.set_field(16);
        board.positions.set_field(1);

        board.figures[Color::White as usize][Piece::King as usize].set_field(19);
        board.figures[Color::Black as usize][Piece::Rook as usize].set_field(16);

        let mut opponent_moves = Bitboard::new();
        opponent_moves.set_field(19);
        opponent_moves.set_field(18);
        opponent_moves.set_field(17);
        assert_eq!(
            3,
            check_and_get_rook_movement_check_field(&board, &19, opponent_moves)
                .unwrap()
                .get_used_fields()
                .len()
        );
    }

    #[test]
    fn test_bishop_check() {
        let mut board = Chessboard {
            ..Default::default()
        };

        board.positions.set_field(19);
        board.positions.set_field(55);

        board.figures[Color::White as usize][Piece::King as usize].set_field(19);
        board.figures[Color::Black as usize][Piece::Bishop as usize].set_field(55);

        let mut opponent_moves = Bitboard::new();
        opponent_moves.set_field(46);
        opponent_moves.set_field(37);
        opponent_moves.set_field(28);
        opponent_moves.set_field(19);
        assert_eq!(4,
            check_and_get_bishop_movement_check_field(
                &board,
                &19,
                opponent_moves,
            )
            .unwrap()
            .get_used_fields()
            .len()
        );
    }

    #[test]
    fn test_pawn_check_white() {
        let mut board = Chessboard {
            ..Default::default()
        };

        board.positions.set_field(19);
        board.positions.set_field(26);

        board.figures[Color::White as usize][Piece::King as usize].set_field(19);
        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(26);
        let opponent_moves = Bitboard::new();
        let result = get_fields_to_prevent_check(&board, 19, opponent_moves).get_used_fields();
        assert_eq!(1, result.len());
        assert_eq!(true, result.contains(&26))
    }

    #[test]
    fn test_knigh_check_black() {
        let mut board = Chessboard {
            positions: Bitboard::new(),
            current_move: Color::Black,
            ..Default::default()
        };

        board.positions.set_field(8);
        board.positions.set_field(18);

        board.figures[Color::Black as usize][Piece::King as usize].set_field(8);
        board.figures[Color::White as usize][Piece::Knight as usize].set_field(18);

        let opponent_moves = Bitboard::new();
        let result = get_fields_to_prevent_check(&board, 8, opponent_moves).get_used_fields();
        assert_eq!(1, result.len());
        assert_eq!(true, result.contains(&18))
    }
}
