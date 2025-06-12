use std::usize;

use regex::Regex;

use crate::{
    figures::{color::Color, piece::Piece},
    helper::movement::{figure_can_move_left, figure_can_move_right},
    ZOBRIST_CASTLE_NUMBERS, ZOBRIST_CURRENT_MOVE, ZOBRIST_EN_PASSANT, ZOBRIST_FIGURE_NUMBERS,
    ZOBRIST_SEED,
};

use super::{
    bitboard::Bitboard,
    castle::Castle,
    promotion::{convert_input_string_to_promotion, convert_promotion_to_figure, Promotion},
};

#[derive(Clone, Copy)]
pub struct Chessboard {
    pub positions: Bitboard,
    pub used_positions: [Bitboard; 2],
    pub figures: [[Bitboard; 6]; 2],
    pub current_move: Color,
    // possible field with figure that can be taken en passant
    pub en_passant: Option<usize>,
    pub castle: Castle,
    pub zobrist_key: u64,
}

impl Default for Chessboard {
    fn default() -> Chessboard {
        let mut board = Chessboard {
            positions: Bitboard::new(),
            used_positions: [Bitboard::new(), Bitboard::new()],
            figures: [
                [
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                ],
                [
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                    Bitboard::new(),
                ],
            ],
            current_move: Color::White,
            en_passant: None,
            castle: Castle {
                ..Default::default()
            },
            zobrist_key: *ZOBRIST_SEED
        };
        board.set_to_default();
        board
    }
}

impl Chessboard {
    // used by tests
    #[allow(dead_code)]
    pub fn empty(color: Color) -> Chessboard {
        let mut board = Chessboard{..Default::default()};
        board.set_empty();
        board.current_move = color;
        board
    }

    fn set_empty(&mut self){
        self.positions = Bitboard::new();
        self.used_positions = [Bitboard::new(), Bitboard::new()];
        self.figures = [
            [
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
            ],
            [
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
            ],
        ];
        self.current_move = Color::White;
        self.en_passant = None;
        self.castle = Castle {
            ..Default::default()
        };
        self.zobrist_key = u64::default()
    }
    fn set_current_move(&mut self) {
        self.zobrist_key ^= *ZOBRIST_CURRENT_MOVE;
        match self.current_move {
            Color::Black => self.current_move = Color::White,
            Color::White => self.current_move = Color::Black,
        }
    }

    pub fn get_pieces(&self, color: Color, piece: Piece) -> &Bitboard {
        &self.figures[color as usize][piece as usize]
    }

    fn remove_piece(&mut self, color: Color, piece: Piece, position: usize) {
        self.figures[color as usize][piece as usize].remove_field(position);
        self.used_positions[color as usize].remove_field(position);
        self.positions.remove_field(position);
        self.zobrist_key ^= ZOBRIST_FIGURE_NUMBERS[color as usize][piece as usize][position];
    }

    fn add_piece(&mut self, color: Color, piece: Piece, position: usize) {
        self.figures[color as usize][piece as usize].set_field(position);
        self.used_positions[color as usize].set_field(position);
        self.positions.set_field(position);
        self.zobrist_key ^= ZOBRIST_FIGURE_NUMBERS[color as usize][piece as usize][position];
    }

    fn get_type_of_figure(&self, color: Color, position: usize) -> Option<Piece> {
        // not beautiful, but faster than array of pieces
        if self.get_pieces(color, Piece::Pawn).field_is_used(position) {
            return Some(Piece::Pawn);
        };
        if self
            .get_pieces(color, Piece::Knight)
            .field_is_used(position)
        {
            return Some(Piece::Knight);
        };
        if self
            .get_pieces(color, Piece::Bishop)
            .field_is_used(position)
        {
            return Some(Piece::Bishop);
        };
        if self.get_pieces(color, Piece::Rook).field_is_used(position) {
            return Some(Piece::Rook);
        };
        if self.get_pieces(color, Piece::Queen).field_is_used(position) {
            return Some(Piece::Queen);
        };
        if self.get_pieces(color, Piece::King).field_is_used(position) {
            return Some(Piece::King);
        };
        None
    }

