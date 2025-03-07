use crate::board::board::Chessboard;

pub struct Evaluation {
    white_pieces_value: u8,
    black_pieces_value: u8,
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
    }
}
