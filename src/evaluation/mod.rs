use std::usize;

use log::info;

use crate::{board::board::Chessboard, figures::{color::Color, piece::Piece, sliding_moves::{get_fields_threatened_by_bishop, get_fields_threatened_by_queen, get_fields_threatened_by_rook}}, helper::moves_by_field::MoveInEveryDirection, DOUPLICATE_PAWN_TARIFF};

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

#[derive(PartialEq, PartialOrd, Clone, Debug, Copy)]
enum KingPosition{
    CENTER,
    LEFT,
    RIGHT
}


fn check_where_king_is_located(king_position: usize, is_black_king: bool) -> KingPosition{
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
    if pieces >=15.0{
        return EARLY_GAME_KING_RATE[position];
    }
    return LATE_GAME_KING_RATE[position];
}

fn get_pawn_rate(position: usize, color: Color, king_position: &KingPosition) -> f32{
    let adjusted_position = match color{
        Color::White => position,
        Color::Black => 63 - position    
    };
    // push pawns on side where out king is not!
    match king_position{
        KingPosition::CENTER => PAWN_RATE_KING_CENTER[adjusted_position],
        KingPosition::LEFT => PAWN_RATE_KING_LEFT[adjusted_position],
        KingPosition::RIGHT => PAWN_RATE_KING_RIGHT[adjusted_position]
    }
}

fn get_rook_weight(position: usize, board: &Chessboard, king_position: usize) -> f32{
    let threatened_fields = get_fields_threatened_by_rook(&board, position, king_position);
    let mut threat_bonus = ROOK_RATE[position];
    threat_bonus = threat_bonus+ 0.02 * threatened_fields.board.count_ones() as f32;
    return threat_bonus;
}

fn get_bishop_weight( position: usize, board: &Chessboard, color: Color, king_position: usize) -> f32{
    let adjusted_position = match color{
        Color::White => position,
        Color::Black => 63 - position    
    };
    let threatened_fields = get_fields_threatened_by_bishop(&board, position, king_position);
    let mut weight = BISHOP_RATE[adjusted_position];
    weight = weight+0.02*threatened_fields.board.count_ones() as f32;
    return weight;
}

fn get_queen_weight(position: usize, board: &Chessboard, king_position: usize) -> f32{
    let threatened_fields = get_fields_threatened_by_queen(&board, position, king_position);
    let mut weight = QUEEN_RATE[position];
    weight = weight + 0.02 * threatened_fields.board.count_ones() as f32;
    return weight;
}

fn get_position_weight(board: &Chessboard, color: Color, king_position: &KingPosition, pieces: f32, king_usize: usize) -> f32 {
    let mut score: f32 = 0.0;
    board.get_pieces(color, Piece::Pawn).iterate_board(|position| score+=get_pawn_rate(position, color, &king_position));
    board.get_pieces(color, Piece::Knight).iterate_board(|position| score += KNIGHT_RATE[position]);
    if king_usize == 64{
        println!("{:?}", board.played_moves);
    }
    board.get_pieces(color, Piece::Bishop).iterate_board(|position| score+=get_bishop_weight(position, &board, color, king_usize));
    board.get_pieces(color, Piece::Rook).iterate_board(|position| score+=get_rook_weight(position, &board, king_usize));
    board.get_pieces(color, Piece::Queen).iterate_board(|position| score+=get_queen_weight(position, &board, king_usize));
    score+=get_king_weight(king_usize, pieces);
    return score;
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

fn get_douplicate_pawn_tariff(board: &Chessboard, color: Color) -> f32{
    let mut tariff = 0.0;
    for field in DOUPLICATE_PAWN_TARIFF.iter(){
        let fields = board.get_pieces(color, Piece::Pawn).board & field.board;
        let used_fields = fields.count_ones();
        if used_fields > 1{
            tariff = tariff + (used_fields-1) as f32 * 0.3
        }
    }
    return tariff;
}

fn get_pieces_value(board: &Chessboard, color: Color) -> f32{
    // counting ones is faster than iterating board
    let mut score = 0.0;
    score = score+board.get_pieces(color, Piece::Pawn).board.count_ones() as f32;
    score = score+ 3.0*board.get_pieces(color, Piece::Knight).board.count_ones() as f32;
    score = score+ 3.2*board.get_pieces(color, Piece::Bishop).board.count_ones() as f32;
    score = score+ 5.0*board.get_pieces(color, Piece::Rook).board.count_ones() as f32;
    score = score+ 9.0*board.get_pieces(color, Piece::Queen).board.count_ones() as f32;
    score
}


pub fn evaluate(board: &Chessboard) -> f32 {
 
    let white_pieces_value: f32 = get_pieces_value(&board, Color::White);
    let black_pieces_value: f32 = get_pieces_value(&board, Color::Black);

    let white_king_usize =  board.get_pieces(Color::White, Piece::King).get_first_field(); 
    let black_king_usize = board.get_pieces(Color::Black, Piece::King).get_first_field();

    let white_king_position = check_where_king_is_located(white_king_usize, false);
    let black_king_position = check_where_king_is_located(black_king_usize, true);

    let white_pieces_position_value = get_position_weight(&board, Color::White, &white_king_position, black_pieces_value, white_king_usize);
    let black_pieces_position_value = get_position_weight(&board, Color::Black, &black_king_position, white_pieces_value, black_king_usize);
    // in the endgame push opponent king to the edge of the board
    let white_opponent_king_bonus = get_opponent_king_bonus(black_pieces_value, &black_king_usize);
    let black_opponent_king_bonus = get_opponent_king_bonus(white_pieces_value, &white_king_usize);

    let white_douplicate_pawn_tariff = get_douplicate_pawn_tariff(&board, Color::White);
    let black_douplicate_pawn_tariff = get_douplicate_pawn_tariff(&board, Color::Black);   

    let white_value = white_pieces_value + white_pieces_position_value + white_opponent_king_bonus - white_douplicate_pawn_tariff;
    let black_value = black_pieces_value + black_pieces_position_value + black_opponent_king_bonus - black_douplicate_pawn_tariff;


    return white_value - black_value;
}