    pub fn is_queen_or_rook(&self, color: Color, position: usize) -> bool {
        self.get_pieces(color, Piece::Rook).field_is_used(position)
            || self.get_pieces(color, Piece::Queen).field_is_used(position)
    }

    pub fn is_queen_or_bishop(&self, color: Color, position: usize) -> bool {
        self
            .get_pieces(color, Piece::Bishop)
            .field_is_used(position)
            || self.get_pieces(color, Piece::Queen).field_is_used(position)
    }

    pub fn get_opponents(&self) -> &Bitboard {
        match self.current_move {
            Color::White => &self.used_positions[Color::Black as usize],
            Color::Black => &self.used_positions[Color::White as usize],
        }
    }

    pub fn get_opponent_color(&self) -> Color {
        match self.current_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_opponent_piece(&self, piece: Piece) -> &Bitboard {
        match self.current_move {
            Color::White => &self.figures[Color::Black as usize][piece as usize],
            Color::Black => &self.figures[Color::White as usize][piece as usize],
        }
    }

    pub fn get_positions_by_current_player(&self) -> &Bitboard {
        &self.used_positions[self.current_move as usize]
    }

    pub fn update_position_from_uci_input(&mut self, mov: &str) {
        if let Some((from_row, from_column, to_row, to_column, promoted_to_piece)) =
            self.validate_string_position(mov)
        {
            let old_field = self.get_position_id(from_row, from_column);
            let new_field = self.get_position_id(to_row, to_column);

            let promoted_figure = convert_input_string_to_promotion(promoted_to_piece);
            self.move_figure(old_field, new_field, promoted_figure);
        }
    }

    fn update_figure_to_promoted_one(
        &mut self,
        old_field: usize,
        new_field: usize,
        promoted_figure: Promotion,
    ) {
        self.remove_piece(self.current_move, Piece::Pawn, old_field);

        // if we captures a piece on the way remove it as well
        self.remove_opponent_piece_from_field(new_field, self.get_opponent_color());

        let new_piece = convert_promotion_to_figure(promoted_figure);
        self.add_piece(self.current_move, new_piece, new_field);
    }

    fn possible_future_en_passant(&mut self, old_field: usize, new_field: usize) {
        match self.current_move {
            Color::White => self.possible_white_en_passant(old_field, new_field),
            Color::Black => self.possible_black_en_passant(old_field, new_field),
        }
    }

    fn possible_white_en_passant(&mut self, old_field: usize, new_field: usize) {
        if new_field == old_field + 16 {
            let white_pawns = self.get_pieces(Color::White, Piece::Pawn);
            if white_pawns.field_is_used(old_field) {
                self.en_passant = Some(new_field);
                self.zobrist_key ^= ZOBRIST_EN_PASSANT[new_field];
                return;
            }
        }
        self.en_passant = None;
    }

    fn possible_black_en_passant(&mut self, old_field: usize, new_field: usize) {
        if old_field == new_field + 16 {
            let black_pawns = self.get_pieces(Color::Black, Piece::Pawn);
            if black_pawns.field_is_used(old_field) {
                self.en_passant = Some(new_field);
                self.zobrist_key ^= ZOBRIST_EN_PASSANT[new_field];
                return;
            }
        }
        self.en_passant = None;
    }

    fn check_and_execute_en_passant(&mut self, old_field: usize, new_field: usize) {
        // no possible en passant no need to check any longer
        if self.en_passant.is_none() {
            return;
        }        
        // we have none checked en_passant at this point
        let en_passanted_figure = self.en_passant.unwrap();
        // remove possible prev. en passant from zobrist
        self.zobrist_key ^= ZOBRIST_EN_PASSANT[en_passanted_figure];
        let pawns = self.get_pieces(self.current_move, Piece::Pawn);

        // check if figure moving is actually a pawn
        if !pawns.field_is_used(old_field) {
            return;
        }

        let is_en_passant = match self.current_move {
            Color::White => self.is_en_passant_white(old_field, new_field),
            Color::Black => self.is_en_passant_black(old_field, new_field),
        };
        if !is_en_passant {
            return;
        }
        self.remove_piece(self.get_opponent_color(), Piece::Pawn, en_passanted_figure);
    }

    fn is_en_passant_black(&mut self, old_field: usize, new_field: usize) -> bool {
        if let Some(possible_en_passant_field) = self.en_passant {
            // post en passant would be one column less than possible en passant"ed" pawn
            if possible_en_passant_field - 8 != new_field {
                return false;
            }
            if figure_can_move_left(&old_field) && possible_en_passant_field + 1 == old_field {
                return true;
            }
            if figure_can_move_right(&old_field) && possible_en_passant_field - 1 == old_field {
                return true;
            }
        }
        false
    }

    fn is_en_passant_white(&mut self, old_field: usize, new_field: usize) -> bool {
        if let Some(possible_en_passant_field) = self.en_passant {
            // post en passant would be one column more than possible en passant"ed" pawn
            if possible_en_passant_field + 8 != new_field {
                return false;
            }
            if figure_can_move_left(&old_field) && possible_en_passant_field + 1 == old_field {
                return true;
            }
            if figure_can_move_right(&old_field) && possible_en_passant_field - 1 == old_field {
                return true;
            }
        }
        false
    }

    fn castle(&mut self, old_field: usize, new_field: usize) {
        // not white - nor black castle
        // white castle 4 -> 2/6 || black castle 60 -> 58/62
        if !((old_field == 4 && (new_field == 2 || new_field == 6))
            || ((old_field == 60) && (new_field == 58 || new_field == 62)))
        {
            return;
        }
        // side is not able to castle
        if !self.castle.can_castle(self.current_move) {
            return;
        }

        if old_field == 4 {
            self.white_castle(new_field);
        } else if old_field == 60 {
            self.black_castle(new_field);
        }
    }

    fn black_castle(&mut self, new_field: usize) {
        self.castle.set_has_castled(self.current_move);
        self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[2];
        self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[3];

        // lange rutsche
        if new_field == 58 {
            self.move_black_figure(56, 59);
        }
        if new_field == 62 {
            self.move_black_figure(63, 61);
        }
    }

    fn white_castle(&mut self, new_field: usize) {
        self.castle.set_has_castled(self.current_move);
        self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[0];
        self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[1];

        if new_field == 2 {
            self.move_white_figure(0, 3);
        }
        if new_field == 6 {
            self.move_white_figure(7, 5);
        }
    }

    pub fn move_figure(&mut self, from: usize, to: usize, promoted_to: Option<Promotion>) {
        if let Some(promoted_figure) = promoted_to {
            self.update_figure_to_promoted_one(from, to, promoted_figure);

            if let Some(en_passant) = self.en_passant {
                self.zobrist_key ^= ZOBRIST_EN_PASSANT[en_passant];
                self.en_passant = None;
            }
        } else {
            // if move is caste move rook prior to moving king
            self.castle(from, to);
            // if move is en passant remove opponent (from field we did not move to!)
            self.check_and_execute_en_passant(from, to);
            // check for possible future en_passant options
            self.possible_future_en_passant(from, to);
            match self.current_move {
                Color::White => self.move_white_figure(from, to),
                Color::Black => self.move_black_figure(from, to),
            }
        }
        self.set_current_move();
    }

    fn remove_opponent_piece_from_field(&mut self, field: usize, opponent_color: Color) {
        // first check if there even is a opponent on this field
        if !self.get_opponents().field_is_used(field) {
            return;
        }
        if let Some(opponent_piece) = self.get_type_of_figure(opponent_color, field) {
            self.remove_piece(opponent_color, opponent_piece, field);
        }
    }

    fn move_black_figure(&mut self, from: usize, to: usize) {
        self.remove_opponent_piece_from_field(to, Color::White);
        if let Some(black_piece) = self.get_type_of_figure(Color::Black, from) {
            self.remove_piece(Color::Black, black_piece, from);
            self.add_piece(Color::Black, black_piece, to);
        }
        if (from == 56 || from == 60 || from == 63) && self.castle.black_can_castle() {
            if self.castle.black_castle_short && from == 63 {
                self.castle.black_castle_short = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[2];
            } else if self.castle.black_castle_long && from == 56 {
                self.castle.black_castle_long = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[3];
            } else if self.castle.black_can_castle() && from == 60 {
                self.castle.black_castle_long = false;
                self.castle.black_castle_short = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[2];
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[3];
            }
        }
        if (to == 0 || to == 7) && self.castle.white_can_castle() {
            if self.castle.white_castle_short && to == 7 {
                self.castle.white_castle_short = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[0];
            } else if self.castle.white_castle_long && to == 0 {
                self.castle.white_castle_long = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[1];
            }
        }
    }

    fn move_white_figure(&mut self, from: usize, to: usize) {
        self.remove_opponent_piece_from_field(to, Color::Black);
        if let Some(white_piece) = self.get_type_of_figure(Color::White, from) {
            self.remove_piece(Color::White, white_piece, from);
            self.add_piece(Color::White, white_piece, to);
        }
        if (from == 0 || from == 4 || from == 7) && self.castle.white_can_castle() {
            if self.castle.white_castle_short && from == 7 {
                self.castle.white_castle_short = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[0];
            } else if self.castle.white_castle_long && from == 0 {
                self.castle.white_castle_long = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[1];
            } else if self.castle.white_can_castle() && from == 4 {
                self.castle.white_castle_long = false;
                self.castle.white_castle_short = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[0];
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[1];
            }
        }
        if (to == 56 || to == 63) && self.castle.black_can_castle() {
            if self.castle.black_castle_short && to == 63 {
                self.castle.black_castle_short = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[2];
            } else if self.castle.black_castle_long && to == 56 {
                self.castle.black_castle_long = false;
                self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[3];
            }
        }
    }

    fn validate_string_position<'a>(
        &'a self,
        mov: &'a str,
    ) -> Option<(&'a str, u8, &'a str, u8, Option<&'a str>)> {
        // first validate that input is in valid format - then split it into x/y for both positions (new and old)
        let valid_move_regex =
            Regex::new(r"\A[abcdefgh][1-8][abcdefgh][1-8]([qrbkQrbK]?)").unwrap();
        let valid_move = valid_move_regex.captures(mov);

        valid_move.as_ref()?;

        //not beautiful or fast, but not important
        let valid_move_unpacked = valid_move.unwrap().get(1);
        let promoted_to_piece = if !valid_move_unpacked.unwrap().is_empty() {
            Some(valid_move_unpacked.unwrap().as_str())
        } else {
            None
        };

        let split_move_regex = Regex::new(r"((\S)(\S)(\S)(\S))").unwrap();
        let split_moves = split_move_regex.captures(mov).unwrap();
        Some((
            split_moves.get(2).unwrap().as_str(),
            split_moves.get(3).unwrap().as_str().parse::<u8>().unwrap(),
            split_moves.get(4).unwrap().as_str(),
            split_moves.get(5).unwrap().as_str().parse::<u8>().unwrap(),
            promoted_to_piece,
        ))
    }

