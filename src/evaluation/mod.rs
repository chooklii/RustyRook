use std::usize;

use rustc_hash::FxHashMap;

use crate::{board::board::Chessboard, figures::{bishop::Bishop, figures::Figure, queen::Queen, rook::Rook}, helper::moves_by_field::MoveInEveryDirection};

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
const PAWN_RATE_KING_CENTER: [f32; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.8, 0.8, 1.0, 1.1, 1.1, 1.0, 0.8, 0.8, 
    0.9, 0.8, 1.2, 1.3, 1.3, 1.2, 0.8, 0.9, 
    1.0, 1.0, 1.2, 1.3, 1.3, 1.2, 1.0, 1.0, 
    1.0, 1.0, 1.1, 1.1, 1.1, 1.1, 1.0, 1.0, 
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 
    9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 
];

const PAWN_RATE_KING_LEFT: [f32; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.7, 0.7, 0.7, 1.1, 1.1, 1.1, 1.1, 1.1, 
    0.7, 0.7, 0.7, 1.3, 1.3, 1.3, 1.3, 1.3, 
    1.0, 1.0, 1.2, 1.3, 1.3, 1.3, 1.3, 1.3, 
    1.0, 1.0, 1.1, 1.1, 1.1, 1.3, 1.3, 1.3, 
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.5, 2.5, 
    9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 
];

const PAWN_RATE_KING_RIGHT: [f32; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.1, 1.1, 1.1, 1.1, 1.1, 0.7, 0.7, 0.7, 
    1.3, 1.3, 1.3, 1.3, 1.3, 1.7, 0.7, 0.7, 
    1.3, 1.3, 1.3, 1.3, 1.3, 1.2, 1.0, 1.0, 
    1.3, 1.3, 1.3, 1.1, 1.1, 1.1, 1.0, 1.0, 
    2.5, 2.5, 2.5, 2.0, 2.0, 2.0, 2.0, 2.0, 
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

enum KingPosition{
    CENTER,
    LEFT,
    RIGHT
}

fn get_king_position(figures: &FxHashMap<usize, Figure>) -> usize{
    let (&king_position, _) = figures.iter()
                                            .find(|(_, fig)| fig.is_king())
                                            // if we have no king we have other problems :-D
                                            .unwrap();
    return king_position;
}

fn check_where_king_is_located(king_position: &usize, is_black_king: bool) -> KingPosition{
    if is_black_king{
        return match king_position % 8{
            0 | 1 | 2 => KingPosition::RIGHT,
            5 | 6 | 7 => KingPosition::LEFT,
            _ => KingPosition::CENTER
        }
    }                                    
    match king_position % 8{
        0 | 1 | 2 => KingPosition::LEFT,
        5 | 6 | 7 => KingPosition::RIGHT,
        _ => KingPosition::CENTER
    }
}

fn get_king_weight(position: usize, pieces: f32) -> f32 {
    // if there is less then 10 pieces value left activate king
    if pieces >=10.0{
        return EARLY_GAME_KING_RATE[position];
    }
    return LATE_GAME_KING_RATE[position];
}

fn get_pawn_rate(position: usize, king_position: &KingPosition) -> f32{
    // push pawns on side where out king is not!
    match king_position{
        KingPosition::CENTER => PAWN_RATE_KING_CENTER[position],
        KingPosition::LEFT => PAWN_RATE_KING_LEFT[position],
        KingPosition::RIGHT => PAWN_RATE_KING_RIGHT[position]
    }
}

fn get_rook_weight(rook: &Rook, position: usize, board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> f32{
    let threatened_fields = rook.threatened_fields(&board, &position, &moves_by_field, &64);
    let threat_bonus = threatened_fields.len() as f32 * 0.02;
    return ROOK_RATE[position] + threat_bonus;
}

fn get_bishop_weight(bishop: &Bishop, position: usize, board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> f32{
    let threatened_fields = bishop.threatened_fields(&board, &position, &moves_by_field, &64);
    let threat_bonus = threatened_fields.len() as f32 * 0.02;
    return BISHOP_RATE[position] + threat_bonus;
}

fn get_queen_weight(queen: &Queen, position: usize, board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> f32{
    let threatened_fields = queen.threatened_fields(&board, &position, &moves_by_field, &64);
    let threat_bonus = threatened_fields.len() as f32 * 0.02;
    return QUEEN_RATE[position] + threat_bonus;
}

fn get_figure_position_weight(figure: &Figure, position: usize, board: &Chessboard, king_position: &KingPosition, pieces: f32, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> f32 {
    return match figure {
        Figure::Pawn(_) => get_pawn_rate(position, &king_position),
        Figure::Knight(_) => KNIGHT_RATE[position],
        Figure::Bishop(bishop) => get_bishop_weight(&bishop, position, &board, &moves_by_field),
        Figure::Rook(rook) => get_rook_weight(&rook, position, &board, &moves_by_field),
        Figure::Queen(queen) => get_queen_weight(&queen, position, &board, &moves_by_field),
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

fn get_opponent_king_bonus(piece_value: f32, king_position: &usize) -> f32{
    if piece_value > 15.0{
        return 0.0
    }
    let king_column = king_position % 8 ;
    if king_position > &55 || king_position < &8 || king_column == 0 || king_column == 7{
        return 2.0;
    };
    if king_position > &47 || king_position < &15 || king_column == 1 || king_column == 6{
        return 1.0;
    }
    return 0.0;
}

pub fn evaluate(board: &Chessboard, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> Evaluation {
    let white_pieces_value: f32 = board
        .white_figures
        .iter()
        .map(|(_, figure)| get_figure_weight(figure))
        .sum();

    let white_king_usize =  get_king_position(&board.white_figures);
    let black_king_usize = get_king_position(&board.black_figures);

    let white_king_position = check_where_king_is_located(&white_king_usize, false);
    let black_king_position = check_where_king_is_located(&black_king_usize, true);

    let white_pieces_position_value: f32 = board
        .white_figures
        .iter()
        .map(|(&position, figure)| get_figure_position_weight(&figure, position, &board, &white_king_position, white_pieces_value, &moves_by_field))
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
        .map(|(&position, figure)| get_figure_position_weight(&figure, 63 - position,&board, &black_king_position, black_pieces_value, &moves_by_field))
        // invert black position to use same values for both colors
        .sum();


    // in the endgame push opponent king to the edge of the board
    let white_opponent_king_bonus = get_opponent_king_bonus(black_pieces_value, &black_king_usize);
    let black_opponent_king_bonus = get_opponent_king_bonus(white_pieces_value, &white_king_usize);


    let white_value = white_pieces_value + white_pieces_position_value + white_opponent_king_bonus;
    let black_value = black_pieces_value + black_pieces_position_value + black_opponent_king_bonus;

    Evaluation {
        white_pieces_value: white_value,
        black_pieces_value: black_value,
        net_rating: white_value - black_value,
    }
}
