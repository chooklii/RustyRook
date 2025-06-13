use std::{
    fs::File,
    io::{self, BufRead},
    path::Path
};

use dashmap::DashMap;

use crate::board::board::Chessboard;

#[derive(Debug, Clone, Copy)]
pub struct OpeningMove {
    pub from: usize,
    pub to: usize,
}

pub fn create_opening_map() -> DashMap<u64, Vec<OpeningMove>> {
    let openings = DashMap::new();
    if let Ok(lines) = read_lines("./openings.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_key = 0;

        for line in lines.map_while(Result::ok) {
            // indicating new position
            if line.contains("pos") {
                let mut board = Chessboard {..Default::default()};
                // not really performant, but it is not important here in static map
                let fen: String = line.chars().skip(4).collect();
                board.create_position_from_input_string(fen.clone());
                current_key = board.zobrist_key;
                openings.insert(current_key, Vec::new());
            } else {
                let (from, to) = get_position_from_input(line);
                openings.entry(current_key).or_insert(Vec::new()).push(OpeningMove { from, to });
            }
        }
    } else {
        println!("Failed to Load Openings Book!");
    }
    openings
}
// All not beautiful - But works for now
fn get_position_from_input(line: String) -> (usize, usize) {
    let chars: Vec<char> = line.chars().take(4).collect();
    let from_row: String = chars.get(0).unwrap().to_string();
    let from_column: u8 = chars.get(1).unwrap().to_digit(10).unwrap() as u8;
    let to_row: String = chars.get(2).unwrap().to_string();
    let to_column: u8 = chars.get(3).unwrap().to_digit(10).unwrap() as u8;

    (get_position_id(&from_row, from_column), get_position_id(&to_row, to_column)) 
}

fn get_position_id(row: &str, column: u8) -> usize {
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


// taken from Rust Book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
