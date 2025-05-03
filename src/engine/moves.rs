use crate::{
    board::{bitboard::Bitboard, board::Chessboard},
    figures::{
        color::Color, king::{get_fields_threatened_by_king, get_possible_king_moves, get_possible_king_takes}, knight::{
            get_fields_threatened_by_knight, get_possible_knight_moves, get_possible_knight_moves_to_prevent_check, get_possible_knight_takes
        }, pawn::{
            get_fields_threatened_by_pawn, get_possible_pawn_moves, get_possible_pawn_moves_to_prevent_check, get_possible_pawn_takes_and_promotion
        }, piece::Piece, sliding_moves::{
            get_fields_threatened_by_bishop, get_fields_threatened_by_queen, get_fields_threatened_by_rook, get_possible_bishop_moves, get_possible_bishop_moves_to_prevent_check, get_possible_bishop_takes, get_possible_queen_moves, get_possible_queen_moves_to_prevent_check, get_possible_queen_takes, get_possible_rook_moves, get_possible_rook_moves_to_prevent_check, get_possible_rook_takes
        }
    },
};

use super::{
    checked::get_fields_to_prevent_check, engine::PossibleMove,
    ray::get_pinned_pieces_and_possible_moves,
};

pub fn get_takes_in_position(
    board: &Chessboard,
) -> (Vec<PossibleMove>, bool) {
    let king_position = board
        .get_pieces(board.current_move, Piece::King)
        .get_first_field();
    // get moves from opponent - we ignore our own king position for rook/bishop/queen to standing on d8, and going to c8 to prevent check from h8
    let (opponent_moves, count_of_checks) =
        get_all_threatened_fields(&board, board.get_opponent_color(), king_position);

    // if opponent moves include own king -> we are in check
    let is_in_check = opponent_moves.field_is_used(king_position);

    let moves = if is_in_check {
        let all_moves =
            get_all_possible_moves(&board, board.current_move, opponent_moves, true, true);
        let prevent_check_fields =
            get_fields_to_prevent_check(&board, king_position, opponent_moves);
        // either figure is king (we allow all his moves - or figure can prevent check)
        all_moves
            .into_iter()
            .filter(|mov| {
                prevent_check_fields.field_is_used(mov.to)
                    || mov.from.eq(&king_position)
                    || en_passant_to_prevent_check(&board, &mov, prevent_check_fields)
            })
            .collect()
    } else {
        get_all_possible_takes(&board, board.current_move, opponent_moves)
    };
    let not_pinned_moves: Vec<PossibleMove> =
        get_not_pinned_pieces(&board, &king_position, moves);
    return (not_pinned_moves, is_in_check);
}

pub fn get_valid_moves_in_position(
    board: &Chessboard,
) -> (Vec<PossibleMove>, bool) {
    let king_position = board
        .get_pieces(board.current_move, Piece::King)
        .get_first_field();
    // get moves from opponent - we ignore our own king position for rook/bishop/queen to standing on d8, and going to c8 to prevent check from h8
    let (opponent_moves, count_of_checks) =
        get_all_threatened_fields(&board, board.get_opponent_color(), king_position);
    // if opponent moves include own king -> we are in check
    let is_in_check = opponent_moves.field_is_used(king_position);
    let is_in_double_check = is_in_check && count_of_checks > 1;

    let moves: Vec<PossibleMove> = get_all_possible_moves(&board, board.current_move, opponent_moves, is_in_check, is_in_double_check);

    let not_pinned_moves: Vec<PossibleMove> =
        get_not_pinned_pieces(&board, &king_position, moves);
    return (not_pinned_moves, is_in_check);
}

