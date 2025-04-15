use std::usize;

use bitmaps::Bitmap;
use regex::Regex;
use rustc_hash::FxHashMap;

use crate::{figures::{bishop::Bishop, color::Color, figures::Figure, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook}, helper::movement::{figure_can_move_left, figure_can_move_right}};

use super::promotion::{convert_input_string_to_promotion, convert_promotion_to_figure, Promotion};


#[derive(Clone)]
pub struct Chessboard {
    pub positions: Bitmap<64>,
    pub white_figures: FxHashMap<usize, Figure>,
    pub black_figures: FxHashMap<usize, Figure>,
    pub current_move: Color,
    // possible field with figure that can be taken en passant
    pub en_passant: Option<usize>
}

impl Default for Chessboard{
    fn default() -> Chessboard {
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: None
        };
        board.set_to_default();
        board
    }
}

impl Chessboard{

    fn set_current_move(&mut self){
        match self.current_move{
            Color::Black => self.current_move = Color::White,
            Color::White => self.current_move = Color::Black
        }
    }


    pub fn get_opponents(&self) -> &FxHashMap<usize, Figure>{
        match self.current_move{
            Color::White => &self.black_figures,
            Color::Black => &self.white_figures
        }
    }


    pub fn get_next_player_figures(&self) -> &FxHashMap<usize, Figure>{
        match self.current_move{
            Color::White => &self.white_figures,
            Color::Black => &self.black_figures
        } 
    }

    pub fn update_position_from_uci_input(&mut self, mov: &str){
        if let Some((from_row, from_column, to_row, to_column, promoted_to_piece)) = self.validate_string_position(mov){
            let old_field  = self.get_position_id(from_row, from_column);
            let new_field  = self.get_position_id(to_row, to_column);
            
            let promoted_figure = convert_input_string_to_promotion(promoted_to_piece);
            self.move_figure(old_field, new_field, promoted_figure);         
        }
    }

