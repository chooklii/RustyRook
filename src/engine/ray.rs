use crate::{
    board::board::Chessboard,
    helper::movement::{
        figure_can_move_backward, figure_can_move_backward_and_left,
        figure_can_move_backward_and_right, figure_can_move_forward,
        figure_can_move_forward_and_left, figure_can_move_forward_and_right, figure_can_move_left,
        figure_can_move_right,
    },
};

// File Contains Logic to check for possible pinned Figures

// Opposite Ray-Directions
pub fn get_pinned_pieces(board: &Chessboard, king_position: &usize) -> Vec<usize> {
    let mut pinned_pieces: Vec<usize> = Vec::new();

    let mut figures_pinned_by_rooks = rook_pins(board, king_position);
    pinned_pieces.append(&mut figures_pinned_by_rooks);

    let mut figures_pinned_by_bishop = bishop_pins(board, king_position);
    pinned_pieces.append(&mut figures_pinned_by_bishop);

    pinned_pieces
}

fn bishop_pins(board: &Chessboard, king_position: &usize) -> Vec<usize> {
    let mut pieces_pined_by_bishop: Vec<usize> = Vec::new();

    // left back
    if let Some(pinned_piece) = get_pinned_piece_by_bishop(
        board,
        king_position,
        figure_can_move_backward_and_left,
        9,
        true,
    ) {
        pieces_pined_by_bishop.push(pinned_piece);
    }
    // left forward
    if let Some(pinned_piece) = get_pinned_piece_by_bishop(
        board,
        king_position,
        figure_can_move_forward_and_left,
        7,
        false,
    ) {
        pieces_pined_by_bishop.push(pinned_piece);
    }
    // right forward
    if let Some(pinned_piece) = get_pinned_piece_by_bishop(
        board,
        king_position,
        figure_can_move_forward_and_right,
        9,
        false,
    ) {
        pieces_pined_by_bishop.push(pinned_piece);
    }
    // backward
    if let Some(pinned_piece) = get_pinned_piece_by_bishop(
        board,
        king_position,
        figure_can_move_backward_and_right,
        7,
        true,
    ) {
        pieces_pined_by_bishop.push(pinned_piece);
    }

    pieces_pined_by_bishop
}

fn get_pinned_piece_by_bishop(
    board: &Chessboard,
    king_position: &usize,
    movement_check: fn(&usize) -> bool,
    next_field: usize,
    backwards: bool,
) -> Option<usize> {
    if let Some(opponent_figure_in_line) = possible_opponent_bishop_or_queen(
        board,
        king_position,
        movement_check,
        next_field,
        backwards,
    ) {
        return get_possible_pinned_piece(
            board,
            opponent_figure_in_line,
            // todo
            *king_position,
            next_field,
            backwards,
        );
    }
    return None;
}

fn possible_opponent_bishop_or_queen(
    board: &Chessboard,
    field_position: &usize,
    movement_check: fn(&usize) -> bool,
    next_field: usize,
    negative: bool,
) -> Option<usize> {
    if movement_check(field_position) {
        let field = if negative {
            field_position - next_field
        } else {
            field_position + next_field
        };
        if let Some(opponent_figure) = board.get_opponents(&board.current_move).get(&field) {
            if opponent_figure.is_queen() || opponent_figure.is_bishop() {
                return Some(field);
            }
            return None;
        }
        return possible_opponent_bishop_or_queen(
            board,
            &field,
            movement_check,
            next_field,
            negative,
        );
    }
    return None;
}

fn rook_pins(board: &Chessboard, king_position: &usize) -> Vec<usize> {
    let mut pieces_pined_by_rook: Vec<usize> = Vec::new();

    // left
    if let Some(pinned_piece) =
        get_pinned_piece_by_rook(board, king_position, figure_can_move_left, 1, true)
    {
        pieces_pined_by_rook.push(pinned_piece);
    }
    // right
    if let Some(pinned_piece) =
        get_pinned_piece_by_rook(board, king_position, figure_can_move_right, 1, false)
    {
        pieces_pined_by_rook.push(pinned_piece);
    }
    // forward
    if let Some(pinned_piece) =
        get_pinned_piece_by_rook(board, king_position, figure_can_move_forward, 8, false)
    {
        pieces_pined_by_rook.push(pinned_piece);
    }
    // backward
    if let Some(pinned_piece) =
        get_pinned_piece_by_rook(board, king_position, figure_can_move_backward, 8, true)
    {
        pieces_pined_by_rook.push(pinned_piece);
    }
    pieces_pined_by_rook
}

fn get_pinned_piece_by_rook(
    board: &Chessboard,
    king_position: &usize,
    movement_check: fn(&usize) -> bool,
    next_field: usize,
    backwards: bool,
) -> Option<usize> {
    if let Some(opponent_figure_in_line) =
        possible_opponent_rook_or_queen(board, king_position, movement_check, next_field, backwards)
    {
        return get_possible_pinned_piece(
            board,
            opponent_figure_in_line,
            // todo
            *king_position,
            next_field,
            backwards,
        );
    }
    return None;
}

// check for pins between opponent rook and own king
// one opponent piece === no pin
// more than one own piece == no pin
// one own piece = pin
fn get_possible_pinned_piece(
    board: &Chessboard,
    thread_field: usize,
    own_position: usize,
    step: usize,
    backwards: bool,
) -> Option<usize> {
    // get all positions to check
    let positions_to_check: Vec<usize> = if backwards {
        (thread_field..=own_position).step_by(step).collect()
    } else {
        (own_position..=thread_field).step_by(step).collect()
    };
    let mut possible_pinned_piece: Option<usize> = None;

    for position in positions_to_check {
        // need to ignore own field and thread field
        if position != own_position && position != thread_field {
            if board.positions.get(position) {
                if board
                    .get_opponents(&board.current_move)
                    .contains_key(&position)
                {
                    return None;
                }
                // field is used - but it is no opponent so it has to be us!
                // if there is already a possible pinned piece = no peace is pinned, as there are two between
                if possible_pinned_piece.is_some() {
                    return None;
                }
                possible_pinned_piece = Some(position);
            }
        }
    }
    return possible_pinned_piece;
}

fn possible_opponent_rook_or_queen(
    board: &Chessboard,
    field_position: &usize,
    movement_check: fn(&usize) -> bool,
    next_field: usize,
    negative: bool,
) -> Option<usize> {
    if movement_check(field_position) {
        let field = if negative {
            field_position - next_field
        } else {
            field_position + next_field
        };
        if let Some(opponent_figure) = board.get_opponents(&board.current_move).get(&field) {
            if opponent_figure.is_queen() || opponent_figure.is_rook() {
                return Some(field);
            }
        }
        return possible_opponent_rook_or_queen(
            board,
            &field,
            movement_check,
            next_field,
            negative,
        );
    }
    return None;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_pinned_piece() {
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

        let pinned = get_pinned_pieces(&board, &60);
        // e pawn and knight on 42
        assert_eq!(2, pinned.len());
    }
}