// get all fields threadned (ignore if opponent figure is on field)
fn get_all_threatened_fields(
    board: &Chessboard,
    color: Color,
    king_position: usize
) -> (Bitboard, u8) {
    let mut moves = Bitboard::new();
    let mut cound_of_checks = 0;

    let pawn_positions = board.get_pieces(color, Piece::Pawn);
    pawn_positions.iterate_board(|position| {
        let pawn_moves = get_fields_threatened_by_pawn(position, color);
        moves.board = moves.board | pawn_moves.board;

        if pawn_moves.field_is_used(king_position){
            cound_of_checks+=1;
        }
    });

    let rook_positions: &Bitboard = board.get_pieces(color, Piece::Rook);
    rook_positions.iterate_board(|position| {
        let rook_moves = get_fields_threatened_by_rook(
            &board,
            position,
            king_position
        );
        moves.board = moves.board | rook_moves.board;

        if rook_moves.field_is_used(king_position){
            cound_of_checks+=1;
        }
    });

    let bishop_positions = board.get_pieces(color, Piece::Bishop);
    bishop_positions.iterate_board(|position| {
        let bishop_moves = get_fields_threatened_by_bishop(
            &board,
            position,
            king_position
        );
        moves.board = moves.board | bishop_moves.board;

        if bishop_moves.field_is_used(king_position){
            cound_of_checks+=1;
        }
    });

    let queen_positions = board.get_pieces(color, Piece::Queen);
    queen_positions.iterate_board(|position| {
        let queen_moves = get_fields_threatened_by_queen(
            &board,
            position,
            king_position
        );
        moves.board = moves.board | queen_moves.board;

        if queen_moves.field_is_used(king_position){
            cound_of_checks+=1;
        }
    });

    let knight_positions = board.get_pieces(color, Piece::Knight);
    knight_positions.iterate_board(|position| {
        let knight_moves = get_fields_threatened_by_knight(position);
        moves.board = moves.board | knight_moves.board;

        if knight_moves.field_is_used(king_position){
            cound_of_checks+=1;
        }
    });

    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    moves.board = moves.board | get_fields_threatened_by_king(king_position).board;
    (moves, cound_of_checks)
}

fn get_all_prevent_check_moves(
    board: &Chessboard,
    color: Color,
    opponent_moves: Bitboard,
) -> Vec<PossibleMove>{
    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    let prevent_check_fields = get_fields_to_prevent_check(&board, king_position, opponent_moves);

    let mut moves = Vec::new();

    let bishop_positions = board.get_pieces(color, Piece::Bishop);
    bishop_positions.iterate_board(|position| {
        get_possible_bishop_moves_to_prevent_check(&board, position, prevent_check_fields, &mut moves);
    });

    let queen_positions = board.get_pieces(color, Piece::Queen);
    queen_positions.iterate_board(|position| {
        get_possible_queen_moves_to_prevent_check(&board, position, prevent_check_fields, &mut moves);
    });
    
    let knight_positions = board.get_pieces(color, Piece::Knight);
    knight_positions.iterate_board(|position| {
        get_possible_knight_moves_to_prevent_check(position, prevent_check_fields, &mut moves);
    });

    let rook_positions = board.get_pieces(color, Piece::Rook);
    rook_positions.iterate_board(|position| {
        get_possible_rook_moves_to_prevent_check(&board, position, prevent_check_fields, &mut moves);
    });

    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    moves.append(&mut get_possible_king_moves(&board, king_position, color, opponent_moves));

    let pawn_positions = board.get_pieces(color, Piece::Pawn);   
    pawn_positions.iterate_board(|position| {
        get_possible_pawn_moves_to_prevent_check(&board, position, color, prevent_check_fields, &mut moves);
    });

    moves
}
// default logic get all pseudo legal moves
fn get_all_possible_moves(
    board: &Chessboard,
    color: Color,
    opponent_moves: Bitboard,
    is_in_check: bool,
    is_in_double_check: bool,
) -> Vec<PossibleMove> {

    // if we are in double check only moving the king can save us
    if is_in_double_check{
        let king_position = board.get_pieces(color, Piece::King).get_first_field();
        return get_possible_king_moves(&board, king_position, color, opponent_moves);
    }
    if is_in_check{
        return get_all_prevent_check_moves(board, color, opponent_moves);
    }
    let mut moves = Vec::new();

    let bishop_positions = board.get_pieces(color, Piece::Bishop);
    bishop_positions.iterate_board(|position| {
        moves.append(&mut get_possible_bishop_moves(&board, position));
    });

    let queen_positions = board.get_pieces(color, Piece::Queen);
    queen_positions.iterate_board(|position| {
        moves.append(&mut get_possible_queen_moves(&board, position));
    });
    
    let knight_positions = board.get_pieces(color, Piece::Knight);
    knight_positions.iterate_board(|position| {
        moves.append(&mut get_possible_knight_moves(&board, position));
    });

    let rook_positions = board.get_pieces(color, Piece::Rook);
    rook_positions.iterate_board(|position| {
        moves.append(&mut get_possible_rook_moves(&board, position));
    });

    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    moves.append(&mut get_possible_king_moves(&board, king_position, color, opponent_moves));

    let pawn_positions = board.get_pieces(color, Piece::Pawn);   
    pawn_positions.iterate_board(|position| {
        moves.append(&mut get_possible_pawn_moves(&board, position, color));
    });

    moves
}

