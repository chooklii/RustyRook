use crate::{board::board::Chessboard, figures::figures::Figure};

#[derive(PartialEq, PartialOrd, Clone, Debug, Copy)]
pub struct Evaluation {
    pub white_pieces_value: f32,
    pub black_pieces_value: f32,
    pub net_rating: f32
}

impl Default for Evaluation{
    fn default() -> Evaluation {
        Evaluation{
            white_pieces_value: 0.0,
            black_pieces_value: 0.0,
            net_rating: 0.0
        }
    }
}


// a1 to h8
const PAWN_RATE: [f32; 64] = [
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
   1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
   1.0, 1.0, 1.1, 1.1, 1.1, 1.1, 1.0, 1.0,
   1.0, 1.0, 1.3, 1.3, 1.3, 1.3, 1.0, 1.0,
   1.0, 1.0, 1.3, 1.3, 1.3, 1.3, 1.0, 1.0,
   1.0, 1.0, 1.1, 1.1, 1.1, 1.1, 1.0, 1.0,
   1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
];

const KNIGHT_RATE: [f32; 64] = [
   1.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 1.5,
   2.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 2.0,
   2.0, 3.1, 3.2, 3.2, 3.2, 3.2, 3.1, 2.0,
   2.0, 3.0, 3.5, 3.5, 3.5, 3.5, 3.0, 2.0,
   2.0, 3.0, 3.5, 3.5, 3.5, 3.5, 3.0, 2.0,
   2.0, 3.1, 3.2, 3.2, 3.2, 3.2, 3.1, 2.0,
   2.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 2.0,
   1.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 1.5,
];

const ROOK_RATE: [f32; 64] = [
   5.0, 5.0, 5.1, 5.1, 5.1, 5.1, 5.0, 5.0,
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
   5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
   5.0, 5.0, 5.1, 5.1, 5.1, 5.1, 5.0, 5.0,
];

const BISHOP_RATE: [f32; 64] = [
   2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
   3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
   3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
   3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
   3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
   3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
   3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
   2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
];

const QUEEN_RATE: [f32; 64] = [
   8.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 8.0,
   8.0, 9.0, 9.2, 9.2, 9.2, 9.2, 9.0, 8.0,
   8.0, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.0,
   8.0, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.0,       
   8.0, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.0,   
   8.0, 9.0, 9.2, 9.3, 9.3, 9.2, 9.0, 8.0, 
   8.0, 9.0, 9.2, 9.2, 9.2, 9.2, 9.0, 8.0,
   8.0, 9.0, 9.0, 9.0, 9.0, 9.0, 9.0, 8.0
];

const KING_RATE: [f32; 64] = [
   90.4, 90.4, 90.2, 90.0, 90.0, 90.2, 90.4, 90.4,
   89.0, 89.0, 89.0, 89.0, 89.0, 89.0, 89.0, 89.0,
   88.0, 88.0, 88.0, 88.0, 88.0, 88.0, 88.0, 88.0,
   87.0, 87.0, 87.0, 87.0, 87.0, 87.0, 87.0, 87.0,
   87.0, 87.0, 87.0, 87.0, 87.0, 87.0, 87.0, 87.0,
   88.0, 88.0, 88.0, 88.0, 88.0, 88.0, 88.0, 88.0,
   89.0, 89.0, 89.0, 89.0, 89.0, 89.0, 89.0, 89.0,
   90.4, 90.4, 90.2, 90.0, 90.0, 90.2, 90.4, 90.4,
];

fn get_figure_weight(figure: &Figure, position: usize) -> f32{
    return match figure{
        Figure::Pawn(_) => PAWN_RATE[position],
        Figure::Knight(_) => KNIGHT_RATE[position],
        Figure::Bishop(_) => BISHOP_RATE[position],
        Figure::Rook(_) => ROOK_RATE[position],
        Figure::Queen(_) => QUEEN_RATE[position],
        Figure::King(_) => KING_RATE[position]
    }
}

pub fn evaluate(board: &Chessboard) -> Evaluation {
    let white_pieces_value: f32 = board
        .white_figures
        .iter()
        .map(|(&position, figure)| get_figure_weight(figure, position))
        .sum();
    
    let black_pieces_value: f32 = board
        .black_figures
        .iter()
        .map(|(&position, figure)| get_figure_weight(figure, position))
        .sum();

    Evaluation {
        white_pieces_value,
        black_pieces_value,
        net_rating: white_pieces_value - black_pieces_value
    }
}
