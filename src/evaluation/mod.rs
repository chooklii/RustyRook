use rustc_hash::FxHashMap;

use crate::{board::board::Chessboard, figures::{bishop::{self, Bishop}, figures::Figure, rook::Rook}, helper::moves_by_field::MoveInEveryDirection};

#[derive(PartialEq, PartialOrd, Clone, Debug, Copy)]
pub struct Evaluation {
    pub white_pieces_value: f32,
    pub black_pieces_value: f32,
    pub net_rating: f32,
}

impl Default for Evaluation {
    fn default() -> Evaluation {
        Evaluation {
            white_pieces_value: 0.0,
            black_pieces_value: 0.0,
            net_rating: 0.0,
        }
    }
}

// a1 to h8
const PAWN_RATE: [f32; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.8, 0.8, 1.0, 1.1, 1.1, 1.0, 0.8, 0.8, 
    0.9, 0.8, 1.2, 1.3, 1.3, 1.2, 0.8, 0.9, 
    1.0, 1.0, 1.2, 1.3, 1.3, 1.2, 1.0, 1.0, 
    1.0, 1.0, 1.1, 1.1, 1.1, 1.1, 1.0, 1.0, 
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 
    9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 
];

const KNIGHT_RATE: [f32; 64] = [
    1.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 1.5, 
    2.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 2.0, 
    2.0, 3.1, 3.2, 3.2, 3.2, 3.2, 3.1, 2.0, 
    2.0, 3.0, 3.5, 3.5, 3.5, 3.5, 3.0, 2.0, 
    2.0, 3.0, 3.5, 3.5, 3.5, 3.5, 3.0, 2.0, 
    2.0, 3.1, 3.2, 3.2, 3.2, 3.2, 3.1, 2.0, 
    2.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 2.0, 
    1.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 1.5,
];

const ROOK_RATE: [f32; 64] = [
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
];

const BISHOP_RATE: [f32; 64] = [
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 
    3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 
    3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 
    3.0, 3.2, 3.0, 3.0, 3.0, 3.0, 3.2, 3.0, 
    3.0, 3.0, 3.2, 3.0, 3.0, 3.2, 3.0, 3.0, 
    3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 
    3.0, 3.2, 3.0, 3.0, 3.0, 3.0, 3.2, 3.0, 
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
];

const QUEEN_RATE: [f32; 64] = [
    8.6, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 8.6, 
    8.6, 9.0, 9.2, 9.2, 9.2, 9.2, 9.0, 8.6, 
    8.7, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.7, 
    8.7, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.7, 
    8.7, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.7, 
    8.7, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.7, 
    8.6, 9.0, 9.2, 9.2, 9.2, 9.2, 9.0, 8.6, 
    8.6, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 8.6,
];

const EARLY_GAME_KING_RATE: [f32; 64] = [
    1.0, 1.3, 1.1, 1.0, 1.0, 1.1, 1.3, 1.2,
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
];

const LATE_GAME_KING_RATE: [f32; 64] = [
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
    1.0, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.0, 
    1.0, 1.1, 1.3, 1.3, 1.3, 1.3, 1.1, 1.0, 
    1.0, 1.1, 1.3, 1.5, 1.5, 1.3, 1.1, 1.0, 
    1.0, 1.1, 1.3, 1.5, 1.5, 1.3, 1.1, 1.0, 
    1.0, 1.1, 1.3, 1.3, 1.3, 1.3, 1.1, 1.0, 
    1.0, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.0, 
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 
];

fn get_king_weight(position: usize, pieces: f32) -> f32 {
    // if there is less then 10 pieces value left activate king
    if pieces >=10.0{
        return EARLY_GAME_KING_RATE[position];
    }
    return LATE_GAME_KING_RATE[position];
}

fn get_rook_weight(rook: &Rook, position: usize, board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> f32{
    return ROOK_RATE[position];
}

fn get_bishop_weight(bishop: &Bishop, position: usize, board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> f32{
    return BISHOP_RATE[position];
}

fn get_figure_position_weight(figure: &Figure, position: usize, board: &Chessboard, pieces: f32, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> f32 {
    return match figure {
        Figure::Pawn(_) => PAWN_RATE[position],
        Figure::Knight(_) => KNIGHT_RATE[position],
        Figure::Bishop(bishop) => get_bishop_weight(&bishop, position, &board, &moves_by_field),
        Figure::Rook(rook) => get_rook_weight(&rook, position, &board, &moves_by_field),
        Figure::Queen(_) => QUEEN_RATE[position],
        Figure::King(_) => get_king_weight(position, pieces),
    };
}

fn get_figure_weight(figure: &Figure) -> f32 {
    return match figure {
        Figure::Pawn(_) => 1.0,
        Figure::Knight(_) => 3.0,
        Figure::Bishop(_) => 3.2,
        Figure::Rook(_) => 5.0,
        Figure::Queen(_) => 9.0,
        Figure::King(_) => 10.0,
    };
}

pub fn evaluate(board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> Evaluation {
    let white_pieces_value: f32 = board
        .white_figures
        .iter()
        .map(|(_, figure)| get_figure_weight(figure))
        .sum();

    let white_pieces_position_value: f32 = board
        .white_figures
        .iter()
        .map(|(&position, figure)| get_figure_position_weight(&figure, position, &board, white_pieces_value, &moves_by_field))
        .sum();

    let black_pieces_value: f32 = board
        .black_figures
        .iter()
        .map(|(_, figure)| get_figure_weight(figure))
        // invert black position to use same values for both colors
        .sum();

    let black_pieces_position_value: f32 = board
        .black_figures
        .iter()
        .map(|(&position, figure)| get_figure_position_weight(&figure, 63 - position, &board, black_pieces_value, &moves_by_field))
        // invert black position to use same values for both colors
        .sum();


    let white_value = white_pieces_value + white_pieces_position_value;
    let black_value = black_pieces_value + black_pieces_position_value;

    Evaluation {
        white_pieces_value: white_value,
        black_pieces_value: black_value,
        net_rating: white_value - black_value,
    }
}
