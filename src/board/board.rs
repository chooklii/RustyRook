use std::{collections::HashMap, usize};

use bitmaps::Bitmap;
use regex::Regex;

use crate::{figures::{bishop::Bishop, color::Color, figures::Figure, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook}, helper::movement::{figure_can_move_left, figure_can_move_right}};


#[derive(Clone)]
pub struct Chessboard {
    pub positions: Bitmap<64>,
    pub white_figures: HashMap<usize, Figure>,
    pub black_figures: HashMap<usize, Figure>,
    pub current_move: Color,
    // possible field with figure that can be taken en passant
    pub en_passant: Option<usize>
}

impl Default for Chessboard{
    fn default() -> Chessboard {
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
            en_passant: None
        };
        board.set_to_default();
        board
    }
}

impl Chessboard{

    // convert board to unique position key 
    pub fn position_key(&self) -> String{
        let mut key = String::new();
        for n in 0..63{
            if !self.positions.get(n){
                key.push_str("0");
            }else if self.white_figures.contains_key(&n) {
                key.push_str("W");  
                key.push_str(&self.white_figures.get(&n).unwrap().get_name());
            }else if self.black_figures.contains_key(&n) {
                key.push_str("B");
                key.push_str(&self.black_figures.get(&n).unwrap().get_name());  
            }
        }
        key
    }