    // exchange old field with promoted to figure
    fn update_figure_to_promoted_one(&mut self, old_field: usize, promoted_figure: Promotion){
        let new_piece = convert_promotion_to_figure(promoted_figure);
        match self.current_move{
            Color::White => self.white_figures.insert(old_field, new_piece),
            Color::Black => self.black_figures.insert(old_field, new_piece)
        };
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
        if let Some(figure) = self.get_next_player_figures().get(&old_field){
            // check if figure moving is actually a pawn
            if !figure.is_pawn(){
                return
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

    pub fn move_figure(&mut self, from: usize, to: usize, promoted_to: Option<Promotion>){
        // if move is caste move rook as well
        self.castle(from, to);
        // if move is en passant remove opponent (from field we did not move to!)
        self.check_and_execute_en_passant(from, to);
        // check for possible future en_passant options
        self.possible_future_en_passant(from, to);

        if let Some(promoted_figure) = promoted_to{
            self.update_figure_to_promoted_one(from, promoted_figure);
        };
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

    fn validate_string_position<'a>(&'a self, mov: &'a str) -> Option<(&'a str, u8, &'a str, u8, Option<&'a str>)>{
        // first validate that input is in valid format - then split it into x/y for both positions (new and old)
        let valid_move_regex = Regex::new(r"\A[abcdefgh][1-8][abcdefgh][1-8]([qrbkQrbK]?)").unwrap();
        let valid_move = valid_move_regex.captures(mov);

        if valid_move.is_none(){
            return None;
        }

        //todo refactor
        let valid_move_unpacked = valid_move.unwrap().get(1);
        let promoted_to_piece = if !valid_move_unpacked.unwrap().is_empty(){
            Some(valid_move_unpacked.unwrap().as_str())
        } else{None};

        let split_move_regex = Regex::new(r"((\S)(\S)(\S)(\S))").unwrap();
        let split_moves = split_move_regex.captures(mov).unwrap();
        return Some((
                split_moves.get(2).unwrap().as_str(),
                split_moves.get(3).unwrap().as_str().parse::<u8>().unwrap(),
                split_moves.get(4).unwrap().as_str(),
                split_moves.get(5).unwrap().as_str().parse::<u8>().unwrap(),
                promoted_to_piece
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
        self.black_figures = FxHashMap::default();
        self.white_figures = FxHashMap::default();
        self.current_move = Color::White;

        // https://www.chessprogramming.org/Perft_Results
        if false{
            let position_2 = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
            self.create_position_from_input_string(position_2);
            return;
        }
        if false{
            let position_3 = String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8");
            self.create_position_from_input_string(position_3);
            return;
        }
        if false{
            let position_4 = String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1");
            self.create_position_from_input_string(position_4);
            // todo: include castle rights from position fen
            let mut white_king = self.white_figures.remove(&6).unwrap();
            white_king.set_moved();
            self.white_figures.insert(6, white_king);
            return
        }
        if false{
            let position_5 = String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R");
            self.create_position_from_input_string(position_5);
            return
        }
        if false{
            let position = String::from("8/pR1r3k/2pN2pp/3NQ3/2B5/8/P1q2PPP/5KK1");
            self.create_position_from_input_string(position);
            return;
        }

        let default_position = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        self.create_position_from_input_string(default_position);

    }


    // e.g. "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8"
    fn create_position_from_input_string(&mut self, position: String){
        let mut current_position: usize = 56;

        for c in position.chars() { 
            if c == '/'{
                current_position=current_position - 16;
            }
            if c.is_digit(10){
                // can u feel the magic? :D
                let as_digit = usize::from(c as u8 - 0x30);
                current_position=current_position + as_digit;
            }
            if c.is_alphabetic(){
                self.positions.set(current_position, true);
                if c.is_lowercase(){
                    self.black_figures.insert(current_position, self.get_figure_from_char(c, true));
                }
                else{
                    self.white_figures.insert(current_position, self.get_figure_from_char(c, false));
                }
                current_position = current_position +1;
            }

        }
    }

    fn get_figure_from_char(&self, figure: char, is_black: bool) -> Figure{
        let fig = figure.to_uppercase().to_string();
        
        if fig.eq("K"){
            if is_black{
                return Figure::King(King{color: Color::Black, ..Default::default()});
            }
            return Figure::King(King{..Default::default()});
        }
        if fig.eq("P"){
            if is_black{
                return Figure::Pawn(Pawn{color: Color::Black,..Default::default()});
            }
            return Figure::Pawn(Pawn{..Default::default()});
        }
        if fig.eq("Q"){
            return Figure::Queen(Queen{..Default::default()});
        }
        if fig.eq("B"){
            return Figure::Bishop(Bishop{..Default::default()});
        }
        if fig.eq("R"){
            return Figure::Rook(Rook{..Default::default()});
        }
        return Figure::Knight(Knight{..Default::default()})

    }
}


#[cfg(test)]
mod tests {
    use crate::{engine::count::count_moves, helper::moves_by_field::get_moves_for_each_field};

    use super::*;

    #[test]
    fn short_castle_white(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
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
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
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
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
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
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
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
    fn promotion_black(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::Black,
            en_passant: None
        };

        board.black_figures.insert(14, Figure::Pawn(Pawn { color: Color::Black }));
        board.positions.set(14, true);

        board.update_position_from_uci_input("g2g1q");
        assert_eq!(board.positions.get(6), true);
        assert_eq!(true, board.black_figures.get(&6).unwrap().is_queen());
    }

    #[test]
    fn promotion_white(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: None
        };

        board.white_figures.insert(52, Figure::Pawn(Pawn { color: Color::White }));
        board.positions.set(52, true);

        board.update_position_from_uci_input("e7e8K");
        assert_eq!(board.positions.get(60), true);
        assert_eq!(true, board.white_figures.get(&60).unwrap().is_knight());
    }

    #[test]
    fn test_en_passant(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::Black,
            en_passant: None
        };

        board.black_figures.insert(52, Figure::Pawn(Pawn{..Default::default()}));
        board.positions.set(52, true);
        board.move_figure(52, 36, None);

        assert_eq!(board.en_passant, Some(36));
    }

    #[test]
    fn test_no_en_passant(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::Black,
            en_passant: None
        };

        board.black_figures.insert(52, Figure::Pawn(Pawn{..Default::default()}));
        board.positions.set(52, true);
        board.move_figure(52, 44, None);

        assert_eq!(board.en_passant, None);
    }

    #[test]
    fn test_if_en_passanted_figure_is_removed_black(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::Black,
            en_passant: Some(26)
        }; 
        board.white_figures.insert(26, Figure::Pawn(Pawn{..Default::default()}));
        board.black_figures.insert(25, Figure::Pawn(Pawn { color: Color::Black, ..Default::default()}));

        board.move_figure(25, 18, None);

        assert_eq!(0, board.white_figures.len());
        assert_eq!(false, board.positions.get(26))
    }

    #[test]
    fn test_if_en_passanted_figure_is_removed_white(){
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: Some(36)
        }; 
        board.black_figures.insert(36, Figure::Pawn(Pawn{color: Color::Black, ..Default::default()}));
        board.white_figures.insert(35, Figure::Pawn(Pawn { ..Default::default()}));
        board.move_figure(35, 44, None);

        assert_eq!(0, board.black_figures.len());
        assert_eq!(false, board.positions.get(36))
    }

