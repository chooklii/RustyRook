use rustc_hash::FxHashMap;

use crate::{board::board::Chessboard, helper::moves_by_field::MoveInEveryDirection};

use super::figures::SingleMove;


#[derive(Default, Clone)]
pub struct Knight {}

impl Knight {

    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> Vec<SingleMove> {
        let mut possible_moves = Vec::new();

        if let Some(moves) = moves_by_field.get(own_position){
            for field in moves.knight_moves.iter(){
                
                if !board.get_next_player_figures().contains_key(&field) {
                    possible_moves.push(SingleMove{to: *field, promotion: None});
                }  
            }
        }
        possible_moves
    }

    pub fn threatened_fields(&self, own_position: &usize, moves_by_field: &FxHashMap<usize, MoveInEveryDirection>) -> Vec<usize>{
        if let Some(moves) = moves_by_field.get(own_position){
            return moves.knight_moves.to_owned()
        }
        return Vec::new();
    }
}


#[cfg(test)]
mod tests{
    use bitmaps::Bitmap;

    use crate::helper::moves_by_field::get_moves_for_each_field;

    use super::*;

    #[test]
    fn test_empty_board(){
        let possible_moves = get_moves_for_each_field();
        let figure = Knight {
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &27, &possible_moves);
        assert_eq!(8, moves.len());

        let moves = figure.possible_moves(&board, &0, &possible_moves);
        assert_eq!(2, moves.len());

        let moves = figure.possible_moves(&board, &54, &possible_moves);
        assert_eq!(4, moves.len());

    }
}