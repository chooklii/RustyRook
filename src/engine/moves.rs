use crate::{
    board::{bitboard::Bitboard, board::Chessboard},
    figures::{
        color::Color,
        king::{
            get_all_king_moves_in_check, get_fields_threatened_by_king, get_possible_king_moves,
            get_possible_king_takes,
        },
        knight::{
            get_fields_threatened_by_knight, get_possible_knight_moves,
            get_possible_knight_moves_to_prevent_check, get_possible_knight_takes,
        },
        pawn::{
            get_fields_threatened_by_pawn, get_possible_pawn_moves,
            get_possible_pawn_moves_to_prevent_check, get_possible_pawn_takes_and_promotion,
        },
        piece::Piece,
        sliding_moves::{
            get_fields_threatened_by_bishop, get_fields_threatened_by_queen,
            get_fields_threatened_by_rook, get_possible_bishop_moves,
            get_possible_bishop_moves_to_prevent_check, get_possible_bishop_takes,
            get_possible_queen_moves, get_possible_queen_moves_to_prevent_check,
            get_possible_queen_takes, get_possible_rook_moves,
            get_possible_rook_moves_to_prevent_check, get_possible_rook_takes,
        },
    },
};

use super::{
    checked::get_fields_to_prevent_check,
    engine::PossibleMove,
    ray::get_pinned_pieces_and_possible_moves,
    transposition::{table::{get_entry_without_check}, transposition::Transposition},
};

pub fn get_valid_moves_in_position(
    board: &Chessboard,
    calculate_all_moves: bool,
) -> (Vec<PossibleMove>, bool) {
    let king_position = board
        .get_pieces(board.current_move, Piece::King)
        .get_first_field();
    // get moves from opponent - we ignore our own king position for rook/bishop/queen to standing on d8, and going to c8 to prevent check from h8
    let (opponent_moves, count_of_checks) =
        get_all_threatened_fields(board, board.get_opponent_color(), king_position);
    // if opponent moves include own king -> we are in check
    let is_in_check = opponent_moves.field_is_used(king_position);
    let is_in_double_check = is_in_check && count_of_checks > 1;

    let moves: Vec<PossibleMove> = get_all_possible_moves(
        board,
        board.current_move,
        opponent_moves,
        is_in_check,
        is_in_double_check,
        calculate_all_moves,
    );
    let not_pinned_moves: Vec<PossibleMove> = get_not_pinned_pieces(board, &king_position, moves);
    (not_pinned_moves, is_in_check)
}

// get all fields threadned (ignore if opponent figure is on field)
fn get_all_threatened_fields(
    board: &Chessboard,
    color: Color,
    king_position: usize,
) -> (Bitboard, u8) {
    let mut moves = Bitboard::new();
    let mut cound_of_checks = 0;

    let pawn_positions = board.get_pieces(color, Piece::Pawn);
    pawn_positions.iterate_board(|position| {
        let pawn_moves = get_fields_threatened_by_pawn(position, color);
        moves.board |= pawn_moves.board;

        if pawn_moves.field_is_used(king_position) {
            cound_of_checks += 1;
        }
    });

    let rook_positions: &Bitboard = board.get_pieces(color, Piece::Rook);
    rook_positions.iterate_board(|position| {
        let rook_moves = get_fields_threatened_by_rook(board, position, king_position);
        moves.board |= rook_moves.board;

        if rook_moves.field_is_used(king_position) {
            cound_of_checks += 1;
        }
    });

    let bishop_positions = board.get_pieces(color, Piece::Bishop);
    bishop_positions.iterate_board(|position| {
        let bishop_moves = get_fields_threatened_by_bishop(board, position, king_position);
        moves.board |= bishop_moves.board;

        if bishop_moves.field_is_used(king_position) {
            cound_of_checks += 1;
        }
    });

    let queen_positions = board.get_pieces(color, Piece::Queen);
    queen_positions.iterate_board(|position| {
        let queen_moves = get_fields_threatened_by_queen(board, position, king_position);
        moves.board |= queen_moves.board;

        if queen_moves.field_is_used(king_position) {
            cound_of_checks += 1;
        }
    });

    let knight_positions = board.get_pieces(color, Piece::Knight);
    knight_positions.iterate_board(|position| {
        let knight_moves = get_fields_threatened_by_knight(position);
        moves.board |= knight_moves.board;

        if knight_moves.field_is_used(king_position) {
            cound_of_checks += 1;
        }
    });

    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    moves.board |= get_fields_threatened_by_king(king_position).board;
    (moves, cound_of_checks)
}

