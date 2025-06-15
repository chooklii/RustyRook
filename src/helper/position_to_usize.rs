use regex::Regex;

use crate::{board::promotion::convert_input_string_to_promotion, engine::engine::PossibleMove};

// unchecked and only for opening book 
pub fn get_position_from_input(line: String) -> (usize, usize) {
    let chars: Vec<char> = line.chars().take(4).collect();
    let from_row: String = chars.get(0).unwrap().to_string();
    let from_column: u8 = chars.get(1).unwrap().to_digit(10).unwrap() as u8;
    let to_row: String = chars.get(2).unwrap().to_string();
    let to_column: u8 = chars.get(3).unwrap().to_digit(10).unwrap() as u8;

    (
        get_position_id(&from_row, from_column),
        get_position_id(&to_row, to_column),
    )
}

pub fn get_validated_position_from_input(mov: &str) -> Option<PossibleMove> {
    if let Some((from_row, from_column, to_row, to_column, promoted_to_piece)) =
        validate_string_position(mov)
    {
        let old_field = get_position_id(from_row, from_column);
        let new_field = get_position_id(to_row, to_column);

        let promoted_figure = convert_input_string_to_promotion(promoted_to_piece);
        return Some(PossibleMove {
            from: old_field,
            to: new_field,
            promoted_to: promoted_figure,
        });
    }
    None
}

fn validate_string_position<'a>(
    mov: &'a str,
) -> Option<(&'a str, u8, &'a str, u8, Option<&'a str>)> {
    // first validate that input is in valid format - then split it into x/y for both positions (new and old)
    let valid_move_regex = Regex::new(r"\A[abcdefgh][1-8][abcdefgh][1-8]([qrbkQrbK]?)").unwrap();
    let valid_move = valid_move_regex.captures(mov);

    valid_move.as_ref()?;

    //not beautiful or fast, but not important
    let valid_move_unpacked = valid_move.unwrap().get(1);
    let promoted_to_piece = if !valid_move_unpacked.unwrap().is_empty() {
        Some(valid_move_unpacked.unwrap().as_str())
    } else {
        None
    };

    let split_move_regex = Regex::new(r"((\S)(\S)(\S)(\S))").unwrap();
    let split_moves = split_move_regex.captures(mov).unwrap();
    Some((
        split_moves.get(2).unwrap().as_str(),
        split_moves.get(3).unwrap().as_str().parse::<u8>().unwrap(),
        split_moves.get(4).unwrap().as_str(),
        split_moves.get(5).unwrap().as_str().parse::<u8>().unwrap(),
        promoted_to_piece,
    ))
}

pub fn get_position_id(row: &str, column: u8) -> usize {
    usize::from(get_row_from_string(row) + ((column - 1) * 8) - 1)
}

fn get_row_from_string(row: &str) -> u8 {
    match row {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        _ => 0,
    }
}
