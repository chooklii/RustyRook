use std::cmp::min;

use rustc_hash::FxHashMap;

use crate::{board::bitboard::Bitboard, figures::color::Color};

/*
 calculate all maximum possible moves in every direction once and then just use them when calculating the moves
 
 Functions in here should prob. only be used for static values as they are not improved for performance
*/
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct MoveInEveryDirection {
    pub left: Vec<usize>,
    pub right: Vec<usize>,
    pub forward: Vec<usize>,
    pub back: Vec<usize>,
    pub left_forward: Vec<usize>,
    pub left_back: Vec<usize>,
    pub right_forward: Vec<usize>,
    pub right_back: Vec<usize>,
}

pub fn get_moves_for_each_field() -> FxHashMap<usize, MoveInEveryDirection> {
    let mut values: FxHashMap<usize, MoveInEveryDirection> = FxHashMap::default();

    for column in 0..8 {
        for row in 0..8 {
            let mut left = Vec::new();
            let mut right= Vec::new();
            let mut forward= Vec::new();
            let mut back= Vec::new();
            let mut left_forward = Vec::new();
            let mut left_back= Vec::new();
            let mut right_forward= Vec::new();
            let mut right_back= Vec::new();
            let index: usize = column *8 + row;

            for val in 0..row{
                left.push(index - val -1)
            };
            for val in 0..(7-row){
                right.push(index + val + 1)
            }

            for val in 1..=(7-column){
                forward.push(index + (val*8))
            }
            
            for val in 1..=column{
                back.push(index - (val*8))
            };

            // Bishop
            let left_fw_moves = min(left.len(), forward.len());
            let left_bw_moves = min(left.len(), back.len());
            let right_fw_moves = min(right.len(), forward.len());
            let right_bw_moves = min(right.len(), back.len()); 

            for val in 1..=left_fw_moves{
                left_forward.push(index + (val*7))
            };
            for val in 1..=left_bw_moves{
                left_back.push(index - (val*9))
            };
            for val in 1..=right_fw_moves{
                right_forward.push(index + (val*9))
            };
            for val in 1..=right_bw_moves{
                right_back.push(index - (val*7))
            };
            values.insert(
                index,
                MoveInEveryDirection {
                    // Bishop Movement
                    left_back,
                    left_forward,
                    right_back,
                    right_forward,
                    // Rook Movement
                    left,
                    right,
                    forward,
                    back,
                },
            );
        }
    }
    values
}

pub fn get_pawn_promotion_moves() -> Bitboard{
    let mut board = Bitboard::new();

    for val in 0..=7{
        board.set_field(val);
    }
    for val in 56..=63{
        board.set_field(val);
    }
    board
}

pub fn get_bishop_blockers_for_field(column: usize, row: usize) -> Bitboard {
    let mut blockers = Bitboard::new();
    let position: usize = column *8 + row;
    let left = 0..row;
    let right = 0..(7 - row);
    let forward = 1..(8 - column);
    let back = 1..(column + 1);

    let left_fw_moves = min(left.len(), forward.len());
    let left_bw_moves = min(left.len(), back.len());
    let right_fw_moves = min(right.len(), forward.len());
    let right_bw_moves = min(right.len(), back.len());

    if left_fw_moves > 1 {
        for val in 1..left_fw_moves {
            blockers.set_field(position + (val * 7))
        }
    }
    if left_bw_moves > 1 {
        for val in 1..left_bw_moves {
            blockers.set_field(position - (val * 9))
        }
    }
    if right_fw_moves > 1 {
        for val in 1..right_fw_moves {
            blockers.set_field(position + (val * 9))
        }
    }
    if right_bw_moves > 1 {
        for val in 1..right_bw_moves {
            blockers.set_field(position - (val * 7))
        }
    }
    blockers
}

pub fn get_rook_blockers_for_field(column: usize, row: usize) -> Bitboard {
    let mut moves = Bitboard::new();
    let position: usize = column *8 + row;

    if row > 1 {
        for val in 0..(row - 1) {
            moves.set_field(position - val - 1)
        }
    }
    if row <= 6 {
        for val in 0..(6 - row) {
            moves.set_field(position + val + 1)
        }
    }
    if column <= 6 {
        for val in 1..=(6 - column) {
            moves.set_field(position + (val * 8))
        }
    }
    if column > 1 {
        for val in 1..=(column - 1) {
            moves.set_field(position - (val * 8))
        }
    }
    moves
}

pub fn get_douplicate_pawn_boards() -> [Bitboard; 8]{
    let mut fields = [Bitboard::new(); 8];

    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;
            fields[row].set_field(position);
        }
    }
    fields
}


pub fn get_pawn_takes_for_field() -> [[Bitboard; 64];2] {
    let mut moves  = [[Bitboard::new(); 64],[Bitboard::new(); 64]];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;

            if row >=1{
                if column <=6{
                    moves[Color::White as usize][position].set_field(position +7);
                }
                if column >=1{
                    moves[Color::Black as usize][position].set_field(position - 9);
                }
            }
            if row <=6{
                if column <=6{
                    moves[Color::White as usize][position].set_field(position +9);
                }
                if column >=1{
                    moves[Color::Black as usize][position].set_field(position - 7);
                }
            }
        }
    }
    moves  
}

pub fn get_king_moves_for_field() -> [Bitboard; 64] {
    let mut moves: [Bitboard; 64] = [Bitboard::new(); 64];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;
            if column >=1 {
                moves[position].set_field(position - 8);
                if row >=1 {
                    moves[position].set_field(position - 9);
                }
                if row <=6 {
                    moves[position].set_field(position - 7);
                }
            }
            if column <=6 {
                moves[position].set_field(position + 8);
                if row >=1 {
                    moves[position].set_field(position + 7);
                }
                if row <=6 {
                    moves[position].set_field(position + 9);
                }
            }
            if row >= 1 {
                moves[position].set_field(position - 1);
            }
            if row <=6 {
                moves[position].set_field(position + 1);
            }
        }
    }
    moves
}

pub fn get_knight_moves_for_field() -> [Bitboard; 64] {
    let mut moves: [Bitboard; 64] = [Bitboard::new(); 64];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column *8 + row;
            if row >= 2 {
                if column >= 1 {
                    moves[position].set_field(position - 10);
                }
                if column <=6 {
                    moves[position].set_field(position + 6);
                }
            }
            if row <= 5 {
                if column >= 1 {
                    moves[position].set_field(position - 6);
                }
                if column <=6 {
                    moves[position].set_field(position + 10);
                }
            }
            if column >= 2 {
                if row >= 1 {
                    moves[position].set_field(position - 17);
                }
                if row <= 6 {
                    moves[position].set_field(position - 15);
                }
            }
            if column <=5 {
                if row >= 1 {
                    moves[position].set_field(position + 15);
                }
                if row <= 6 {
                    moves[position].set_field(position + 17);
                }
            }
        }
    }
    moves
}
