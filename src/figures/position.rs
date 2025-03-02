use super::color::Color;

pub fn calculate_next_position(color: &Color, own_position: &usize, movement: usize) -> usize {
    match color {
        Color::Black => return own_position - movement,
        Color::White => return own_position + movement,
    }
}