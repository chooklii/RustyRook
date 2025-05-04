use crate::ZOBRIST_CURRENT_MOVE;

use super::transposition::{self, Flag, Transposition};


pub struct TranspositionTable{
    pub table: Vec<Transposition>
}

impl Default for TranspositionTable {
    fn default() -> TranspositionTable {
        let mut transposition_table = TranspositionTable { table: Vec::with_capacity(TABLE_SIZE)};
        //todo - better way?
        for _ in 0..TABLE_SIZE{
            transposition_table.table.push(Transposition{..Default::default()});
        }
        return transposition_table;
    }
}

// 40 bytes per Transposition -> 128 MB
const TABLE_SIZE: usize = 3_200_000;

impl TranspositionTable{

    fn get_index(&self, zobrist: u64) -> usize{
        return zobrist as usize % TABLE_SIZE
    }

    pub fn save_entry(&mut self, transposition: Transposition){
        if transposition.hash == 0{
            println!("Trying to add shit data: {:?}", transposition);
        }
        let index = self.get_index(transposition.hash);
        // in v1 we just overwrite everything, maybe need to add check for existing value and if so depth/flag check
        self.table[index] = transposition;
    }

    pub fn get_entry_without_check(&self, board_hash: u64) -> Option<Transposition>{
        if board_hash == 0{
            println!("Should not happen - why?");
            return None;
        }
        let index = self.get_index(board_hash);
        if let Some(&transposition) = self.table.get(index){
            if transposition.hash == board_hash{
                return Some(transposition)
            }
        }
        return None;
    }

    pub fn get_entry(&self, board_hash: u64, depth: u8, alpha: f32, beta: f32) -> Option<Transposition>{
        let index = self.get_index(board_hash);

        if let Some(&transposition) = self.table.get(index){
            // field is currently used by default value or we need to search deeper
            if transposition.hash != board_hash || transposition.depth < depth{
                return None;
            }

            // already made calculation was exact - what more can we expect?
            if transposition.flag == Flag::Exact{
                return Some(transposition);
            }
            // only use not exact values if they result in alpha/beta prunning
            if transposition.flag == Flag::Lowerbound && transposition.evaluation >= beta{
                return Some(transposition);
            }
            if transposition.flag == Flag::Upperbound && transposition.evaluation <= alpha{
                return Some(transposition);
            }
        }
        return None;
    }

}