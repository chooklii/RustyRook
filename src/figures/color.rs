#[derive(PartialEq, Eq, Clone)]
pub enum Color {
    Black,
    White,
}

impl Default for Color{
    fn default() -> Color {
        Color::White
    }
}