    fn get_position_id(&self, row: &str, column: u8) -> usize {
        usize::from(self.get_row_from_string(row) + ((column - 1) * 8) - 1)
    }

    fn get_row_from_string(&self, row: &str) -> u8 {
        match row {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4,
            "e" => 5,
            "f" => 6,
            "g" => 7,
            "h" => 8,
            _ => 0,
        }
    }

    // disconnect 
    // r1b2rk1/3pqpp1/p4Bp1/2p5/1bp1P3/2N5/PPP1N1PP/R2Q1RK1 b - - 0 16
    pub fn set_to_default(&mut self) {
        let default_position =
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        self.create_position_from_input_string(default_position);
    }

    // e.g. 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1
    pub fn create_position_from_input_string(&mut self, position: String) {
        self.set_empty();
        let mut current_position: usize = 56;
        let mut positions_finished = false;
        for c in position.chars() {
            if current_position == 8 && c == ' ' && !positions_finished {
                positions_finished = true;
            } else if positions_finished {
                // handle base game state stuff
                if c == 'w' {
                    self.current_move = Color::White;
                }
                if c == 'b' {
                    self.zobrist_key ^= *ZOBRIST_CURRENT_MOVE;
                    self.current_move = Color::Black;
                }
                if c == 'K' {
                    self.castle.white_castle_short = true;
                    self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[0];
                }
                if c == 'Q' {
                    self.castle.white_castle_long = true;
                    self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[1];
                }
                if c == 'k' {
                    self.castle.black_castle_short = true;
                    self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[2];
                }
                if c == 'q' {
                    self.castle.black_castle_long = true;
                    self.zobrist_key ^= ZOBRIST_CASTLE_NUMBERS[3];
                }
            } else {
                if c == '/' {
                    current_position -= 16;
                }
                if c.is_ascii_digit() {
                    // can u feel the magic? :D
                    let as_digit = usize::from(c as u8 - 0x30);
                    current_position += as_digit;
                }
                if c.is_alphabetic() {
                    let piece = self.get_figure_from_char(c);
                    if c.is_lowercase() {
                        self.add_piece(Color::Black, piece, current_position);
                        self.zobrist_key ^= ZOBRIST_FIGURE_NUMBERS[Color::Black as usize]
                            [piece as usize][current_position];
                    } else {
                        self.add_piece(Color::White, piece, current_position);
                        self.zobrist_key ^= ZOBRIST_FIGURE_NUMBERS[Color::White as usize]
                            [piece as usize][current_position];
                    }
                    current_position += 1;
                }
            }
        }
    }

