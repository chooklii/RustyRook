use crate::TRANSPOSITION_TABLE;

use super::transposition::{Flag, Transposition};


pub fn get_entry_without_check(board_hash: u64) -> Option<Transposition> {
    if let Some(transposition) = TRANSPOSITION_TABLE.get(&board_hash) {
        if transposition.hash == board_hash {
            return Some(*transposition);
        }
    }
    None
}

pub fn get_entry(board_hash: u64, depth: u8, alpha: f32, beta: f32) -> Option<Transposition> {
    if let Some(transposition) = TRANSPOSITION_TABLE.get(&board_hash) {
        // field is currently used by default value or we need to search deeper
        if transposition.hash != board_hash || transposition.depth < depth {
            return None;
        }
        // already made calculation was exact - what more can we expect?
        if transposition.flag == Flag::Exact {
            return Some(*transposition);
        }
        // only use not exact values if they result in alpha/beta prunning
        if transposition.flag == Flag::Lowerbound && transposition.evaluation >= beta {
            return Some(*transposition);
        }
        if transposition.flag == Flag::Upperbound && transposition.evaluation <= alpha {
            return Some(*transposition);
        }
    }
    None
}
