use std::{
    collections::HashMap,
    time::SystemTime,
};

use crate::{
    board::board::Chessboard,
    engine::ray::get_pinned_pieces,
    evaluation::{evaluate, Evaluation},
    figures::{color::Color, figures::Figure},
    helper::moves_by_field::MoveInEveryDirection,
};

use super::{checked::get_fields_to_prevent_check, sender::send_move};

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

// used to check if possible moves are still working the way the shoud
pub fn count_moves(board: &Chessboard, moves_by_field: &HashMap<usize, MoveInEveryDirection>){
    let max_depth: u8 = 2;
    let now = SystemTime::now();
    let moves = make_moves_and_count_moves(board, moves_by_field, max_depth, 1);
    println!("Moves: {} - Depth: {} - took: {:?}", moves, max_depth, now.elapsed());
}

fn make_moves_and_count_moves(
    board: &Chessboard,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
    max_depth: u8,
    depth: u8,
) -> u64 {
    let mut calculated_positions: u64 = 0;

    let (valid_moves, _) = get_valid_moves_in_position(board, moves_by_field);
    if valid_moves.is_empty(){
        return 0
    };
    for single in valid_moves.iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to);

        if depth < max_depth {
            let moves = make_moves_and_count_moves(&new_board,moves_by_field, max_depth, depth + 1);

            if depth == 1{
                println!("Move {} - {}- Possible Moves after it {}", single.from, single.to, moves);
            }
            calculated_positions += moves;
        } else {
            calculated_positions += 1;
        }
    }


    return calculated_positions;
}

pub fn search_for_best_move(board: &Chessboard, moves_by_field: &HashMap<usize, MoveInEveryDirection>) {
    let max_depth: u8 = 2;
    let now = SystemTime::now();
    if let (Some(best_move), calculations) = calculate(board, moves_by_field, max_depth, 1)
    {
        println!(
            "Calculated Positions {} and took {:?}",
            calculations,
            now.elapsed(),
        );
        println!("Best Move Net Rating {:?}", &best_move.rating);
        send_move(&best_move.from, &best_move.to);
    }
}

fn get_own_king(board: &Chessboard) -> (&usize, &Figure) {
    // if at any point there is no king for the color its prob. better to fail anyways (unwrap)
    board
        .get_next_player_figures()
        .iter()
        .find(|fig| fig.1.is_king())
        .unwrap()
}

fn get_valid_moves_in_position(board: &Chessboard, moves_by_field: &HashMap<usize, MoveInEveryDirection>) -> (Vec<PossibleMove>, bool) {
    // get moves from opponent
    let opponent_moves: Vec<usize> = get_all_threatened_fields(&board, board.get_opponents(), moves_by_field);
    // todo: move this down below check check and pass opponent_moves (no reference)
    let mut moves: Vec<PossibleMove> =
        get_all_possible_moves(&board, board.get_next_player_figures(), &opponent_moves, &moves_by_field);
    // if opponent moves include own king -> we are in check
    let (king_position, _) = get_own_king(board);

    let is_in_check = opponent_moves.contains(king_position);

    if is_in_check {
        let prevent_check_fields =
            get_fields_to_prevent_check(board, king_position, &opponent_moves, &moves_by_field);
        // either figure is king (we allow all his moves - or figure can prevent check)
        moves = moves
            .into_iter()
            .filter(|mov| prevent_check_fields.contains(&mov.to) || mov.from.eq(king_position))
            .collect()
    }

    let own_pinned_pieces = get_pinned_pieces(board, king_position);
    let not_pinned_moves: Vec<PossibleMove> = moves
        .into_iter()
        .filter(|x| !own_pinned_pieces.contains(&x.from))
        .collect();
    return (not_pinned_moves, is_in_check);
}

fn calculate(
    board: &Chessboard,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
    max_depth: u8,
    depth: u8,
) -> (Option<MoveWithRating>, u64) {
    let mut best_move_rating: i16 = init_best_move(&board.current_move);
    let mut best_move: Option<MoveWithRating> = None;
    let mut calculated_positions: u64 = 0;

    let (valid_moves, is_in_check) = get_valid_moves_in_position(board, moves_by_field);
    if is_in_check && valid_moves.is_empty() {
        // L
        return (
            Some(MoveWithRating {
                from: 0,
                to: 0,
                rating: Evaluation {
                    net_rating: best_move_rating,
                    ..Default::default()
                },
            }),
            1,
        );
    } else if valid_moves.is_empty() && !is_in_check {
        // draw
        return (
            Some(MoveWithRating {
                from: 0,
                to: 0,
                rating: Evaluation {
                    net_rating: 0,
                    ..Default::default()
                },
            }),
            1,
        );
    }

    for single in valid_moves.iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to);

        if depth < max_depth {
            if let (Some(move_evaluation), calculated_moves) =
                calculate(&new_board,moves_by_field, max_depth, depth + 1)
            {
                calculated_positions += calculated_moves;

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
            if check_if_is_better_move(&board.current_move, best_move_rating, evaluation.net_rating)
            {
                best_move_rating = evaluation.net_rating;
                best_move = Some(MoveWithRating {
                    from: single.from,
                    to: single.to,
                    rating: evaluation,
                });
            }
        }
    }

    return (best_move, calculated_positions);
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

// get all fields threadned (ignore if opponent figure is on field)
fn get_all_threatened_fields(board: &Chessboard, figures: &HashMap<usize, Figure>, moves_by_field: &HashMap<usize, MoveInEveryDirection>) -> Vec<usize> {
    return board.get_opponents()
        .iter()
        .flat_map(|(own_position, figure)| figure.threatened_fields(board, own_position, moves_by_field))
        .collect();
}

// default logic get all pseudo legal moves
fn get_all_possible_moves(
    board: &Chessboard,
    figures: &HashMap<usize, Figure>,
    opponent_moves: &Vec<usize>,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>
) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in figures.iter() {
        val.possible_moves(board, &key, &opponent_moves, &moves_by_field)
            .into_iter()
            .for_each(|single_move| {
                moves.push(PossibleMove {
                    from: key.clone(),
                    to: single_move,
                })
            });
    }
    moves
}
