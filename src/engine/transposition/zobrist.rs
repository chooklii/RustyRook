use crate::figures::{color::Color, piece::Piece};
use rand::Rng;

pub fn get_transposition_figure_random_numbers() ->  [[[u64; 64]; 6];2] {
    let mut rng = rand::rng();
    let mut numbers = [[[u64::default(); 64];6];2];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column as usize * 8 + row as usize;
            numbers[Color::White as usize][Piece::Bishop as usize][position] = rng.random();
            numbers[Color::White as usize][Piece::King as usize][position] = rng.random();
            numbers[Color::White as usize][Piece::Knight as usize][position] = rng.random();
            numbers[Color::White as usize][Piece::Rook as usize][position] = rng.random();
            numbers[Color::White as usize][Piece::Pawn as usize][position] = rng.random();
            numbers[Color::White as usize][Piece::Queen as usize][position] = rng.random();

            numbers[Color::Black as usize][Piece::Bishop as usize][position] = rng.random();
            numbers[Color::Black as usize][Piece::King as usize][position] = rng.random();
            numbers[Color::Black as usize][Piece::Knight as usize][position] = rng.random();
            numbers[Color::Black as usize][Piece::Rook as usize][position] = rng.random();
            numbers[Color::Black as usize][Piece::Pawn as usize][position] = rng.random();
            numbers[Color::Black as usize][Piece::Queen as usize][position] = rng.random();
        }
    }
    numbers
}

pub fn get_transposition_en_passant_numbers() ->  [u64; 64] {
    let mut rng = rand::rng();
    let mut numbers = [u64::default(); 64];
    for column in 0..8 {
        for row in 0..8 {
            let position: usize = column as usize * 8 + row as usize;
            numbers[position] = rng.random();
        }
    }
    numbers
}