    #[test]
    fn test_position_creation(){
        let position = String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8");
        let mut board = Chessboard{..Default::default()};
        board.create_position_from_input_string(position);

        assert_eq!(true, board.black_figures.contains_key(&31));
        assert_eq!(true, board.black_figures.contains_key(&39));
        assert_eq!(false, board.black_figures.contains_key(&38));

        assert_eq!(true, board.white_figures.contains_key(&14));
        assert_eq!(true, board.white_figures.contains_key(&33));
        assert_eq!(false, board.white_figures.contains_key(&34));
    }

    
    #[test]
    fn test_default_position(){
        let board = Chessboard{..Default::default()};
        let moves_by_field = get_moves_for_each_field();
        let count = count_moves(&board, &moves_by_field, 4);
        assert_eq!(197281, count);
    }

    #[test]
    #[ignore]
    fn test_position_2(){
        let mut board = Chessboard{            
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: None};
        let moves_by_field = get_moves_for_each_field();

        let position_2 = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
        board.create_position_from_input_string(position_2);
        let count = count_moves(&board, &moves_by_field, 4);
        assert_eq!(4085603, count);
    }

    #[test]
    #[ignore]
    fn test_position_3(){
        let mut board = Chessboard{            
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: None};
        let moves_by_field = get_moves_for_each_field();

        let position_3 = String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8");
        board.create_position_from_input_string(position_3);
        let count = count_moves(&board, &moves_by_field, 5);
        assert_eq!(674624, count);
    }

    #[test]
    #[ignore]
    fn test_position_4(){
        let mut board = Chessboard{            
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: None};
        let moves_by_field = get_moves_for_each_field();

        let position_4 = String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1");
        board.create_position_from_input_string(position_4);
        let count = count_moves(&board, &moves_by_field, 4);
        assert_eq!(422333, count);
    }

    #[test]
    #[ignore]
    fn test_position_5(){
        let mut board = Chessboard{            
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: None};
        let moves_by_field = get_moves_for_each_field();

        let position_5 = String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R");
        board.create_position_from_input_string(position_5);
        let count = count_moves(&board, &moves_by_field, 4);
        assert_eq!(2103487, count);
    }

    #[test]
    #[ignore]
    fn test_position_6(){
        let mut board = Chessboard{            
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::White,
            en_passant: None};
        let moves_by_field = get_moves_for_each_field();

        let position_6 = String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1");
        board.create_position_from_input_string(position_6);

        let mut white_king = board.white_figures.remove(&6).unwrap();
        white_king.set_moved();
        board.white_figures.insert(6, white_king);

        let mut black_king = board.black_figures.remove(&62).unwrap();
        black_king.set_moved();
        board.black_figures.insert(62, black_king);

        let count = count_moves(&board, &moves_by_field, 4);
        assert_eq!(3894594, count);
    }
}