fn get_all_possible_takes(
    board: &Chessboard,
    color: Color,
    opponent_moves: Bitboard
) -> Vec<PossibleMove> {
    let mut moves = Vec::new();

    let pawn_positions = board.get_pieces(color, Piece::Pawn);
    pawn_positions.iterate_board(|position| {
        get_possible_pawn_takes_and_promotion(&board, position, color, &mut moves);
    });

    let rook_positions = board.get_pieces(color, Piece::Rook);
    rook_positions.iterate_board(|position| {
        get_possible_rook_takes(&board, position, &mut moves);
    });

    let bishop_positions = board.get_pieces(color, Piece::Bishop);
    bishop_positions.iterate_board(|position| {
        get_possible_bishop_takes(&board, position, &mut moves);
    });

    let queen_positions = board.get_pieces(color, Piece::Queen);
    queen_positions.iterate_board(|position| {
        get_possible_queen_takes(&board, position, &mut moves);
    });
    
    let knight_positions = board.get_pieces(color, Piece::Knight);
    knight_positions.iterate_board(|position| {
        get_possible_knight_takes(&board, position, &mut moves);
    });

    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    get_possible_king_takes(&board, king_position, opponent_moves, &mut moves);
    moves
}

fn get_not_pinned_pieces(
    board: &Chessboard,
    king_position: &usize,
    moves: Vec<PossibleMove>,
) -> Vec<PossibleMove> {
    let pinned_pieces =
        get_pinned_pieces_and_possible_moves(&board, &king_position);

    if pinned_pieces.is_empty() {
        return moves;
    }
    // filter out all moves from pinned pieces - but keep the moves on the same "line" as pinner (e.g. Pinned Rook can capture pinning Rook)
    moves
        .into_iter()
        // we have estabilshed, that key is defined (unwrap)
        .filter(|mov| {
            !pinned_pieces.contains_key(&mov.from)
                || pinned_pieces.get(&mov.from).unwrap().contains(&mov.to)
        })
        .collect()
}

// check if move is en en passant to prevent a check given from a pawn
fn en_passant_to_prevent_check(
    board: &Chessboard,
    mov: &PossibleMove,
    prevent_check_fields: Bitboard,
) -> bool {
    // if there is more than one field to prevent check it cannot be from a pawn and prevented by en passant
    if board.en_passant.is_none() || prevent_check_fields.board.count_ones() > 1 {
        return false;
    }
    // both fields are null checked above
    let checked_by_field = prevent_check_fields.get_first_field();
    let en_passant_field = board.en_passant.unwrap();
    if checked_by_field != en_passant_field {
        return false;
    }
    let own_pawns = board.get_pieces(board.current_move, Piece::Pawn);

    if own_pawns.field_is_used(mov.from) {
        return match board.current_move {
            Color::Black => mov.to + 8 == en_passant_field,
            Color::White => mov.to - 8 == en_passant_field,
        };
    }
    return false;
}
