use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use crate::{
    board::board::Chessboard,
    evaluation::{evaluate, Evaluation},
    figures::{color::Color, figures::Figure},
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
    if let (Some(best_move), calculations, checked) = calculate(
        board,
        &mut checked_positions,
        &mut -30000,
        &mut 30000,
        max_depth,
        1,
    ) {
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

fn calculate(
    board: &Chessboard,
    checked_positions: &mut HashSet<String>,
    alpha: &mut i16,
    beta: &mut i16,
    max_depth: u8,
    depth: u8,
) -> (Option<MoveWithRating>, u64, u64) {
    // get moves from opponent to check for castle rights
    let opponent_moves: Vec<usize> = get_fields_thread_by_opponent(&board);
    let moves: Vec<PossibleMove> =
        get_all_possible_moves(&board, board.get_next_player_figures(), &opponent_moves);

    let mut best_move_rating: i16 = init_best_move(&board.current_move);
    let mut best_move: Option<MoveWithRating> = None;
    let mut calculated_positions: u64 = 0;
    let mut checked: u64 = 0;

    for single in moves.iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to);

        let self_in_check = check_if_checked(&new_board);

        // in v1 we just check if the opponent is threadning our king and if so remove this move
        // -> should be improved, as we now calculate positions 3 times for one position
        if self_in_check {
            checked += 1;
        }

        if depth < max_depth {
            if let (Some(move_evaluation), calculated_moves, calculated_checks) = calculate(
                &new_board,
                checked_positions,
                alpha,
                beta,
                max_depth,
                depth + 1,
            ) {
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
    let opponent_moves_to: Vec<usize> = get_all_possible_moves(
        &board,
        board.get_opponents(&board.current_move),
        &Vec::new(),
    )
    .iter()
    .map(|x| x.to)
    .collect();

    if let Some((position, _)) = board
        .get_next_player_figures()
        .iter()
        .find(|entry| entry.1.is_king())
    {
        // we put ourself in "check" with the move we made
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
