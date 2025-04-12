use rustc_hash::FxHashMap;

use crate::{
    board::board::Chessboard,
    figures::{color::Color, figures::Figure},
    helper::moves_by_field::MoveInEveryDirection,
};

use super::{
    checked::get_fields_to_prevent_check, engine::PossibleMove,
    ray::get_pinned_pieces_and_possible_moves,
};

pub fn get_takes_in_position(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> (Vec<PossibleMove>, bool) {
    let (king_position, _) = get_own_king(&board);
    // get moves from opponent - we ignore our own king position for rook/bishop/queen to standing on d8, and going to c8 to prevent check from h8
    let opponent_moves: Vec<usize> =
        get_all_threatened_fields(&board, &moves_by_field, &king_position);

    // if opponent moves include own king -> we are in check
    let is_in_check = opponent_moves.contains(king_position);

    let moves = if is_in_check {
        let all_moves = get_all_possible_moves(
            &board,
            &board.get_next_player_figures(),
            &opponent_moves,
            &moves_by_field,
        );
        let prevent_check_fields =
            get_fields_to_prevent_check(&board, king_position, &opponent_moves, &moves_by_field);
        // either figure is king (we allow all his moves - or figure can prevent check)
        all_moves
            .into_iter()
            .filter(|mov| {
                prevent_check_fields.contains(&mov.to)
                    || mov.from.eq(king_position)
                    || en_passant_to_prevent_check(&board, &mov, &prevent_check_fields)
            })
            .collect()
    } else {
        get_all_possible_takes(&board, board.get_next_player_figures(), &moves_by_field)
    };
    let not_pinned_moves: Vec<PossibleMove> =
        get_not_pinned_pieces(&board, &king_position, moves, &moves_by_field);
    return (not_pinned_moves, is_in_check);
}

pub fn get_valid_moves_in_position(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> (Vec<PossibleMove>, bool) {
    let (king_position, _) = get_own_king(&board);
    // get moves from opponent - we ignore our own king position for rook/bishop/queen to standing on d8, and going to c8 to prevent check from h8
    let opponent_moves: Vec<usize> =
        get_all_threatened_fields(&board, &moves_by_field, &king_position);
    // todo: move this down below check check and pass opponent_moves (no reference)
    let mut moves: Vec<PossibleMove> = get_all_possible_moves(
        &board,
        board.get_next_player_figures(),
        &opponent_moves,
        &moves_by_field,
    );
    // if opponent moves include own king -> we are in check
    let is_in_check = opponent_moves.contains(king_position);
    if is_in_check {
        let prevent_check_fields =
            get_fields_to_prevent_check(&board, king_position, &opponent_moves, &moves_by_field);
        // either figure is king (we allow all his moves - or figure can prevent check)
        moves = moves
            .into_iter()
            .filter(|mov| {
                prevent_check_fields.contains(&mov.to)
                    || mov.from.eq(king_position)
                    || en_passant_to_prevent_check(&board, &mov, &prevent_check_fields)
            })
            .collect()
    }
    let not_pinned_moves: Vec<PossibleMove> =
        get_not_pinned_pieces(&board, &king_position, moves, &moves_by_field);
    return (not_pinned_moves, is_in_check);
}

// get all fields threadned (ignore if opponent figure is on field)
fn get_all_threatened_fields(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    king_position: &usize,
) -> Vec<usize> {
    return board
        .get_opponents()
        .iter()
        .flat_map(|(own_position, figure)| {
            figure.threatened_fields(board, own_position, moves_by_field, &king_position)
        })
        .collect();
}

// default logic get all pseudo legal moves
fn get_all_possible_moves(
    board: &Chessboard,
    figures: &FxHashMap<usize, Figure>,
    opponent_moves: &Vec<usize>,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in figures.iter() {
        val.possible_moves(board, &key, &opponent_moves, &moves_by_field)
            .into_iter()
            .for_each(|single_move| {
                moves.push(PossibleMove {
                    from: key.clone(),
                    to: single_move.to,
                    promoted_to: single_move.promotion,
                })
            });
    }
    moves
}

fn get_all_possible_takes(
    board: &Chessboard,
    figures: &FxHashMap<usize, Figure>,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in figures.iter() {
        val.possible_takes(board, &key, &moves_by_field)
            .into_iter()
            .for_each(|single_move| {
                moves.push(PossibleMove {
                    from: key.clone(),
                    to: single_move.to,
                    promoted_to: single_move.promotion,
                })
            });
    }
    moves
}

fn get_not_pinned_pieces(
    board: &Chessboard,
    king_position: &usize,
    moves: Vec<PossibleMove>,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
) -> Vec<PossibleMove> {
    let pinned_pieces =
        get_pinned_pieces_and_possible_moves(&board, &king_position, &moves_by_field);

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
    prevent_check_fields: &Vec<usize>,
) -> bool {
    // if there is more than one field to prevent check if cannot be from a pawn and prevented by en passant
    if board.en_passant.is_none() || prevent_check_fields.len() > 1 {
        return false;
    }
    // both fields are null checked above
    let checked_by_field = prevent_check_fields.first().unwrap();
    let en_passant_field = board.en_passant.unwrap();
    if checked_by_field != &en_passant_field {
        return false;
    }
    if let Some(figure) = board.get_next_player_figures().get(&mov.from) {
        if !figure.is_pawn() {
            return false;
        }
        return match board.current_move {
            Color::Black => mov.to + 8 == en_passant_field,
            Color::White => mov.to - 8 == en_passant_field,
        };
    }
    return false;
}

fn get_own_king(board: &Chessboard) -> (&usize, &Figure) {
    // if at any point there is no king for the color its prob. better to fail anyways (unwrap)
    board
        .get_next_player_figures()
        .iter()
        .find(|fig| fig.1.is_king())
        .unwrap()
}
