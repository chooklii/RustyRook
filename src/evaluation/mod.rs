use std::{u64, usize};

use crate::{board::{bitboard::Bitboard, board::Chessboard}, figures::{color::Color, piece::Piece, sliding_moves::{get_fields_threatened_by_bishop, get_fields_threatened_by_queen, get_fields_threatened_by_rook}}, DOUPLICATE_PAWN_TARIFF, KING_SAFETY_FIELDS, PASSED_PAWN_ROWS};

// a1 to h8
const PAWN_RATE_KING_CENTER: [f32; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.8, 0.8, 1.0, 1.1, 1.1, 1.0, 0.8, 0.8, 
    0.9, 0.8, 1.1, 1.3, 1.3, 1.1, 0.8, 0.9, 
    1.0, 1.0, 1.1, 1.3, 1.3, 1.1, 1.0, 1.0, 
    1.0, 1.0, 1.1, 1.2, 1.2, 1.1, 1.0, 1.0, 
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
];

const PAWN_RATE_KING_LEFT: [f32; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    0.7, 0.7, 0.7, 1.1, 1.1, 1.1, 1.1, 1.1, 
    0.7, 0.7, 0.7, 1.3, 1.3, 1.3, 1.3, 1.3, 
    1.0, 1.0, 1.2, 1.3, 1.3, 1.3, 1.3, 1.3, 
    1.0, 1.0, 1.1, 1.1, 1.1, 1.3, 1.3, 1.3, 
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.5, 2.5, 
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
];

