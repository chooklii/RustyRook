use crate::{board::board::Chessboard, helper::movement::{figure_can_move_backward, figure_can_move_forward, figure_can_move_left, figure_can_move_right}};

use super::{color::Color, figures::SingleMove};

#[derive(Default, Clone)]
pub struct King {
    pub color: Color,
    pub has_moved: bool,
}

impl King {
    pub fn set_moved(&mut self) {
        self.has_moved = true;
    }

    fn check_move(
        &self,
        board: &Chessboard,
        next_position: usize,
        possible_moves: &mut Vec<SingleMove>,
    ) {
        if board.positions.get(next_position) {
            if board
                .get_opponents()
                .contains_key(&next_position)
            {
                possible_moves.push(SingleMove { to: next_position, promotion: None });
            }
        } else {
            possible_moves.push(SingleMove { to: next_position, promotion: None });
        }
    }

    pub fn possible_moves(
        &self,
        board: &Chessboard,
        own_position: &usize,
        opponent_moves: &Vec<usize>,
    ) -> Vec<SingleMove> {
        let mut possible_moves = Vec::new();

        let can_move_backward = figure_can_move_backward(own_position);
        let can_move_left = figure_can_move_left(own_position);
        let can_move_right = figure_can_move_right(own_position);
        let can_move_forward = figure_can_move_forward(own_position);

        if can_move_backward {
            self.check_move(board, own_position - 8, &mut possible_moves);
            if can_move_left {
                self.check_move(board, own_position - 9, &mut possible_moves);
            }
            if can_move_right {
                self.check_move(board, own_position - 7, &mut possible_moves);
            }
        }
        if can_move_forward {
            self.check_move(board, own_position + 8, &mut possible_moves);
            if can_move_left {
                self.check_move(board, own_position + 7, &mut possible_moves);
            }
            if can_move_right {
                self.check_move(board, own_position + 9, &mut possible_moves);
            }
        }
        if can_move_left {
            self.check_move(board, own_position - 1, &mut possible_moves);
        }
        if can_move_right {
            self.check_move(board, own_position + 1, &mut possible_moves);
        }

        // castle
        if !self.has_moved && !opponent_moves.contains(own_position) {
            match self.color {
                Color::White => self.white_castle(&board, &opponent_moves, &mut possible_moves),
                Color::Black => self.black_castle(&board, &opponent_moves, &mut possible_moves),
            }
        }
        // filter out fields opponent can take
        possible_moves.into_iter().filter(|position|!opponent_moves.contains(&position.to)).collect()
    }

    pub fn threatened_fields(
        &self,
        own_position: &usize,
    ) -> Vec<usize>{
        let mut possible_moves = Vec::new();

        let can_move_backward = figure_can_move_backward(own_position);
        let can_move_left = figure_can_move_left(own_position);
        let can_move_right = figure_can_move_right(own_position);
        let can_move_forward = figure_can_move_forward(own_position);

        if can_move_backward {
            possible_moves.push(own_position -8);
            if can_move_left {
                possible_moves.push(own_position -9);
            }
            if can_move_right {
                possible_moves.push(own_position -7);
            }
        }
        if can_move_forward {
            possible_moves.push(own_position +8);
            if can_move_left {
                possible_moves.push(own_position +7);
            }
            if can_move_right {
                possible_moves.push(own_position +9);
            }
        }
        if can_move_left {
            possible_moves.push(own_position -1);
        }
        if can_move_right {
            possible_moves.push(own_position +1);
        } 
        possible_moves
    }

    fn is_possible_castle(
        &self,
        board: &Chessboard,
        opponent_moves: &Vec<usize>,
        rook_field: &usize,
        new_king_position: usize,
        field_between: usize,
        // opt. field we need to empty check for long rochade
        long_rochade_free_field: Option<usize>
    ) -> bool {
        // rook is in the corner, has not moved && all fields between them are not in danger
        if let Some(figure) = board.get_next_player_figures().get(rook_field) {

            if !figure.is_rook() || figure.has_moved(){
                return false;
            }
            
            if let Some(extra_field) = long_rochade_free_field{
                if board.positions.get(extra_field){
                    return false;
                }
            }

            return !(opponent_moves.contains(&field_between)
                    || opponent_moves.contains(&new_king_position)
                    || board.positions.get(field_between)
                    || board.positions.get(new_king_position));
        }
        return false;
    }

    fn white_castle(
        &self,
        board: &Chessboard,
        opponent_moves: &Vec<usize>,
        possible_moves: &mut Vec<SingleMove>,
    ) {
        // short
        if self.is_possible_castle(board, opponent_moves, &7, 6, 5, None) {
            possible_moves.push(SingleMove { to: 6, promotion: None });
        }
        // long
        if self.is_possible_castle(board, opponent_moves, &0, 2, 3, Some(1)) {
            possible_moves.push(SingleMove { to: 2, promotion: None });
        }
    }

