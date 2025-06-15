use std::{
    env, fs::File, io::{self, BufRead}, path::Path
};

use dashmap::DashMap;
use log::info;

use crate::{board::board::Chessboard, helper::position_to_usize::get_position_from_input};

#[derive(Debug, Clone, Copy)]
pub struct OpeningMove {
    pub from: usize,
    pub to: usize,
}

// book taken from https://github.com/SebLague/Chess-Coding-Adventure :-)
// initial book had issues with missing en passant fields, was updated for the most played positions
pub fn create_opening_map() -> DashMap<u64, Vec<OpeningMove>> {
    let openings = DashMap::new();
    if let Ok(lines) = read_lines("./openings.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut board = Chessboard {..Default::default()};
        for line in lines.map_while(Result::ok) {
            // indicating new position
            if line.contains("pos") {
                // not really performant, but it is not important here in static map
                let fen: String = line.chars().skip(4).collect();
                board.create_position_from_input_string(fen);
                openings.insert(board.zobrist_key, Vec::new());
            } else {
                let (from, to) = get_position_from_input(line);
                openings.entry(board.zobrist_key).or_insert(Vec::new()).push(OpeningMove { from, to });
            }
        }
    } else {
        info!("Failed to load opening book at path {:?}", env::current_dir());
        panic!("Failed to load opening book - make sure to place the 'openings.txt' file at {:?}", env::current_dir());
    }
    openings
}



// taken from Rust Book
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
