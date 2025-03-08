use std::cmp::Ordering;

use crate::board::board::Chessboard;

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Copy)]
pub struct Evaluation {
    white_pieces_value: u8,
    black_pieces_value: u8,
    net_rating: i32
}

impl Ord for Evaluation {
    fn cmp(&self, other: &Self  ) -> Ordering  {
        if self.net_rating > other.net_rating {
            Ordering::Greater
        }else {
            Ordering::Less
        }
    }
}

pub fn evaluate(board: &Chessboard) -> Evaluation {
    let white_pieces_value: u8 = board
        .white_figures
        .iter()
        .map(|(_, fig)| fig.get_weight())
        .sum();
    
    let black_pieces_value: u8 = board
        .black_figures
        .iter()
        .map(|(_, fig)| fig.get_weight())
        .sum();

    Evaluation {
        white_pieces_value,
        black_pieces_value,
        net_rating: i32::from(white_pieces_value) - i32::from(black_pieces_value)

    }
}