    fn set_current_move(&mut self){
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

    pub fn update_position_from_uci_input(&mut self, mov: &str){
        if let Some((from_row, from_column, to_row, to_column)) = self.validate_string_position(mov){
            let old_field  = self.get_position_id(from_row, from_column);
            let new_field  = self.get_position_id(to_row, to_column);
            
            self.move_figure(old_field, new_field);            
        }
    }

    fn possible_future_en_passant(&mut self, old_field: usize, new_field: usize){
        match self.current_move{
            Color::White => self.possible_white_en_passant(old_field, new_field),
            Color::Black => self.possible_black_en_passant(old_field, new_field)
        }
    }

    fn possible_white_en_passant(&mut self, old_field: usize, new_field: usize){
        if new_field == old_field +16 {
            if let Some(figure) = self.white_figures.get(&old_field){
                if figure.is_pawn(){
                    self.en_passant = Some(new_field);
                    return;
                }
            }
        }
        self.en_passant = None;
    }

    fn possible_black_en_passant(&mut self, old_field: usize, new_field: usize){
        if old_field == new_field +16 {
            if let Some(figure) = self.black_figures.get(&old_field){
                if figure.is_pawn(){
                    self.en_passant = Some(new_field);
                    return;
                }
            }
        }
        self.en_passant = None;
    }

    fn check_and_execute_en_passant(&mut self, old_field: usize, new_field: usize){
        // no possible en passant no need to check any longer
        if self.en_passant.is_none(){
            return;
        }
        let is_en_passant = match self.current_move{
            Color::White => self.is_en_passant_white(old_field, new_field),
            Color::Black => self.is_en_passant_black(old_field, new_field)
        };
        if !is_en_passant{
            return;
        }
        // we have none checked en_passant at this point
        let en_passanted_figure = self.en_passant.unwrap();
        self.positions.set(en_passanted_figure, false);
        match self.current_move{
            Color::Black => self.white_figures.remove(&en_passanted_figure),
            Color::White => self.black_figures.remove(&en_passanted_figure)
        };
    }

    fn is_en_passant_black(&mut self, old_field: usize, new_field: usize) -> bool{
        if let Some(possible_en_passant_field) = self.en_passant{
            // post en passant would be one column less than possible en passant"ed" pawn
            if possible_en_passant_field -8 != new_field{
                return false;
            }
            if figure_can_move_left(&old_field) && possible_en_passant_field -1 == old_field{
                return true;
            }
            if figure_can_move_right(&old_field) && possible_en_passant_field +1 == old_field {
                return true;
            }
            }
            return false;
        }

    fn is_en_passant_white(&mut self, old_field: usize, new_field: usize) -> bool{
        if let Some(possible_en_passant_field) = self.en_passant{
            // post en passant would be one column more than possible en passant"ed" pawn
            if possible_en_passant_field +8 != new_field{
                return false;
            }
            if figure_can_move_left(&old_field) && possible_en_passant_field +1 == old_field{
                return true;
            }
            if figure_can_move_right(&old_field) && possible_en_passant_field -1 == old_field {
                return true;
            }
            }
            return false;
        }

    fn castle(&mut self, old_field: usize, new_field: usize){
        // not white - nor black castle
        // white castle 4 -> 2/6 || black castle 60 -> 58/62
        if !((old_field == 4 && (new_field == 2 || new_field == 6)) || ((old_field == 60) && (new_field == 58 || new_field == 62))){
            return
        }
        if old_field == 4{
            self.white_castle(old_field, new_field);
        }
        else if old_field == 60{
            self.black_castle(old_field, new_field);
        }
    }

    fn black_castle(&mut self, old_field: usize, new_field: usize){
        if let Some(figure) = self.black_figures.get(&old_field){
            if figure.is_king(){
                // lange rutsche
                if new_field == 58{
                    self.move_black_figure(56, 59);
                    self.positions.set(56, false);
                    self.positions.set(59, true);
                }
                if new_field == 62{
                    self.move_black_figure(63, 61);
                    self.positions.set(63, false);
                    self.positions.set(61, true);
                }
            }
        }
    }

    fn white_castle(&mut self, old_field: usize, new_field: usize){
        if let Some(figure) = self.white_figures.get(&old_field){
            if figure.is_king(){
                if new_field == 2{
                    self.move_white_figure(0, 3);
                    self.positions.set(0, false);
                    self.positions.set(3, true);
                }
                if new_field == 6{
                    self.move_white_figure(7, 5);
                    self.positions.set(7, false);
                    self.positions.set(5, true);
                }
            }
        }
    }

    pub fn move_figure(&mut self, from: usize, to: usize){
        // if move is caste move rook as well
        self.castle(from, to);
        // if move is en passant remove opponent (from field we did not move to!)
        self.check_and_execute_en_passant(from, to);
        // check for possible future en_passant options
        self.possible_future_en_passant(from, to);
        self.positions.set(from, false);
        self.positions.set(to, true);

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
        // first validate that input is in valid format - then split it into x/y for both positions (new and old)
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

    pub fn set_to_default(&mut self){
        self.positions = Bitmap::<64>::new();
        self.black_figures = HashMap::new();
        self.white_figures = HashMap::new();
        self.current_move = Color::White;

         
        for n in 0..16{
            self.positions.set(n, true);
        }
        
        for n in 48..63{
            self.positions.set(n, true);
        }
        
        
        // white
        self.white_figures.insert(0, Figure::Rook(Rook{..Default::default()}));
        self.white_figures.insert(1, Figure::Knight(Knight{..Default::default()}));
        self.white_figures.insert(2, Figure::Bishop(Bishop{..Default::default()}));
        self.white_figures.insert(3, Figure::Queen(Queen{..Default::default()}));
        self.white_figures.insert(4, Figure::King(King{..Default::default()}));
        self.white_figures.insert(5, Figure::Bishop(Bishop{..Default::default()}));
        self.white_figures.insert(6, Figure::Knight(Knight{..Default::default()}));
        self.white_figures.insert(7, Figure::Rook(Rook{..Default::default()}));
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
        self.black_figures.insert(56, Figure::Rook(Rook{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(57, Figure::Knight(Knight{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(58, Figure::Bishop(Bishop{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(59, Figure::Queen(Queen{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(60, Figure::King(King{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(61, Figure::Bishop(Bishop{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(62, Figure::Knight(Knight{color: Color::Black, ..Default::default()}));
        self.black_figures.insert(63, Figure::Rook(Rook{color: Color::Black, ..Default::default()}));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_castle_white(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
            en_passant: None
        };

        board.white_figures.insert(0, Figure::Rook(Rook{..Default::default()}));
        board.white_figures.insert(4, Figure::King(King{..Default::default()}));
        board.white_figures.insert(7, Figure::Rook(Rook{..Default::default()}));
        board.positions.set(0, true);
        board.positions.set(4, true);
        board.positions.set(7, true);

        board.update_position_from_uci_input("e1g1");
        assert_eq!(board.positions.get(6), true);
        assert_eq!(board.positions.get(5), true);
        assert_eq!(board.positions.get(7), false);
        assert_eq!(board.positions.get(4), false);
    }

    #[test]
    fn long_castle_white(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
            en_passant: None
        };

        board.white_figures.insert(0, Figure::Rook(Rook{..Default::default()}));
        board.white_figures.insert(4, Figure::King(King{..Default::default()}));
        board.white_figures.insert(7, Figure::Rook(Rook{..Default::default()}));
        board.positions.set(0, true);
        board.positions.set(4, true);
        board.positions.set(7, true);

        board.update_position_from_uci_input("e1c1");
        assert_eq!(board.positions.get(3), true);
        assert_eq!(board.positions.get(2), true);
        assert_eq!(board.positions.get(0), false);
        assert_eq!(board.positions.get(4), false);
    }

    #[test]
    fn short_castle_black(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::Black,
            en_passant: None
        };

        board.black_figures.insert(56, Figure::Rook(Rook{..Default::default()}));
        board.black_figures.insert(60, Figure::King(King{..Default::default()}));
        board.black_figures.insert(63, Figure::Rook(Rook{..Default::default()}));
        board.positions.set(56, true);
        board.positions.set(60, true);
        board.positions.set(63, true);

        board.update_position_from_uci_input("e8c8");
        assert_eq!(board.positions.get(58), true);
        assert_eq!(board.positions.get(59), true);
        assert_eq!(board.positions.get(56), false);
        assert_eq!(board.positions.get(60), false);
    }

    #[test]
    fn long_castle_black(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::Black,
            en_passant: None
        };

        board.black_figures.insert(56, Figure::Rook(Rook{..Default::default()}));
        board.black_figures.insert(60, Figure::King(King{..Default::default()}));
        board.black_figures.insert(63, Figure::Rook(Rook{..Default::default()}));
        board.positions.set(56, true);
        board.positions.set(60, true);
        board.positions.set(63, true);

        board.update_position_from_uci_input("e8g8");
        assert_eq!(board.positions.get(61), true);
        assert_eq!(board.positions.get(62), true);
        assert_eq!(board.positions.get(60), false);
        assert_eq!(board.positions.get(63), false);
    }

    #[test]
    fn test_en_passant(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::Black,
            en_passant: None
        };

        board.black_figures.insert(52, Figure::Pawn(Pawn{..Default::default()}));
        board.positions.set(52, true);
        board.move_figure(52, 36);

        assert_eq!(board.en_passant, Some(36));
    }

    #[test]
    fn test_no_en_passant(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::Black,
            en_passant: None
        };

        board.black_figures.insert(52, Figure::Pawn(Pawn{..Default::default()}));
        board.positions.set(52, true);
        board.move_figure(52, 44);

        assert_eq!(board.en_passant, None);
    }

    #[test]
    fn test_if_en_passanted_figure_is_removed_black(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::Black,
            en_passant: Some(26)
        }; 
        board.white_figures.insert(26, Figure::Pawn(Pawn{..Default::default()}));
        board.black_figures.insert(25, Figure::Pawn(Pawn { color: Color::Black, ..Default::default()}));

        board.move_figure(25, 18);

        assert_eq!(0, board.white_figures.len());
        assert_eq!(false, board.positions.get(26))
    }

    #[test]
    fn test_if_en_passanted_figure_is_removed_white(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::White,
            en_passant: Some(36)
        }; 
        board.black_figures.insert(36, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        board.white_figures.insert(35, Figure::Pawn(Pawn { ..Default::default()}));
        board.move_figure(35, 44);

        assert_eq!(0, board.black_figures.len());
        assert_eq!(false, board.positions.get(36))
    }
}