const PAWN_RATE_KING_RIGHT: [f32; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    1.1, 1.1, 1.1, 1.1, 1.1, 0.7, 0.7, 0.7, 
    1.3, 1.3, 1.3, 1.3, 1.3, 0.7, 0.7, 0.7, 
    1.3, 1.3, 1.3, 1.3, 1.3, 1.2, 1.0, 1.0, 
    1.3, 1.3, 1.3, 1.1, 1.1, 1.1, 1.0, 1.0, 
    2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

const KNIGHT_RATE: [f32; 64] = [
    1.8, 2.9, 2.0, 2.0, 2.0, 2.0, 2.9, 1.8, 
    2.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 2.0, 
    2.0, 3.1, 3.1, 3.1, 3.1, 3.1, 3.1, 2.0, 
    2.0, 3.0, 3.2, 3.2, 3.2, 3.2, 3.0, 2.0, 
    2.0, 3.0, 3.2, 3.2, 3.2, 3.2, 3.0, 2.0, 
    2.0, 3.2, 3.2, 3.2, 3.2, 3.2, 3.2, 2.0, 
    2.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 2.0, 
    1.8, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.8,
];

const ROOK_RATE: [f32; 64] = [
    4.95,5.0, 5.05,5.1, 5.1, 5.05,5.0, 4.95, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 
    5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
];

const BISHOP_RATE: [f32; 64] = [
    3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 
    3.0, 3.1, 3.0, 3.0, 3.0, 3.0, 3.1, 3.0, 
    3.0, 3.0, 3.1, 3.0, 3.0, 3.1, 3.0, 3.0, 
    3.0, 3.0, 3.0, 3.1, 3.1, 3.0, 3.0, 3.0, 
    3.0, 3.0, 3.0, 3.1, 3.1, 3.0, 3.0, 3.0, 
    3.0, 3.0, 3.1, 3.0, 3.0, 3.1, 3.0, 3.0, 
    3.0, 3.1, 3.0, 3.0, 3.0, 3.0, 3.1, 3.0, 
    3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 
];

const QUEEN_RATE: [f32; 64] = [
    8.7, 8.8, 9.0, 9.0, 9.0, 9.0, 8.8, 8.7, 
    8.8, 9.0, 9.1, 9.1, 9.1, 9.1, 9.0, 8.8, 
    8.9, 9.0, 9.1, 9.1, 9.1, 9.1, 9.0, 8.9, 
    8.9, 9.0, 9.1, 9.1, 9.1, 9.1, 9.0, 8.9, 
    8.9, 9.0, 9.1, 9.1, 9.1, 9.1, 9.0, 8.9, 
    8.9, 9.0, 9.1, 9.1, 9.1, 9.1, 9.0, 8.9, 
    8.8, 9.0, 9.1, 9.1, 9.1, 9.1, 9.0, 8.8, 
    8.7, 8.8, 9.0, 9.0, 9.0, 9.0, 8.8, 8.7,
];

// white and black is different rate due to king castle (and starting on other square)
const EARLY_GAME_KING_RATE_WHITE: [f32; 64] = [
    1.2, 1.3, 1.3, 1.0, 1.0, 1.1, 1.3, 1.2,
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
];

// from back pov (so reverse)
const EARLY_GAME_KING_RATE_BLACK: [f32; 64] = [
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 
    0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3,
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,  
    1.2, 1.3, 1.3, 1.0, 1.0, 1.1, 1.3, 1.2,
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
pub enum KingPosition{
    CENTER,
    LEFT,
    RIGHT
}


fn check_where_king_is_located(king_position: usize, is_black_king: bool) -> KingPosition{
    if is_black_king{
        return match king_position % 8{
            0..=2 => KingPosition::RIGHT,
            5..=7 => KingPosition::LEFT,
            _ => KingPosition::CENTER
        }
    }                                    
    match king_position % 8{
        0..=2 => KingPosition::LEFT,
        5..=7 => KingPosition::RIGHT,
        _ => KingPosition::CENTER
    }
}

fn get_king_weight(position: usize, king_position: KingPosition, color: Color, own_positions: &Bitboard, pieces: f32) -> f32 {
    // if there is less then 15 pieces value left activate king
    if pieces <=15.0{
        return LATE_GAME_KING_RATE[position];
    }
    // add a little bonus for all used fields in front of the king -> king safety!
    let fields = KING_SAFETY_FIELDS[color as usize][king_position as usize].board & own_positions.board;

    let safety_bonus = fields.count_ones() as f32 * 0.07;
    match color{
        Color::White => safety_bonus + EARLY_GAME_KING_RATE_WHITE[position],
        Color::Black => safety_bonus + EARLY_GAME_KING_RATE_BLACK[position]
    }
}

fn get_pawn_rate(position: usize, king_position: &KingPosition) -> f32{
    // push pawns on side where out king is not!
    match king_position{
        KingPosition::CENTER => PAWN_RATE_KING_CENTER[position],
        KingPosition::LEFT => PAWN_RATE_KING_LEFT[position],
        KingPosition::RIGHT => PAWN_RATE_KING_RIGHT[position]
    }
}

fn get_rook_weight(position: usize, board: &Chessboard, king_position: usize) -> f32{
    let threatened_fields = get_fields_threatened_by_rook(board, position, king_position);
    let mut threat_bonus = ROOK_RATE[position];
    threat_bonus += 0.02 * threatened_fields.board.count_ones() as f32;
    threat_bonus
}

fn get_bishop_weight( position: usize, board: &Chessboard, king_position: usize) -> f32{
    let threatened_fields = get_fields_threatened_by_bishop(board, position, king_position);
    let mut weight = BISHOP_RATE[position];
    weight += 0.02*threatened_fields.board.count_ones() as f32;
    weight
}

fn get_queen_weight(position: usize, board: &Chessboard, king_position: usize) -> f32{
    let threatened_fields = get_fields_threatened_by_queen(board, position, king_position);
    let mut weight = QUEEN_RATE[position];
    weight += 0.02 * threatened_fields.board.count_ones() as f32;
    weight
}

fn get_position_weight(board: &Chessboard, color: Color, king_position: KingPosition, pieces: f32, king_usize: usize) -> f32 {
    let mut score: f32 = 0.0;
    board.get_pieces(color, Piece::Pawn).iterate_board(|position| 
        score+=get_pawn_rate(get_position(position, color), &king_position));

    board.get_pieces(color, Piece::Knight).iterate_board(|position| 
        score += KNIGHT_RATE[get_position(position, color)]);

    board.get_pieces(color, Piece::Bishop).iterate_board(|position| 
        score+=get_bishop_weight(get_position(position, color), board, king_usize));

    board.get_pieces(color, Piece::Rook).iterate_board(|position| 
        score+=get_rook_weight(get_position(position, color), board, king_usize));

    board.get_pieces(color, Piece::Queen).iterate_board(|position| 
        score+=get_queen_weight(get_position(position, color), board, king_usize));

    score+=get_king_weight(king_usize, king_position, color,board.get_positions_by_current_player(), pieces);
    score
}

fn get_position(position: usize, color: Color) -> usize{
    match color{
            Color::White => position,
            Color::Black => 63 - position    
        }
}

fn get_opponent_king_bonus(piece_value: f32, king_position: &usize) -> f32{
    if piece_value > 15.0{
        return 0.0
    }
    let king_column = king_position % 8 ;
    if !(&8..=&55).contains(&king_position) || king_column == 0 || king_column == 7{
        return 2.0;
    };
    if !(&15..=&47).contains(&king_position) || king_column == 1 || king_column == 6{
        return 1.0;
    }
    0.0
}

fn get_douplicate_pawn_tariff(board: &Chessboard, color: Color) -> f32{
    let mut tariff = 0.0;
    for field in DOUPLICATE_PAWN_TARIFF.iter(){
        let fields = board.get_pieces(color, Piece::Pawn).board & field.board;
        let used_fields = fields.count_ones();
        if used_fields > 1{
            tariff += (used_fields-1) as f32 * 0.3
        }
    }
    tariff
}

fn get_pieces_value(board: &Chessboard, color: Color) -> f32{
    // counting ones is faster than iterating board
    let mut score = 0.0;
    score += board.get_pieces(color, Piece::Pawn).board.count_ones() as f32;
    score += 3.0*board.get_pieces(color, Piece::Knight).board.count_ones() as f32;
    score += 3.2*board.get_pieces(color, Piece::Bishop).board.count_ones() as f32;
    score += 5.0*board.get_pieces(color, Piece::Rook).board.count_ones() as f32;
    score += 9.0*board.get_pieces(color, Piece::Queen).board.count_ones() as f32;
    score
}


pub fn evaluate_for_own_color(board: &Chessboard) -> f32{
    let evaluation = evaluate(board);
    match board.current_move{
        Color::Black => -evaluation,
        Color::White => evaluation
    }
}

fn evaluate(board: &Chessboard) -> f32 {
 
    let white_pieces_value: f32 = get_pieces_value(board, Color::White);
    let black_pieces_value: f32 = get_pieces_value(board, Color::Black);

    let white_king_usize =  board.get_pieces(Color::White, Piece::King).get_first_field(); 
    let black_king_usize = board.get_pieces(Color::Black, Piece::King).get_first_field();

    let white_king_position = check_where_king_is_located(white_king_usize, false);
    let black_king_position = check_where_king_is_located(black_king_usize, true);

    let white_pieces_position_value = get_position_weight(board, Color::White, white_king_position, black_pieces_value, white_king_usize);
    let black_pieces_position_value = get_position_weight(board, Color::Black, black_king_position, white_pieces_value, black_king_usize);

    // in the endgame push opponent king to the edge of the board
    let white_opponent_king_bonus = get_opponent_king_bonus(black_pieces_value, &black_king_usize);
    let black_opponent_king_bonus = get_opponent_king_bonus(white_pieces_value, &white_king_usize);

    let white_douplicate_pawn_tariff = get_douplicate_pawn_tariff(board, Color::White);
    let black_douplicate_pawn_tariff = get_douplicate_pawn_tariff(board, Color::Black);   

    // give extra bonus to passed pawns
    let white_passed_pawn_value = get_passed_pawn_bonus_white(board);
    let black_passed_pawn_value = get_passed_pawn_bonus_black(board);

    let white_value = white_pieces_value 
                            + white_pieces_position_value 
                            + white_opponent_king_bonus 
                            + white_passed_pawn_value 
                            - white_douplicate_pawn_tariff;

    let black_value = black_pieces_value 
                            + black_pieces_position_value 
                            + black_opponent_king_bonus 
                            + black_passed_pawn_value 
                            - black_douplicate_pawn_tariff;

    white_value - black_value
}

fn get_passed_pawn_bonus_white(board: &Chessboard) -> f32{
    let mut bonus = 0.0;

    let white_pawns = board.get_pieces(Color::White, Piece::Pawn);
    white_pawns.iterate_board(|pawn_position| {
        let (column, row) = get_column_and_row_from_position(pawn_position);
        // get all fields where opponent pawns can prevent this pawn from beeing a passed one
        let relevant_columns = u64::MAX << 8*column;
        let relevant_rows = PASSED_PAWN_ROWS[row-1];

        let blockers =  relevant_columns & relevant_rows.board & board.get_pieces(Color::Black, Piece::Pawn).board;
        // no opponent pawn on these fields -> we got a passed pawn!
        if blockers == 0{
            // the higher up the pawn - the better!
            bonus += 0.3 * column as f32
        }
    });

    bonus
}

fn get_passed_pawn_bonus_black(board: &Chessboard) -> f32{
    let mut bonus = 0.0;

    let white_pawns = board.get_pieces(Color::Black, Piece::Pawn);
    white_pawns.iterate_board(|pawn_position| {
        let (column, row) = get_column_and_row_from_position(pawn_position);
        // get all fields where opponent pawns can prevent this pawn from beeing a passed one
        let relevant_columns = u64::MAX >> 8*(9-column);
        let relevant_rows = PASSED_PAWN_ROWS[row-1];

        let blockers =  relevant_columns & relevant_rows.board & board.get_pieces(Color::White, Piece::Pawn).board;
        // no opponent pawn on these fields -> we got a passed pawn!
        if blockers == 0{
            // the higher up the pawn - the better!
            bonus += 0.3 * (9-column) as f32
        }
    });

    bonus
}


// both starting at 1 and ending at 8
fn get_column_and_row_from_position(position: usize) -> (usize, usize){
    let column = position / 8 +1 ;
    let row = position % 8 +1;
    (column, row)
}

#[cfg(test)]

mod tests{
    use crate::{board::board::Chessboard, evaluation::{get_column_and_row_from_position, get_passed_pawn_bonus_black, get_passed_pawn_bonus_white}};

    #[test]
    fn test_column_and_row_finder(){
        let (column, row) = get_column_and_row_from_position(0);
        assert_eq!(1, column);
        assert_eq!(1, row);

        let (column, row) = get_column_and_row_from_position(27);
        assert_eq!(4, column);
        assert_eq!(4, row);

        let (column, row) = get_column_and_row_from_position(22);
        assert_eq!(3, column);
        assert_eq!(7, row);

        let (column, row) = get_column_and_row_from_position(63);
        assert_eq!(8, column);
        assert_eq!(8, row);

        let (column, row) = get_column_and_row_from_position(43);
        assert_eq!(6, column);
        assert_eq!(4, row);
    }

    #[test]
    fn test_white_passed_pawn(){
        let mut board = Chessboard{..Default::default()};

        // three passed pawns - all at 2nd rank
        board.create_position_from_input_string(String::from("k7/8/8/8/8/8/1P1P1P2/K7 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_white(&board);
        assert_eq!(1.8, (bonus * 10.0).round() / 10.0);

        // now two of them are blocked by a black pawn
        board.create_position_from_input_string(String::from("k7/2p5/8/8/8/8/1P1P1P2/K7 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_white(&board);
        assert_eq!(0.6, bonus);

        // only one is blocked and pawn on a-row should not be effected by opponent pawn on h
        board.create_position_from_input_string(String::from("k7/3p3p/8/8/8/8/P2P1P2/K7 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_white(&board);
        assert_eq!(1.2, bonus);

        // same with h not effected by a 
        board.create_position_from_input_string(String::from("k7/p2p4/8/8/8/8/P2P3P/K7 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_white(&board);
        assert_eq!(0.6, bonus);

        // pawn is one field from promotion!
        board.create_position_from_input_string(String::from("8/1p1p2pP/8/1P1P4/8/K2k4/8/8 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_white(&board);
        assert_eq!(2.1, (bonus * 10.0).round() / 10.0)
    }

    #[test]
    fn test_black_passed_pawn(){
        let mut board = Chessboard{..Default::default()};

        // one passed pawn on 7th rank
        board.create_position_from_input_string(String::from("8/1p1p2pP/8/1P1P4/8/K2k4/8/8 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_black(&board);
        assert_eq!(0.6, bonus);

        // one passed pawn  - but on 3rd rank!
        board.create_position_from_input_string(String::from("8/1p1p3P/8/1P1P4/8/K2k2p1/8/8 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_black(&board);
        assert_eq!(1.8, (bonus * 10.0).round() / 10.0);

        // pawn on A not effected by pawn on H
        board.create_position_from_input_string(String::from("8/8/p1pp4/2PP3P/8/K2k4/8/8 w - - 0 1"));
        let bonus = get_passed_pawn_bonus_black(&board);
        assert_eq!(0.9, (bonus * 10.0).round() / 10.0);
    }
}