    fn black_castle(
        &self,
        board: &Chessboard,
        opponent_moves: &Vec<usize>,
        possible_moves: &mut Vec<SingleMove>,
    ) {
        // short
        if self.is_possible_castle(board, opponent_moves, &63, 62, 61, None){
            possible_moves.push(SingleMove { to: 62, promotion: None });
        }
        // long
        if self.is_possible_castle(board, opponent_moves, &56, 58, 59, Some(57)){
            possible_moves.push(SingleMove { to: 58, promotion: None });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::figures::{figures::Figure, pawn::Pawn, queen::Queen, rook::Rook};
    use bitmaps::Bitmap;
    use rustc_hash::FxHashMap;
    use super::*;

    #[test]
    fn move_empty_board() {
        let figure = King {
            color: Color::Black,
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &10, &Vec::new());
        assert_eq!(8, moves.len());

        let moves = figure.possible_moves(&board, &0, &Vec::new());
        assert_eq!(3, moves.len());

        let moves = figure.possible_moves(&board, &31, &Vec::new());
        assert_eq!(5, moves.len());
    }

    #[test]
    fn castle_on_empty_board() {
        let figure = King {
            color: Color::White,
            ..Default::default()
        };

        let mut positions = Bitmap::<64>::new();

        positions.set(0, true);
        positions.set(4, true);
        positions.set(7, true);

        let mut white_figures: FxHashMap<usize, Figure> = FxHashMap::default();

        white_figures.insert(
            0,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );
        white_figures.insert(
            7,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );

        let board = Chessboard {
            positions,
            white_figures,
            ..Default::default()
        };

        let own_moves = figure.possible_moves(&board, &4, &Vec::new());

        let own_move_positions: Vec<usize> = own_moves.into_iter().map(|x| x.to).collect();
        // can castle left and right
        assert_eq!(7, own_move_positions.len());

        assert_eq!(true, own_move_positions.contains(&6));
        assert_eq!(true, own_move_positions.contains(&2));
    }

    #[test]
    fn not_able_to_castle_long() {
        let figure = King {
            color: Color::White,
            ..Default::default()
        };

        let mut positions = Bitmap::<64>::new();
        positions.set(0, true);
        positions.set(4, true);
        positions.set(7, true);

        let mut white_figures: FxHashMap<usize, Figure> = FxHashMap::default();
        white_figures.insert(
            0,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );
        white_figures.insert(
            7,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );

        let board = Chessboard {
            positions,
            white_figures,
            ..Default::default()
        };

        let mut opponent_moves: Vec<usize> = Vec::new();
        opponent_moves.push(2);

        let own_moves = figure.possible_moves(&board, &4, &opponent_moves);
        let own_move_positions: Vec<usize> = own_moves.into_iter().map(|x| x.to).collect();
        assert_eq!(6, own_move_positions.len());
        assert_eq!(true, own_move_positions.contains(&6));
        assert_eq!(false, own_move_positions.contains(&2));
    }

    #[test]
    fn not_able_to_castle_long_as_extra_field_is_used() {
        let figure = King {
            color: Color::White,
            ..Default::default()
        };

        let mut positions = Bitmap::<64>::new();
        positions.set(0, true);
        positions.set(4, true);
        positions.set(7, true);
        positions.set(1, true);

        let mut white_figures: FxHashMap<usize, Figure> = FxHashMap::default();
        white_figures.insert(
            0,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );
        white_figures.insert(
            7,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );

        let board = Chessboard {
            positions,
            white_figures,
            ..Default::default()
        };

        let own_moves = figure.possible_moves(&board, &4, &Vec::new());
        let own_move_positions: Vec<usize> = own_moves.into_iter().map(|x| x.to).collect();
        assert_eq!(6, own_move_positions.len());
        assert_eq!(true, own_move_positions.contains(&6));
        assert_eq!(false, own_move_positions.contains(&2));
    }

    #[test]
    fn not_able_to_castle() {
        let figure = King {
            color: Color::White,
            ..Default::default()
        };

        let mut positions = Bitmap::<64>::new();
        positions.set(0, true);
        positions.set(2, true);
        positions.set(4, true);
        positions.set(6, true);
        positions.set(7, true);

        let mut white_figures: FxHashMap<usize, Figure> = FxHashMap::default();
        white_figures.insert(
            0,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );
        white_figures.insert(
            7,
            Figure::Rook(Rook {
                ..Default::default()
            }),
        );

        let board = Chessboard {
            positions,
            white_figures,
            ..Default::default()
        };

        let own_moves = figure.possible_moves(&board, &4, &Vec::new());
        let own_move_positions: Vec<usize> = own_moves.into_iter().map(|x| x.to).collect();

        // castle is not possible as there are figures in the way
        assert_eq!(5, own_move_positions.len());
        assert_eq!(false, own_move_positions.contains(&6));
        assert_eq!(false, own_move_positions.contains(&2));
    }

    #[test]
    fn not_beeing_able_to_move_as_all_fields_are_check(){

        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: FxHashMap::default(),
            black_figures: FxHashMap::default(),
            current_move: Color::Black,
            ..Default::default()
        };
        // king is on d8 and checked by queen on h8 - king cannot move as c7-e7 are full with pawns 
        board.positions.set(50, true);
        board.positions.set(51, true);
        board.positions.set(52, true);
        board.positions.set(59, true);
        board.positions.set(63, true);
        // it should also not be able to move to c8 
        board.white_figures.insert(63, Figure::Queen(Queen{}));
        board.black_figures.insert(50, Figure::Pawn(Pawn{color: Color::Black}));
        board.black_figures.insert(51, Figure::Pawn(Pawn{color: Color::Black}));
        board.black_figures.insert(52, Figure::Pawn(Pawn{color: Color::Black}));

        let figure = King {
            color: Color::Black,
            ..Default::default()
        };

        let mut opponent_moves =  Vec::new();
        opponent_moves.push(62);
        opponent_moves.push(61);
        opponent_moves.push(60);
        opponent_moves.push(59);
        opponent_moves.push(58);
        opponent_moves.push(57);

        let own_moves = figure.possible_moves(&board, &59, &opponent_moves);
        assert_eq!(0, own_moves.len());
    }
}