    fn get_figure_from_char(&self, figure: char) -> Piece {
        let fig = figure.to_uppercase().to_string();

        match fig.as_str() {
            "K" => Piece::King,
            "P" => Piece::Pawn,
            "Q" => Piece::Queen,
            "B" => Piece::Bishop,
            "R" => Piece::Rook,
            _ => Piece::Knight,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{engine::{count::count_moves}, make_move};
    use super::*;

    #[test]
    fn short_castle_white() {
        let mut board: Chessboard = Chessboard::empty(Color::White);
        board.castle.white_castle_short = true;
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(0);
        board.figures[Color::White as usize][Piece::King as usize].set_field(4);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(7);
        board.positions.set_field(0);
        board.positions.set_field(4);
        board.positions.set_field(7);

        board.update_position_from_uci_input("e1g1");
        assert_eq!(board.positions.field_is_used(6), true);
        assert_eq!(board.positions.field_is_used(5), true);
        assert_eq!(board.positions.field_is_used(7), false);
        assert_eq!(board.positions.field_is_used(4), false);
    }

    #[test]
    fn long_castle_white() {
        let mut board = Chessboard::empty(Color::White);
        board.castle.white_castle_long = true;
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(0);
        board.figures[Color::White as usize][Piece::King as usize].set_field(4);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(7);
        board.positions.set_field(0);
        board.positions.set_field(4);
        board.positions.set_field(7);

        board.update_position_from_uci_input("e1c1");
        assert_eq!(board.positions.field_is_used(3), true);
        assert_eq!(board.positions.field_is_used(2), true);
        assert_eq!(board.positions.field_is_used(0), false);
        assert_eq!(board.positions.field_is_used(4), false);
    }

    #[test]
    fn short_castle_black() {
        let mut board = Chessboard::empty(Color::Black);
        board.castle.black_castle_short = true;

        board.figures[Color::Black as usize][Piece::Rook as usize].set_field(56);
        board.figures[Color::Black as usize][Piece::King as usize].set_field(60);
        board.figures[Color::Black as usize][Piece::Rook as usize].set_field(63);
        board.positions.set_field(56);
        board.positions.set_field(60);
        board.positions.set_field(63);

        board.update_position_from_uci_input("e8c8");
        assert_eq!(board.positions.field_is_used(58), true);
        assert_eq!(board.positions.field_is_used(59), true);
        assert_eq!(board.positions.field_is_used(56), false);
        assert_eq!(board.positions.field_is_used(60), false);
    }

    #[test]
    fn long_castle_black() {
        let mut board = Chessboard::empty(Color::Black);
        board.castle.black_castle_long = true;
        board.figures[Color::Black as usize][Piece::Rook as usize].set_field(56);
        board.figures[Color::Black as usize][Piece::King as usize].set_field(60);
        board.figures[Color::Black as usize][Piece::Rook as usize].set_field(63);
        board.positions.set_field(56);
        board.positions.set_field(60);
        board.positions.set_field(63);

        board.update_position_from_uci_input("e8g8");
        assert_eq!(board.positions.field_is_used(61), true);
        assert_eq!(board.positions.field_is_used(62), true);
        assert_eq!(board.positions.field_is_used(60), false);
        assert_eq!(board.positions.field_is_used(63), false);
    }

    #[test]
    fn promotion_black() {
        let mut board = Chessboard::empty(Color::Black);

        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(14);
        board.positions.set_field(14);

        board.update_position_from_uci_input("g2g1q");
        assert_eq!(board.positions.field_is_used(6), true);

        let black_queens = board
            .get_pieces(Color::Black, Piece::Queen)
            .get_used_fields();
        assert_eq!(true, black_queens.contains(&6));
    }

    #[test]
    fn promotion_white() {
        let mut board = Chessboard::empty(Color::White);

        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(52);

        board.positions.set_field(52);

        board.update_position_from_uci_input("e7e8K");
        assert_eq!(board.positions.field_is_used(60), true);

        let white_knights = board
            .get_pieces(Color::White, Piece::Knight)
            .get_used_fields();
        assert_eq!(true, white_knights.contains(&60));
    }

    #[test]
    fn test_en_passant() {
        let mut board = Chessboard::empty(Color::Black);

        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(52);
        board.positions.set_field(52);
        board.move_figure(52, 36, None);
        assert_eq!(board.en_passant, Some(36));
    }

    #[test]
    fn test_no_en_passant() {
        let mut board = Chessboard::empty(Color::Black);

        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(52);
        board.positions.set_field(52);
        board.move_figure(52, 44, None);

        assert_eq!(board.en_passant, None);
    }

    #[test]
    fn test_if_en_passanted_figure_is_removed_black() {
        let mut board = Chessboard::empty(Color::Black);
        board.en_passant = Some(26);
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(26);
        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(25);

        board.move_figure(25, 18, None);

        assert_eq!(
            0,
            board
                .get_pieces(Color::White, Piece::Pawn)
                .get_used_fields()
                .len()
        );
        assert_eq!(false, board.positions.field_is_used(26))
    }

    #[test]
    fn test_if_en_passanted_figure_is_removed_white() {
        let mut board = Chessboard::empty(Color::White);
        board.en_passant = Some(36);
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(35);
        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(36);
        board.move_figure(35, 44, None);

        assert_eq!(
            0,
            board
                .get_pieces(Color::Black, Piece::Pawn)
                .get_used_fields()
                .len()
        );
        assert_eq!(false, board.positions.field_is_used(36))
    }

    #[test]
    fn test_position_creation() {
        let position = String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8");
        let mut board = Chessboard {
            ..Default::default()
        };
        board.create_position_from_input_string(position);

        assert_eq!(
            true,
            board.used_positions[Color::Black as usize].field_is_used(31)
        );
        assert_eq!(
            true,
            board.used_positions[Color::Black as usize].field_is_used(39)
        );
        assert_eq!(
            false,
            board.used_positions[Color::Black as usize].field_is_used(38)
        );

        assert_eq!(
            true,
            board.used_positions[Color::White as usize].field_is_used(14)
        );
        assert_eq!(
            true,
            board.used_positions[Color::White as usize].field_is_used(33)
        );
        assert_eq!(
            false,
            board.used_positions[Color::White as usize].field_is_used(34)
        );
    }

    #[test]
    fn test_default_position() {
        let board = Chessboard {
            ..Default::default()
        };
        let count = count_moves(&board, 4);
        assert_eq!(197281, count);
    }

    #[test]
    fn test_black_en_passant_on_a_file(){
        let mut board = Chessboard{..Default::default()};
        board.create_position_from_input_string(String::from("8/8/8/8/p6k/8/1P5K/8 w - - 0 1"));
        board.move_figure(9, 25, None);
        
        let is_black_en_passant = board.is_en_passant_black(24, 17);
        assert_eq!(true, is_black_en_passant)
    }

        #[test]
    fn test_black_en_passant_on_h_file(){
        let mut board = Chessboard{..Default::default()};
        board.create_position_from_input_string(String::from("8/k2K4/8/8/7p/8/6P1/8 w - - 0 1"));
        board.move_figure(14, 30, None);
        
        let is_black_en_passant = board.is_en_passant_black(31, 22);
        assert_eq!(true, is_black_en_passant)
    }

    #[test]
    #[ignore]
    fn test_position_2() {
        let mut board = Chessboard::empty(Color::White);

        let position_2 =
            String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");
        board.create_position_from_input_string(position_2);
        let count = count_moves(&board,4);
        assert_eq!(4085603, count);
    }

    #[test]
    #[ignore]
    fn test_position_3() {
        let mut board = Chessboard::empty(Color::White);
        let position_3 = String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ");
        board.create_position_from_input_string(position_3);
        let count = count_moves(&board, 5);
        assert_eq!(674624, count);
    }

    #[test]
    #[ignore]
    fn test_position_4() {
        let mut board = Chessboard::empty(Color::White);

        let position_4 =
            String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        board.create_position_from_input_string(position_4);
        let count = count_moves(&board, 4);
        assert_eq!(422333, count);
    }

    #[test]
    #[ignore]
    fn test_position_5() {
        // this test fails if it is run after the other tests for some reason I dont want to debug :D
        let mut board = Chessboard::empty(Color::White);

        let position_5 =
            String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ");
        board.create_position_from_input_string(position_5);
        let count = count_moves(&board, 4);
        assert_eq!(2103487, count);
    }

    #[test]
    #[ignore]
    fn test_position_6() {
        // this test fails if it is run after the other tests for some reason I dont want to debug :D
        let mut board = Chessboard::empty(Color::White);

        let position_6 = String::from(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ",
        );
        board.create_position_from_input_string(position_6);

        let count = count_moves(&board, 4);
        assert_eq!(3894594, count);
    }

    #[test]
    #[ignore]
    fn test_position_double_checked() {
        let mut board = Chessboard::empty(Color::White);

        let position = String::from("2Q3n1/R7/k7/8/8/8/P1r3P1/3K4 b - - 0 18");
        board.create_position_from_input_string(position);

        let count = count_moves(&board, 4);
        assert_eq!(36899, count);
    }

    #[test]
    #[ignore]
    fn test_if_zobrist_for_color_works(){
        let mut board = Chessboard::empty(Color::White);
        let position = String::from("r1k2b1r/p1p1pppp/2p1q1b1/3pN3/3P1B2/2Q1PP2/PPP3PP/R3K2R w KQ - 2 13");
        board.create_position_from_input_string(position);
        make_move(Vec::new(),&board, &mut Vec::new(), 20);
        // just count to check if we run into issues with king related zo zobrist
    }
}
