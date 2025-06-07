use crate::figures::color::Color;

#[derive(Clone, Copy, Debug)]
#[derive(Default)]
pub struct Castle {
    pub white_castle_long: bool,
    pub white_castle_short: bool,
    pub black_castle_long: bool,
    pub black_castle_short: bool,
}


impl Castle {
    pub fn white_can_castle(&self) -> bool {
        self.white_castle_long || self.white_castle_short
    }

    pub fn black_can_castle(&self) -> bool {
        self.black_castle_long || self.black_castle_short
    }

    pub fn can_castle(&self, color: Color) -> bool{
        match color{
            Color::White => self.white_can_castle(),
            Color::Black => self.black_can_castle()
        }
    }

    pub fn set_has_castled(&mut self, color: Color){
        match color{
            Color::White => {
                self.white_castle_long = false;
                self.white_castle_short = false;
            },
            Color::Black => {
                self.black_castle_long = false;
                self.black_castle_short = false
            }
        }
    }
}
