
use crate::engine::transposition::zobrist::{get_transposition_figure_random_numbers,get_transposition_en_passant_numbers};
use board::bitboard::Bitboard;
use board::board::Chessboard;
use dashmap::DashMap;
use engine::{
    count::count_moves,
    engine::search_for_best_move,
    transposition::{transposition::Transposition},
};
use figures::color::Color;
use helper::{
    magic_bitboards::{
        init_with_predefined::{
            init_bishop_magic_arrays, init_bishop_magic_moves_array, init_rook_magic_arrays,
            init_rook_magic_moves_array,
        },
        magic_bitboard::MagicBitboard,
    },
    moves_by_field::{
        get_douplicate_pawn_boards, get_king_moves_for_field, get_knight_moves_for_field,
        get_moves_for_each_field, get_pawn_promotion_moves, get_pawn_takes_for_field,
        MoveInEveryDirection,
    },
};
use lazy_static::lazy_static;
use log::info;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use simple_file_logger::init_logger;
use std::{
    io::{self}, time::SystemTime
};
mod board;
mod engine;
mod evaluation;
mod figures;
mod helper;

static TRANSPOSITION_TABLE: Lazy<DashMap<u64, Transposition>> = Lazy::new(||DashMap::with_capacity(3_200_000));

lazy_static! {
    static ref KNIGHT_MOVES: [Bitboard; 64] = {
        get_knight_moves_for_field()
    };
    static ref KING_MOVES: [Bitboard; 64] = {
        get_king_moves_for_field()
    };
    // by color as well
    static ref PAWN_THREATS: [[Bitboard; 64];2] = {
        get_pawn_takes_for_field()
    };
    // can ignore color as black can never move to 8th row and white to 1st
    static ref PAWN_PROMOTION_FIELDS: Bitboard = {
        get_pawn_promotion_moves()
    };
    // magic bitboards split into magic and possible moves for magic
    static ref BISHOP_MAGIC_BITBOARDS: [MagicBitboard; 64] = {
        init_bishop_magic_arrays()
    };
    static ref BISHOP_MAGIC_POSITIONS: [Vec<Bitboard>; 64] = {
        init_bishop_magic_moves_array()
    };
    static ref ROOK_MAGIC_BITBOARDS: [MagicBitboard; 64] = {
        init_rook_magic_arrays()
    };
    static ref ROOK_MAGIC_POSITIONS: [Vec<Bitboard>; 64] = {
        init_rook_magic_moves_array()
    };
    static ref MOVES_BY_FIELD: FxHashMap<usize, MoveInEveryDirection> = {
        get_moves_for_each_field()
    };
    static ref DOUPLICATE_PAWN_TARIFF: [Bitboard; 8] = {
        get_douplicate_pawn_boards()
    };
    // static u64 to calculate zobrist hash for each color
    static ref ZOBRIST_FIGURE_NUMBERS: [[[u64; 64];6];2] = {
        get_transposition_figure_random_numbers()
    };
    static ref ZOBRIST_SEED: u64 = {
        3847293847293847239
    };
    static ref ZOBRIST_CURRENT_MOVE: u64 = {
        9182739182739182731
    };
    static ref ZOBRIST_EN_PASSANT: [u64; 64] = {
        get_transposition_en_passant_numbers()
    };
    // white short, white long - black short, black long
    static ref ZOBRIST_CASTLE_NUMBERS: [u64;4] = {
        [
            13294823984729384712,
            8473928472384729384,
            1923847192384719238,
            982374928374928374
        ]
    };
}


fn main() {
    init_logger!("rustyRook").unwrap();
    parse_input();
}

fn map_input_to_action(
    commands: Vec<&str>,
    chessboard: &mut Chessboard,
    once_played_positions: &mut Vec<u64>,
    twice_played_positions: &mut Vec<u64>
) {
    let differentiation: &str = commands.first().unwrap_or(&"stop");
    match differentiation {
        "uci" => send_uci_message(),
        "isready" => send_is_ready(),
        "ucinewgame" => init_new_game(),
        "position" => update_board(commands, chessboard, once_played_positions, twice_played_positions),
        "go" => make_move(commands, chessboard, twice_played_positions),
        "debug" => debug_moves(chessboard),
        "quit" => quit(),
        _ => quit(),
    }
}

fn debug_moves(chessboard: &Chessboard) {
    let now = SystemTime::now();
    let max_depth: u8 = 4;
    let moves = count_moves(chessboard, max_depth);
    println!(
        "Moves: {} - Depth: {} - took: {:?}",
        moves,
        max_depth,
        now.elapsed()
    );
}

