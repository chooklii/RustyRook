use rustc_hash::FxHashMap;

use crate::{board::board::Chessboard, helper::moves_by_field::MoveInEveryDirection};

use super::moves::get_valid_moves_in_position;


// used to check if possible moves are still working the way the shoud
pub fn count_moves(board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>, max_depth: u8) -> u64{
    make_moves_and_count_moves(board, moves_by_field, max_depth, 1)
}

fn make_moves_and_count_moves(
    board: &Chessboard,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    max_depth: u8,
    depth: u8,
) -> u64 {
    let mut calculated_positions: u64 = 0;

    let (valid_moves, _) = get_valid_moves_in_position(board, moves_by_field);
    if valid_moves.is_empty() {
        return 0;
    };
    for single in valid_moves.into_iter() {
        let mut new_board = board.clone();
        new_board.move_figure(single.from, single.to, single.promoted_to);

        if depth < max_depth {
            let moves =
                make_moves_and_count_moves(&new_board, moves_by_field, max_depth, depth + 1);

            // Logging for Debug
            if depth == 1 {
                println!(
                    "Move {} - {}- Possible Moves after it {}",
                    single.from, single.to, moves
                );
            }
            calculated_positions += moves;
        } else {
            calculated_positions += 1;
        }
    }

    return calculated_positions;
}