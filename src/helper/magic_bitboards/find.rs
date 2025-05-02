use rustc_hash::FxHashMap;
use rand::Rng;

use crate::{board::bitboard::Bitboard, helper::moves_by_field::{get_bishop_blockers_for_field, get_moves_for_each_field, get_rook_blockers_for_field, MoveInEveryDirection}};

use super::{helper::{get_magic_index, get_valid_moves_for_position_with_given_blockers}, magic_bitboard::MagicBitboard};

pub fn init_bishop_magics() -> ([MagicBitboard; 64], [Vec<Bitboard>; 64]) {
    let mut magic_bitboards = [MagicBitboard{..Default::default()}; 64];
    let mut magic_positions = [const { Vec::new() }; 64];

    let possible_moves: FxHashMap<usize, MoveInEveryDirection> = get_moves_for_each_field();
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = usize::from(column as usize * 8 + row as usize);
            let blockers = get_bishop_blockers_for_field(column, row);
            let (magic_bitboard, positions) =
                find_magic(blockers, position, &possible_moves, false);
            magic_bitboards[position] = magic_bitboard;
            magic_positions[position] = positions;
        }
    }
    (magic_bitboards, magic_positions)
}

pub fn init_rook_magics() -> ([MagicBitboard; 64], [Vec<Bitboard>; 64]) {
    let mut magic_bitboards = [MagicBitboard{..Default::default()}; 64];
    let possible_moves: FxHashMap<usize, MoveInEveryDirection> = get_moves_for_each_field();
    let mut magic_positions = [const { Vec::new() }; 64];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = usize::from(column as usize * 8 + row as usize);
            let blockers = get_rook_blockers_for_field(column, row);
            let (magic_bitboard, positions) =
                find_magic(blockers, position, &possible_moves, true);
            magic_bitboards[position] = magic_bitboard;
            magic_positions[position] = positions;
        }
    }
    (magic_bitboards, magic_positions)
}

fn find_magic(
    blockers: Bitboard,
    position: usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    is_rook: bool
) -> (MagicBitboard, Vec<Bitboard>) {
    let mut rng = rand::rng();
    let shift: u8 = 64 - blockers.get_used_fields().len() as u8;
    loop {
        let magic = rng.random::<u64>() & rng.random::<u64>() & rng.random::<u64>();
        let magic_bitboard = MagicBitboard {
            relevant_fields: blockers,
            magic_key: magic,
            index: shift,
        };
        if let Some(valid) = create_possible_moves_vec(&magic_bitboard, position, &moves_by_field, is_rook) {
            return (magic_bitboard, valid);
        }
    }
}

fn create_possible_moves_vec(
    magic_bitboard: &MagicBitboard,
    own_position: usize,
    moves_by_field: &FxHashMap<usize, MoveInEveryDirection>,
    is_rook: bool
) -> Option<Vec<Bitboard>> {
    let index_bits = 64 - magic_bitboard.index;
    let mut table = vec![Bitboard::new(); 1 << index_bits];
    let mut blockers = Bitboard::new();
    loop {
        let moves = get_valid_moves_for_position_with_given_blockers(blockers, own_position, &moves_by_field, is_rook);
        let table_entry = &mut table[get_magic_index(blockers, magic_bitboard)];
        if table_entry.board == 0 {
            *table_entry = moves;
        } else if table_entry.board != moves.board {
            return None;
        }
        blockers.board = blockers
            .board
            .wrapping_sub(magic_bitboard.relevant_fields.board)
            & magic_bitboard.relevant_fields.board;
        if blockers.board == 0 {
            break;
        }
    }
    Some(table)
}
