use crate::board::bitboard::Bitboard;

#[derive(Debug, Clone, Copy)]
pub struct MagicBitboard {
    pub relevant_fields: Bitboard,
    pub magic_key: u64,
    pub index: u8,
}

impl Default for MagicBitboard {
    fn default() -> MagicBitboard {
        MagicBitboard {
            relevant_fields: Bitboard::new(),
            magic_key: 0,
            index: 0
        }
    }
}