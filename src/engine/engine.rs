use crate::board::board::Chessboard;

struct PossibleMove {
    from: usize,
    to: usize,
}

pub fn calculate_move(board: &Chessboard) {
    let moves: Vec<PossibleMove> = get_all_possible_moves(&board);       

    let movetobemade = moves.first().unwrap();

    send_move(&movetobemade.from, &movetobemade.to);
}

fn get_row_from_number(row: &usize) -> &str {
    return match row % 8 {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        _ => "h",
    };
}

fn convert_number_to_chess_notation(position: &usize) -> String {
    let mut row = get_row_from_number(position).to_owned();
    let column = position / 8 + 1;

    row.push_str(&column.to_string());
    row
}

fn send_move(old_position: &usize, new_position: &usize) {
    let old_field = convert_number_to_chess_notation(old_position);
    let new_field = convert_number_to_chess_notation(new_position);

    println! {"bestmove {}{}", old_field, new_field};
}

fn get_all_possible_moves(board: &Chessboard) -> Vec<PossibleMove> {
    let mut moves = Vec::new();
    for (key, val) in board.get_next_player_figures().iter() {
        val.possible_moves(board, &key).iter().for_each(|single_move| {
            moves.push(PossibleMove {
                from: key.clone(),
                to: single_move.clone(),
            })
        });
    }
    moves
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_convert() {
        assert_eq!("a1", convert_number_to_chess_notation(&0));
        assert_eq!("c1", convert_number_to_chess_notation(&2));
        assert_eq!("a8", convert_number_to_chess_notation(&56));
        assert_eq!("h8", convert_number_to_chess_notation(&63));
        assert_eq!("g5", convert_number_to_chess_notation(&38));
        assert_eq!("h3", convert_number_to_chess_notation(&23));
        assert_eq!("c8", convert_number_to_chess_notation(&58));
        assert_eq!("d4", convert_number_to_chess_notation(&27));
    }
}
