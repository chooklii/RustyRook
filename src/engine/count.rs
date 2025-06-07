use crate::board::board::Chessboard;

use super::{moves::get_valid_moves_in_position};


// used to check if possible moves are still working the way the shoud
pub fn count_moves(board: &Chessboard,max_depth: u8) -> u64{
    make_moves_and_count_moves(board, max_depth, 1)
}

fn make_moves_and_count_moves(
    board: &Chessboard,
    max_depth: u8,
    depth: u8,
) -> u64 {
    let mut calculated_positions: u64 = 0;

    let (valid_moves, _) = get_valid_moves_in_position(board, true);
    if max_depth == 1{
        // debug
        println!("{:?}", valid_moves);
    }
    if valid_moves.is_empty() {
        return 0;
    };
    for single in valid_moves.into_iter() {
        let mut new_board = *board;
        new_board.move_figure(single.from, single.to, single.promoted_to);
        if depth < max_depth {
            let moves =
                make_moves_and_count_moves(&new_board, max_depth, depth + 1);

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

    calculated_positions
}