fn update_board(
    move_vec: Vec<&str>, 
    board: &mut Chessboard,
    once_played_positions: &mut Vec<u64>, 
    twice_played_positions: &mut Vec<u64>) {
    // not beautiful - but also not really important for performance
    board.set_to_default();
    once_played_positions.clear();
    twice_played_positions.clear();
    for single_move in move_vec {
        // ignore both for now - should not be needed as ucinewgame resets game
        if single_move != "position" && single_move != "startpos" && single_move != "moves" {
            board.update_position_from_uci_input(single_move);
            
            // performance does not matter for these few moves
            if !once_played_positions.contains(&board.zobrist_key){
                once_played_positions.push(board.zobrist_key);
            }else if !twice_played_positions.contains(&board.zobrist_key){
                twice_played_positions.push(board.zobrist_key);
            }
        }
    }
}

fn make_move(commands:  Vec<&str>, board: &Chessboard, twice_played_positions: &[u64]) {
    let time_for_move = get_time_for_move(commands, board.current_move);
    let possible_repetition = !twice_played_positions.is_empty();
    search_for_best_move(time_for_move, board, possible_repetition, twice_played_positions);
}

fn get_time_for_move(commands:  Vec<&str>, color: Color) -> u64{
    return match color{
        Color::White => get_time(commands, "wtime", "winc"),
        Color::Black => get_time(commands, "btime", "binc")
    }
}

fn get_time(commands:  Vec<&str>, overall_time_key: &str, increment_key: &str ) -> u64{
    let mut user_time: u64 = 0;

    // given a exact time per move
    let exact_movetime_opt = get_value_from_commands(&commands, "movetime");
    if let Some(exact_movetime) = exact_movetime_opt{
        return exact_movetime - 100; // buffer to send and finish calculation
    }

    let given_time_opt = get_value_from_commands(&commands, overall_time_key);
    // no timelimit -> we take 5s to calculate
    if given_time_opt.is_none(){
        return 5000;
    }
    let given_time = given_time_opt.unwrap();
    
    // if there is an increment calculate average from rest time and add it to time
    let moves_until_increment_opt = get_value_from_commands(&commands, "movestogo");
    if let Some(move_until_increment) = moves_until_increment_opt{
        user_time += given_time / (move_until_increment +2) // +2 to add some buffer for overhead
    }else{
        user_time += given_time / 40 // just make some guess on total count of moves to manage time
    }

    // add by move increment to each calculation
    let increment_opt = get_value_from_commands(&commands, increment_key);
    if let Some(increment) = increment_opt{
        user_time +=increment;
    }
    
    if user_time > 5000{
        // max take 5s, so we dont calculate forever
        return 5000 
    }
    if user_time < 1000{
        // min 1s
        return 1000;
    }
    user_time
}


fn get_value_from_commands(commands:  &Vec<&str>, key: &str) -> Option<u64>{
    let increment_index_opt = commands.iter().position(|x| x.eq(&key));
    if let Some(increment_index) = increment_index_opt{
        if let Some(increment) = commands.get(increment_index+1){
            let value_result = increment.parse();
            if value_result.is_ok(){
                return Some(value_result.unwrap());
            }
        }
    }
    return None;
}

fn quit() {
    panic!("Unknown Command!");
}

fn init_new_game() {
    println!("isready");
}
fn send_is_ready() {
    println!("readyok");
}

fn send_uci_message() {
    println!("id name RustyRook");
    println!("id author Benjamin Zenth");
    println!("uciok");
}

fn init_static_values(){
    let _ = KING_MOVES.get(0);
    let _ = KNIGHT_MOVES.get(0);
    let _ = PAWN_THREATS.get(0);
    let _ = PAWN_PROMOTION_FIELDS.field_is_used(0);
    let _ = DOUPLICATE_PAWN_TARIFF.get(0);
    let _ = ZOBRIST_FIGURE_NUMBERS.get(0);
    // positions are based on magic and impl. init magics
    let _ = BISHOP_MAGIC_POSITIONS[0];
    let _ = ROOK_MAGIC_POSITIONS[0];
}

// recieve input from UCI
fn parse_input() -> String {
    let mut chessboard = Chessboard {
        ..Default::default()
    };
    // Repetition
    let mut once_played_positions: Vec<u64> = Vec::new();
    let mut twice_played_positions: Vec<u64> = Vec::new();
    init_static_values();
    loop {
        let mut buffer_string = String::new();
        io::stdin().read_line(&mut buffer_string).ok().unwrap();
        info!("Recieved Message: {buffer_string}");
        let commands: Vec<&str> = buffer_string.split_whitespace().collect();
        map_input_to_action(commands, &mut chessboard, &mut once_played_positions, &mut twice_played_positions);
    }
}
