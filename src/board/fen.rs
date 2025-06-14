#[derive(PartialEq)]
pub enum FEN {
    FIGURES,
    MOVE,
    CASTLING,
    ENPASSANT,
    // as of now we dont use played moves
    IGNORED
}

impl FEN {
    
    pub fn update_to_next_state(self) -> FEN{
        match self{
            FEN::FIGURES => FEN::MOVE,
            FEN::MOVE => FEN::CASTLING,
            FEN::CASTLING => FEN::ENPASSANT,
            FEN::ENPASSANT => FEN::IGNORED,
            FEN::IGNORED => FEN::IGNORED
        }
    }
}