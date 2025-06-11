use rustc_hash::FxHashMap;

use crate::{board::bitboard::Bitboard, evaluation::KingPosition, figures::color::Color, helper::moves_by_field::MoveInEveryDirection};

use super::magic_bitboard::MagicBitboard;


// get index from array of possible moves
pub fn get_magic_index(blockers: Bitboard, magic_bitboard: &MagicBitboard) -> usize {
    let relevant_pieces = Bitboard {
        board: blockers.board & magic_bitboard.relevant_fields.board,
    };
    let hash = relevant_pieces.board.wrapping_mul(magic_bitboard.magic_key);
    (hash >> magic_bitboard.index) as usize
}

fn itter_direction(
    moves: &Vec<usize>,
    possible_moves: &mut Bitboard,
    blockers: Bitboard
){
    for single in moves {
        possible_moves.set_field(*single);
        if blockers.field_is_used(*single) {
            return
        }
    }
}

// for each color and each position of the king
pub fn init_king_safety_bitboards() -> [[Bitboard; 3]; 2]{
    let mut return_value = [[Bitboard::new(), Bitboard::new(), Bitboard::new()], [Bitboard::new(), Bitboard::new(), Bitboard::new()]];

    return_value[Color::White as usize][KingPosition::LEFT as usize].set_field(8);
    return_value[Color::White as usize][KingPosition::LEFT as usize].set_field(9);
    return_value[Color::White as usize][KingPosition::LEFT as usize].set_field(10);

    return_value[Color::White as usize][KingPosition::RIGHT as usize].set_field(13);
    return_value[Color::White as usize][KingPosition::RIGHT as usize].set_field(14);
    return_value[Color::White as usize][KingPosition::RIGHT as usize].set_field(15);

    return_value[Color::Black as usize][KingPosition::LEFT as usize].set_field(53);
    return_value[Color::Black as usize][KingPosition::LEFT as usize].set_field(54);
    return_value[Color::Black as usize][KingPosition::LEFT as usize].set_field(55);

    return_value[Color::Black as usize][KingPosition::RIGHT as usize].set_field(48);
    return_value[Color::Black as usize][KingPosition::RIGHT as usize].set_field(49);
    return_value[Color::Black as usize][KingPosition::RIGHT as usize].set_field(50);

    return_value
}

pub fn get_valid_moves_for_position_with_given_blockers(
    blockers: Bitboard,
    position: usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    is_rook: bool
) -> Bitboard {
    let mut possible_moves = Bitboard::new();

    // if this fails good night
    let relevant_moves = moves_by_field.get(&position).unwrap();

    if is_rook{
        itter_direction(&relevant_moves.left, &mut possible_moves, blockers);
        itter_direction(&relevant_moves.right, &mut possible_moves, blockers);
        itter_direction(&relevant_moves.forward, &mut possible_moves, blockers);
        itter_direction(&relevant_moves.back, &mut possible_moves, blockers);
    }else{
        itter_direction(&relevant_moves.left_back, &mut possible_moves, blockers);
        itter_direction(&relevant_moves.left_forward, &mut possible_moves, blockers);
        itter_direction(&relevant_moves.right_forward, &mut possible_moves, blockers);
        itter_direction(&relevant_moves.right_back, &mut possible_moves, blockers);
    }

    possible_moves
}