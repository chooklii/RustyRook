use rustc_hash::FxHashMap;

use crate::{board::bitboard::Bitboard, helper::moves_by_field::MoveInEveryDirection};

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