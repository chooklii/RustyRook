use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use crate::{
    board::board::Chessboard,
    evaluation::{evaluate, Evaluation},
    figures::{color::Color, figures::Figure, rock},
    helper::movement::{
        figure_can_move_backward, figure_can_move_backward_and_left, figure_can_move_backward_and_right, figure_can_move_forward, figure_can_move_forward_and_left, figure_can_move_forward_and_right, figure_can_move_left, figure_can_move_right
    },
};

use super::sender::send_move;

#[derive(Debug)]
pub struct PossibleMove {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct MoveWithRating {
    from: usize,
    to: usize,
    rating: Evaluation,
}

pub fn search_for_best_move(board: &Chessboard) {
    let max_depth: u8 = 4;
    let now = SystemTime::now();
    let mut checked_positions: HashSet<String> = HashSet::new();
    if let (Some(best_move), calculations, checked) =
        calculate(board, &mut checked_positions, max_depth, 1)
    {
        println!(
            "Calculated Positions {} and took {:?} - with checks {}",
            calculations,
            now.elapsed(),
            checked
        );
        println!("Best Move Net Rating {:?}", &best_move.rating);
        send_move(&best_move.from, &best_move.to);
    }
}

// Opposite Ray-Directions
fn get_pinned_pieces(board: &Chessboard) -> Vec<usize> {
    let mut pinned_pieces: Vec<usize> = Vec::new();

    if let Some((position, _)) = board
        .get_next_player_figures()
        .iter()
        .find(|x| x.1.is_king())
    {
        let mut figures_pinned_by_rooks = rock_pins(board, position);
        pinned_pieces.append(&mut figures_pinned_by_rooks);

        let mut figures_pinned_by_bishop = bishop_pins(board, position);
        pinned_pieces.append(&mut figures_pinned_by_bishop);
    }

    pinned_pieces
}

fn bishop_pins(board: &Chessboard, king_position: &usize) -> Vec<usize>{
    let mut pieces_pined_by_bishop: Vec<usize> = Vec::new();

    // left back
    if let Some(pinned_piece) =
        get_pinned_piece_by_bishop(board, king_position, figure_can_move_backward_and_left, 9, true)
    {
        pieces_pined_by_bishop.push(pinned_piece);
    }
        // left forward
        if let Some(pinned_piece) =
        get_pinned_piece_by_bishop(board, king_position, figure_can_move_forward_and_left, 7, false)
    {
        pieces_pined_by_bishop.push(pinned_piece);
    }
    // right forward
    if let Some(pinned_piece) =
    get_pinned_piece_by_bishop(board, king_position, figure_can_move_forward_and_right, 9, false)
    {
        pieces_pined_by_bishop.push(pinned_piece);
    }
    // backward
    if let Some(pinned_piece) =
    get_pinned_piece_by_bishop(board, king_position, figure_can_move_backward_and_right, 7, true)
    {
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
    if let Some(opponent_figure_in_line) =
        possible_opponent_bishop_or_queen(board, king_position, movement_check, next_field, backwards)
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

fn rock_pins(board: &Chessboard, king_position: &usize) -> Vec<usize> {
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
        possible_opponent_rock_or_queen(board, king_position, movement_check, next_field, backwards)
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

// check for pins between opponent rock and own king
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
        if position != own_position && position != thread_field{
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

fn possible_opponent_rock_or_queen(
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
        return possible_opponent_rock_or_queen(
            board,
            &field,
            movement_check,
            next_field,
            negative,
        );
    }
    return None;
}

fn calculate(
    board: &Chessboard,
    checked_positions: &mut HashSet<String>,
    max_depth: u8,
    depth: u8,
) -> (Option<MoveWithRating>, u64, u64) {
    // get moves from opponent to check for castle rights
    let opponent_moves: Vec<usize> = get_fields_thread_by_opponent(&board);
    let moves: Vec<PossibleMove> =
        get_all_possible_moves(&board, board.get_next_player_figures(), &opponent_moves);

    let own_pinned_pieces = get_pinned_pieces(board);
    let not_pinned_moves: Vec<&PossibleMove> = moves.iter().filter(|x| !own_pinned_pieces.contains(&x.from)).collect();

    let mut best_move_rating: i16 = init_best_move(&board.current_move);
    let mut best_move: Option<MoveWithRating> = None;
    let mut calculated_positions: u64 = 0;
    let mut checked: u64 = 0;

    for single in not_pinned_moves.iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to);

        let self_in_check = check_if_checked(&new_board);

        // in v1 we just check if the opponent is threadning our king and if so remove this move
        // -> should be improved, as we now calculate positions 3 times for one position
        if self_in_check {
            checked += 1;
        }

        if !self_in_check {
            if depth < max_depth {
                if let (Some(move_evaluation), calculated_moves, calculated_checks) =
                    calculate(&new_board, checked_positions, max_depth, depth + 1)
                {
                    calculated_positions += calculated_moves;
                    checked += calculated_checks;

                    if check_if_is_better_move(
                        &board.current_move,
                        best_move_rating,
                        move_evaluation.rating.net_rating,
                    ) {
                        best_move_rating = move_evaluation.rating.net_rating;
                        best_move = Some(MoveWithRating {
                            from: single.from,
                            to: single.to,
                            rating: move_evaluation.rating,
                        });
                    }
                }
            } else {
                let evaluation = evaluate(&new_board);
                calculated_positions += 1;
                if check_if_is_better_move(
                    &board.current_move,
                    best_move_rating,
                    evaluation.net_rating,
                ) {
                    best_move_rating = evaluation.net_rating;
                    best_move = Some(MoveWithRating {
                        from: single.from,
                        to: single.to,
                        rating: evaluation,
                    });
                }
            }
        }
    }

    return (best_move, calculated_positions, checked);
}

fn init_best_move(turn: &Color) -> i16 {
    match turn {
        Color::White => -30000,
        Color::Black => 30000,
    }
}

fn check_if_is_better_move(turn: &Color, prev: i16, new: i16) -> bool {
    match turn {
        Color::White => new > prev,
        Color::Black => new < prev,
    }
}

fn check_if_position_should_be_calculated(
    board: &Chessboard,
    calculated_positions: &mut HashSet<String>,
) -> bool {
    let position_key = board.position_key();
    if calculated_positions.contains(&position_key) {
        return false;
    }
    calculated_positions.insert(position_key);
    return true;
}

fn check_if_checked(board: &Chessboard) -> bool {
    // somewhat ugly workaround - we have changed the current move at this point this next-player is opponent und "opponents" is us
    let opponent_moves_to: Vec<usize> =
        get_all_possible_moves(&board, board.get_next_player_figures(), &Vec::new())
            .iter()
            .map(|x| x.to)
            .collect();

    if let Some((position, _)) = board
        .get_opponents(&board.current_move)
        .iter()
        .find(|entry| entry.1.is_king())
    {
        // we put ourself in "check" with the move we made (or are still in check after the move)
        return opponent_moves_to.contains(position);
    }
    return false;
}

fn get_fields_thread_by_opponent(board: &Chessboard) -> Vec<usize> {
    get_all_possible_moves(
        &board,
        board.get_opponents(&board.current_move),
        &Vec::new(),
    )
    .iter()
    .map(|x| x.to)
    .collect()
}

fn get_all_possible_moves(
    board: &Chessboard,
    figures: &HashMap<usize, Figure>,
    opponent_moves: &Vec<usize>,
) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in figures.iter() {
        val.possible_moves(board, &key, &opponent_moves)
            .iter()
            .for_each(|single_move| {
                moves.push(PossibleMove {
                    from: key.clone(),
                    to: single_move.clone(),
                })
            });
    }
    moves
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn check_pinned_piece(){

        let mut board = Chessboard{
            ..Default::default()
        };
        // move white queen up on diagonal
        board.move_figure(3, 33);
        // move black pawn forward
        board.move_figure(51, 35);
        // white rock to pin center
        board.move_figure(0, 20);
        // black knight into pin of queen
        board.move_figure(57, 42);
        // white dummy move to give the move to black
        board.move_figure(8, 16);

        let pinned = get_pinned_pieces(&board);
        // e pawn and knight on 42
        assert_eq!(2, pinned.len());
    }
}