fn get_all_prevent_check_moves(
    board: &Chessboard,
    color: Color,
    opponent_moves: Bitboard,
) -> Vec<PossibleMove> {
    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    let prevent_check_fields = get_fields_to_prevent_check(board, king_position, opponent_moves);

    let mut moves = Vec::new();

    let bishop_positions = board.get_pieces(color, Piece::Bishop);
    bishop_positions.iterate_board(|position| {
        get_possible_bishop_moves_to_prevent_check(
            board,
            position,
            prevent_check_fields,
            &mut moves,
        );
    });

    let queen_positions = board.get_pieces(color, Piece::Queen);
    queen_positions.iterate_board(|position| {
        get_possible_queen_moves_to_prevent_check(
            board,
            position,
            prevent_check_fields,
            &mut moves,
        );
    });

    let knight_positions = board.get_pieces(color, Piece::Knight);
    knight_positions.iterate_board(|position| {
        get_possible_knight_moves_to_prevent_check(position, prevent_check_fields, &mut moves);
    });

    let rook_positions = board.get_pieces(color, Piece::Rook);
    rook_positions.iterate_board(|position| {
        get_possible_rook_moves_to_prevent_check(
            board,
            position,
            prevent_check_fields,
            &mut moves,
        );
    });

    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    get_all_king_moves_in_check(board, king_position, opponent_moves, &mut moves);

    let pawn_positions = board.get_pieces(color, Piece::Pawn);
    pawn_positions.iterate_board(|position| {
        get_possible_pawn_moves_to_prevent_check(
            board,
            position,
            color,
            prevent_check_fields,
            &mut moves,
        );
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
    get_all_moves: bool,
) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    // if we are in double check only moving the king can save us
    if is_in_double_check {
        let king_position = board.get_pieces(color, Piece::King).get_first_field();
        get_all_king_moves_in_check(board, king_position, opponent_moves, &mut moves);
        return moves;
    }
    if is_in_check {
        return get_all_prevent_check_moves(board, color, opponent_moves);
    }

    let bishop_positions = board.get_pieces(color, Piece::Bishop);
    let queen_positions = board.get_pieces(color, Piece::Queen);
    let knight_positions = board.get_pieces(color, Piece::Knight);
    let rook_positions = board.get_pieces(color, Piece::Rook);
    let king_position = board.get_pieces(color, Piece::King).get_first_field();
    let pawn_positions = board.get_pieces(color, Piece::Pawn);

    // add all takes to possible Moves
    pawn_positions.iterate_board(|position| {
        get_possible_pawn_takes_and_promotion(board, position, color, &mut moves);
    });

    rook_positions.iterate_board(|position| {
        get_possible_rook_takes(board, position, &mut moves);
    });

    bishop_positions.iterate_board(|position| {
        get_possible_bishop_takes(board, position, &mut moves);
    });

    queen_positions.iterate_board(|position| {
        get_possible_queen_takes(board, position, &mut moves);
    });

    knight_positions.iterate_board(|position| {
        get_possible_knight_takes(board, position, &mut moves);
    });
    get_possible_king_takes(board, king_position, opponent_moves, &mut moves);

    let prev_best_move_opt = get_entry_without_check(board.zobrist_key);
    // we only want some moves which we think should be calculated
    if !get_all_moves {
        add_prev_best_move_as_first_move(&mut moves, prev_best_move_opt);
        return moves;
    }

    // add all silent moves (not takes) at the end
    bishop_positions.iterate_board(|position| {
        get_possible_bishop_moves(board, position, &mut moves);
    });

    queen_positions.iterate_board(|position| {
        get_possible_queen_moves(board, position, &mut moves);
    });

    rook_positions.iterate_board(|position| {
        get_possible_rook_moves(board, position, &mut moves);
    });

    knight_positions.iterate_board(|position| {
        get_possible_knight_moves(board, position, &mut moves);
    });

    get_possible_king_moves(board, king_position, color, opponent_moves, &mut moves);

    pawn_positions.iterate_board(|position| {
        get_possible_pawn_moves(board, position, color, &mut moves);
    });
    add_prev_best_move_as_first_move(&mut moves, prev_best_move_opt);
    moves
}

fn add_prev_best_move_as_first_move(
    moves: &mut Vec<PossibleMove>,
    prev_best_move_opt: Option<Transposition>,
) {
    if prev_best_move_opt.is_none() {
        return;
    }
    // order to first position (or add previous best move)
    // null checked before
    let prev_best_move = prev_best_move_opt.unwrap().best_move;
    if let Some(pos) = moves.iter().position(|single| {
        single.from == prev_best_move.from
            && single.to == prev_best_move.to
            && single.promoted_to == prev_best_move.promoted_to
    }) {
        moves.remove(pos);
    }
    moves.insert(0, prev_best_move);
}

fn get_not_pinned_pieces(
    board: &Chessboard,
    king_position: &usize,
    moves: Vec<PossibleMove>,
) -> Vec<PossibleMove> {
    let pinned_pieces = get_pinned_pieces_and_possible_moves(board, king_position);

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