use std::{collections::HashMap, usize};

use bitmaps::Bitmap;
use regex::Regex;

use crate::figures::{figures::Figure, blablabla::Pawn, color::Color};

impl Default for Chessboard{
    fn default() -> Chessboard {
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            figures:  HashMap::new()
        };
        board.set_to_default();
        board
    }
}

impl Chessboard{

    pub fn make_move(&mut self, mov: &str){
        let validated_move = self.validate_string_position(mov);
        if validated_move.is_none(){
            return;
        }
        let (from_row, from_column, to_row, to_column) = validated_move.unwrap();

        let old_field  = self.get_position_id(from_row, from_column);
        let new_field  = self.get_position_id(to_row, to_column);

        self.move_figure(old_field, new_field);
    }

    pub fn move_figure(&mut self, from: usize, to: usize){
        self.positions.set(from.into(), false);
        self.positions.set(to.into(), true);

        self.figures.remove(&to);
        let mut moved_figure = self.figures.remove(&from).unwrap();
        moved_figure.set_moved();
        self.figures.insert(to, moved_figure);
    }

    fn validate_string_position<'a>(&'a self, mov: &'a str) -> Option<(&'a str, u8, &'a str, u8)>{
        let valid_move_regex = Regex::new(r"\A[abcdefgh][1-8][abcdefgh][1-8]").unwrap();
        let valid_move = valid_move_regex.captures(mov);

        if valid_move.is_none(){
            return None;
        }
        let split_move_regex = Regex::new(r"((\S)(\S)(\S)(\S))").unwrap();
        let split_moves = split_move_regex.captures(mov).unwrap();
        return Some((
                split_moves.get(2).unwrap().as_str(),
                split_moves.get(3).unwrap().as_str().parse::<u8>().unwrap(),
                split_moves.get(4).unwrap().as_str(),
                split_moves.get(5).unwrap().as_str().parse::<u8>().unwrap()
            ))
    }

    fn get_position_id(&self, row: &str, column: u8) -> usize{
        usize::from(self.get_row_from_string(row) + ((column -1) *8) -1)
    }

    fn get_row_from_string(&self, row: &str) -> u8{
        match row {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4,
            "e" => 5,
            "f" => 6,
            "g" => 7,
            "h" => 8,
            _ => 0
        }
    }

    pub fn is_in_a_row(&self, field: &usize) -> bool{
        field % 8 == 0
    } 

    pub fn is_in_h_row(&self, field: &usize) -> bool{
        field % 8 == 7
    }

    pub fn set_to_default(&mut self){
        self.positions = Bitmap::<64>::new();
        self.figures = HashMap::new();

        for n in 0..16{
            self.positions.set(n, true);
        }
        for n in 47..64{
            self.positions.set(n, true);
        }
    
        self.figures.insert(0, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(1, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(2, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(3, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(4, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(5, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(6, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(7, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(8, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(9, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(10, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(11, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(12, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(13, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(14, Figure::Pawn(Pawn{..Default::default()}));
        self.figures.insert(15, Figure::Pawn(Pawn{..Default::default()}));

        // testing
        self.figures.insert(16, Figure::Pawn(Pawn{color: Color::Black,..Default::default()}));
        self.positions.set(16, true);
        
    }
}

pub struct Chessboard {
    pub positions: Bitmap<64>,
    pub figures: HashMap<usize, Figure>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_row(){
        let board = Chessboard{..Default::default()};
        assert_eq!(true, board.is_in_a_row(&8));
        assert_eq!(true, board.is_in_a_row(&56));
        assert_eq!(true, board.is_in_a_row(&32));
        assert_eq!(false, board.is_in_a_row(&25));
        assert_eq!(false, board.is_in_a_row(&30));
    }

    #[test]
    fn test_h_row(){
        let board = Chessboard{..Default::default()};
        assert_eq!(true, board.is_in_h_row(&7));
        assert_eq!(true, board.is_in_h_row(&31));
        assert_eq!(true, board.is_in_h_row(&39));
        assert_eq!(false, board.is_in_h_row(&18));
        assert_eq!(false, board.is_in_h_row(&38));
    }
}