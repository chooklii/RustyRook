use log::info;

use crate::board::promotion::{convert_promotion_to_output_string, Promotion};

fn get_row_from_number(row: usize) -> String {
    match row % 8 {
        0 => String::from("a"),
        1 => String::from("b"),
        2 => String::from("c"),
        3 => String::from("d"),
        4 => String::from("e"),
        5 => String::from("f"),
        6 => String::from("g"),
        _ => String::from("h"),
    }
}

fn convert_number_to_chess_notation(position: usize) -> String {
    let mut row = get_row_from_number(position).to_owned();
    let column = position / 8 + 1;

    row.push_str(&column.to_string());
    row
}

pub fn send_move(old_position: usize, new_position: usize, promoted_to: Option<Promotion>) {
    let old_field = convert_number_to_chess_notation(old_position);
    let new_field = convert_number_to_chess_notation(new_position);
    let promoted_to = convert_promotion_to_output_string(promoted_to);

    info!("Found best Move was: {}{}{}", old_field, new_field, promoted_to);
    println!("bestmove {}{}{}", old_field, new_field, promoted_to);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_convert() {
        assert_eq!("a1", convert_number_to_chess_notation(0));
        assert_eq!("c1", convert_number_to_chess_notation(2));
        assert_eq!("a8", convert_number_to_chess_notation(56));
        assert_eq!("h8", convert_number_to_chess_notation(63));
        assert_eq!("g5", convert_number_to_chess_notation(38));
        assert_eq!("h3", convert_number_to_chess_notation(23));
        assert_eq!("c8", convert_number_to_chess_notation(58));
        assert_eq!("d4", convert_number_to_chess_notation(27));
    }
}
