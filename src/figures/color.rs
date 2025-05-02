#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Color {
    White,
    Black,
}

impl Default for Color{
    fn default() -> Color {
        Color::White
    }
}