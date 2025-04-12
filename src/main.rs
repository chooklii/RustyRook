use std::{io::{self}, time::SystemTime};
use board::board::Chessboard;
use engine::{count::count_moves, engine::search_for_best_move};
use rustc_hash::FxHashMap;
use simple_file_logger::init_logger;
use log::info;
use helper::moves_by_field::{get_moves_for_each_field, MoveInEveryDirection};

mod board;
mod figures;
mod engine;
mod helper;
mod evaluation;


fn main(){
    init_logger!("rustyRook").unwrap();
    parse_input();
}

fn map_input_to_action(commands: Vec<&str>, chessboard: &mut Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>){
    let differentiation: &str = commands.first().unwrap_or(&"stop");
    match differentiation {
        "uci" => send_uci_message(),
        "isready" => send_is_ready(),
        "ucinewgame" => init_new_game(),
        "position" => update_board(commands, chessboard),
        "go" => make_move(&chessboard, &moves_by_field),
        "debug" => debug_moves(&chessboard, moves_by_field,),
        "quit" => quit(),
        _ => quit()
    }
}

fn debug_moves(chessboard: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>){
    let now = SystemTime::now();
    let max_depth: u8 = 4;
    let moves = count_moves(&chessboard, moves_by_field, max_depth);
    println!(
        "Moves: {} - Depth: {} - took: {:?}",
        moves,
        max_depth,
        now.elapsed()
    );
}

fn update_board(move_vec: Vec<&str>, board: &mut Chessboard){
    board.set_to_default();
    for single_move in move_vec{
        // ignore both for now - should not be needed as ucinewgame resets game
        if single_move != "position" && single_move != "startpos" && single_move != "moves" {
            board.update_position_from_uci_input(single_move);
        }
    }
}

fn make_move(board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>){
    search_for_best_move(&board, &moves_by_field);
}

fn quit(){
    panic!("Unknown!");
}

fn init_new_game(){
    println!("isready");
}
fn send_is_ready(){
    println!("readyok");
}

fn send_uci_message(){
    println!("id name RustyRook");
    println!("id author Benjamin Zenth");
    println!("uciok");
}

// recieve input from UCI
fn parse_input() -> String{
    let mut chessboard = Chessboard{..Default::default()};
    // can prob be made static - but in some way is static :D
    let moves_by_field = get_moves_for_each_field();
    loop{
        let mut buffer_string = String::new();
        io::stdin().read_line(&mut buffer_string).ok().unwrap();
        info!("Recieved Message: {buffer_string}");
        let commands: Vec<&str> = buffer_string.split_whitespace().collect();
        map_input_to_action(commands, &mut chessboard, &moves_by_field);
    }
}