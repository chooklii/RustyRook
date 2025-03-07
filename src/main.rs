use std::io::{self};
use board::board::Chessboard;
use engine::engine::search_for_best_move;
use simple_file_logger::init_logger;
use log::info;

mod board;
mod figures;
mod engine;
mod evaluation;


fn main(){
    init_logger!("rustyRook").unwrap();
    parse_input();
}

fn map_input_to_action(commands: Vec<&str>, chessboard: &mut Chessboard){
    let differentiation: &str = commands.first().unwrap_or(&"stop");
    match differentiation {
        "uci" => send_uci_message(),
        "isready" => send_is_ready(),
        "ucinewgame" => init_new_game(),
        "position" => update_board(commands, chessboard),
        "go" => make_move(&chessboard),
        "quit" => quit(),
        _ => quit()
    }
}

fn update_board(move_vec: Vec<&str>, board: &mut Chessboard){
    board.set_to_default();
    for single_move in move_vec{
        // ignore both for now - should not be needed as ucinewgame resets game
        if single_move != "position" && single_move != "startpos" && single_move != "moves" {
            board.make_move(single_move);
        }
    }
}

fn make_move(board: &Chessboard){
    search_for_best_move(&board);
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
    loop{
        let mut buffer_string = String::new();
        io::stdin().read_line(&mut buffer_string).ok().unwrap();
        info!("Recieved Message: {buffer_string}");
        let commands: Vec<&str> = buffer_string.split_whitespace().collect();
        map_input_to_action(commands, &mut chessboard);
    }

}