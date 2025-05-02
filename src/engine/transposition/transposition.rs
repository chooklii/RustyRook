use crate::{engine::engine::PossibleMove};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Flag {
    Exact,
    Lowerbound,
    Upperbound
}


#[derive(Clone, Debug, Copy)]
pub struct Transposition{
    pub hash: u64,
    pub depth: u8,
    pub evaluation: f32,
    pub best_move: PossibleMove,
    pub flag: Flag
}

impl Default for Transposition {
    fn default() -> Transposition {
        Transposition {
            hash: 0,
            depth: 0,
            evaluation: 0.0,
            best_move: PossibleMove{from: 0, to: 0, promoted_to: None},
            flag: Flag::Exact
        }
    }
}