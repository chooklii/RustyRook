use std::{collections::HashMap, usize};

use bitmaps::Bitmap;
use regex::Regex;

use crate::figures::{color::Color, figures::Figure, king::King, knight::Knight, pawn::Pawn, queen::Queen, rock::Rock, Bishop::Bishop};


#[derive(Clone)]
pub struct Chessboard {
    pub positions: Bitmap<64>,
    pub white_figures: HashMap<usize, Figure>,
    pub black_figures: HashMap<usize, Figure>,
    pub current_move: Color
}

impl Default for Chessboard{
    fn default() -> Chessboard {
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures:  HashMap::new(),
            black_figures:  HashMap::new(),
            current_move: Color::White
        };
        board.set_to_default();
        board
    }
}

impl Chessboard{

    pub fn set_current_move(&mut self){
        match self.current_move{
            Color::Black => self.current_move = Color::White,
            Color::White => self.current_move = Color::Black
        }
    }

    // color from here can prob. be removed and just use by currentMove Color (no need to calculate for other side?)
    pub fn get_opponents(&self, color: &Color) -> &HashMap<usize, Figure>{
        match color{
            Color::White => &self.black_figures,
            Color::Black => &self.white_figures
        }
    }

    pub fn get_next_player_figures(&self) -> &HashMap<usize, Figure>{
        match self.current_move{
            Color::White => &self.white_figures,
            Color::Black => &self.black_figures
        } 
    }

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

        match self.current_move {
            Color::White => self.move_white_figure(from, to),
            Color::Black => self.move_black_figure(from, to),
        }
        self.set_current_move();
    }

    fn move_black_figure(&mut self, from: usize, to: usize){
        self.white_figures.remove(&to);
        let mut moved_figure = self.black_figures.remove(&from).unwrap();
        moved_figure.set_moved();
        self.black_figures.insert(to, moved_figure);
    }

    fn move_white_figure(&mut self, from: usize, to: usize){
        self.black_figures.remove(&to);
        let mut moved_figure = self.white_figures.remove(&from).unwrap();
        moved_figure.set_moved();
        self.white_figures.insert(to, moved_figure);
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

    pub fn figure_can_move_left(&self, field: &usize) -> bool{
        field % 8 != 0
    } 

    pub fn figure_can_move_right(&self, field: &usize) -> bool{
        field % 8 != 7
    }

    pub fn figure_can_move_forward(&self, field: &usize) -> bool{
        field <= &55
    }

    pub fn figure_can_move_backward(&self, field: &usize) -> bool{
        field >=&8
    }

    pub fn set_to_default(&mut self){
        self.positions = Bitmap::<64>::new();
        self.black_figures = HashMap::new();
        self.white_figures = HashMap::new();
        self.current_move = Color::White;

         
        for n in 0..16{
            self.positions.set(n, true);
        }
        
        for n in 47..63{
            self.positions.set(n, true);
        }
        
        
        // white
        self.white_figures.insert(0, Figure::Rock(Rock{..Default::default()}));
        self.white_figures.insert(1, Figure::Knight(Knight{..Default::default()}));
        self.white_figures.insert(2, Figure::Bishop(Bishop{..Default::default()}));
        self.white_figures.insert(3, Figure::Queen(Queen{..Default::default()}));
        self.white_figures.insert(4, Figure::King(King{..Default::default()}));
        self.white_figures.insert(5, Figure::Bishop(Bishop{..Default::default()}));
        self.white_figures.insert(6, Figure::Knight(Knight{..Default::default()}));
        self.white_figures.insert(7, Figure::Rock(Rock{..Default::default()}));
        self.white_figures.insert(8, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(9, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(10, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(11, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(12, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(13, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(14, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(15, Figure::Pawn(Pawn{..Default::default()}));

        // black
        self.black_figures.insert(48, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(49, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(50, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(51, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(52, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(53, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(54, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(55, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(56, Figure::Rock(Rock{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(57, Figure::Knight(Knight{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(58, Figure::Bishop(Bishop{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(59, Figure::Queen(Queen{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(60, Figure::King(King{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(61, Figure::Bishop(Bishop{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(62, Figure::Knight(Knight{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(63, Figure::Rock(Rock{color: Color::Black, ..Default::default()}));
        
        
        // testing

        /*
        self.white_figures.insert(10, Figure::Pawn(Pawn{..Default::default()}));
        self.white_figures.insert(46, Figure::Pawn(Pawn{..Default::default()}));
        self.black_figures.insert(19, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(55, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        self.positions.set(19, true);
        self.positions.set(55, true);
        self.positions.set(10, true);
        self.positions.set(46, true);
        */
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_left(){
        let board = Chessboard{..Default::default()};
        assert_eq!(false, board.figure_can_move_left(&8));
        assert_eq!(true, board.figure_can_move_left(&15));
        assert_eq!(false, board.figure_can_move_left(&56));
        assert_eq!(false, board.figure_can_move_left(&32));
        assert_eq!(true, board.figure_can_move_left(&25));
        assert_eq!(true, board.figure_can_move_left(&30));
    }

    #[test]
    fn test_move_right(){
        let board: Chessboard = Chessboard{..Default::default()};
        assert_eq!(false, board.figure_can_move_right(&7));
        assert_eq!(false, board.figure_can_move_right(&15));
        assert_eq!(false, board.figure_can_move_right(&31));
        assert_eq!(false, board.figure_can_move_right(&39));
        assert_eq!(true, board.figure_can_move_right(&18));
        assert_eq!(true, board.figure_can_move_right(&38));
        assert_eq!(true, board.figure_can_move_right(&16));
    }

    #[test]
    fn test_move_forward(){
        let board = Chessboard{..Default::default()};
        assert_eq!(true, board.figure_can_move_forward(&27));
        assert_eq!(true, board.figure_can_move_forward(&27));
        assert_eq!(true, board.figure_can_move_forward(&0));
        assert_eq!(true, board.figure_can_move_forward(&0));
        assert_eq!(false, board.figure_can_move_forward(&60));
    }

    #[test]
    fn test_move_backward(){
        let board = Chessboard{..Default::default()};
        assert_eq!(true, board.figure_can_move_backward(&27));
        assert_eq!(true, board.figure_can_move_backward(&27));
        assert_eq!(false, board.figure_can_move_backward(&0));
        assert_eq!(false, board.figure_can_move_backward(&0));
        assert_eq!(true, board.figure_can_move_backward(&60)); 
    }
}