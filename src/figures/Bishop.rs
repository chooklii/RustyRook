use crate::board::board::Chessboard;
use crate::figures::color::Color;

#[derive(Default, Clone)]
pub struct Bishop {
    pub color: Color,
}

// Queen is a Bishop as well - reuse this
pub fn get_bishop_moves(board: &Chessboard,color: &Color, position: &usize) -> Vec<usize>{
    let mut moves = Vec::new();
    move_back_left(board, color, position, &mut moves);
    move_forward_left(board,color, position, &mut moves);
    move_backward_right(board,color, position, &mut moves);
    move_forward_right(board,color, position, &mut moves);
    moves
}

fn move_back_left(board: &Chessboard, color: &Color, own_position: &usize, moves: &mut Vec<usize>) {
    if board.figure_can_move_left(own_position) && board.figure_can_move_backward(own_position){
        let next_position = own_position - 9;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                moves.push(next_position);
            }
        } else {
            moves.push(next_position);
            move_back_left(board, color, &next_position, moves);
        }
    }
}

fn move_forward_left(board: &Chessboard, color: &Color, own_position: &usize, moves: &mut Vec<usize>) {
    if board.figure_can_move_left(own_position) && board.figure_can_move_forward(own_position){
        let next_position = own_position +7;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                moves.push(next_position);
            }
        } else {
            moves.push(next_position);
            move_forward_left(board, color, &next_position, moves);
        }
    }
}

fn move_forward_right(board: &Chessboard, color: &Color, own_position: &usize, moves: &mut Vec<usize>) {
    if board.figure_can_move_right(own_position) && board.figure_can_move_forward(own_position){
        let next_position = own_position + 9;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                moves.push(next_position);
            }
        } else {
            moves.push(next_position);
            move_forward_right(board, color, &next_position, moves);
        }
    }
}

fn move_backward_right(board: &Chessboard,color: &Color, own_position: &usize, moves: &mut Vec<usize>) {
    if board.figure_can_move_right(own_position) && board.figure_can_move_backward(own_position){
        let next_position = own_position -7;
        if board.positions.get(next_position) {
            if board
                .get_opponents(color)
                .contains_key(&next_position)
            {
                moves.push(next_position);
            }
        } else {
            moves.push(next_position);
            move_backward_right(board, color,&next_position, moves);
        }
    }
}

impl Bishop {
    pub fn possible_moves(&self, board: &Chessboard, own_position: &usize) -> Vec<usize> {
        get_bishop_moves(board, &self.color, own_position)
    }
}

#[cfg(test)]
mod tests {
    use bitmaps::Bitmap;

    use super::*;


    #[test]
    fn move_empty_board(){
        let figure = Bishop {
            ..Default::default()
        };
        let board = Chessboard {
            positions: Bitmap::<64>::new(),
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &27);
        assert_eq!(13, moves.len());

        let moves = figure.possible_moves(&board, &0);
        assert_eq!(7, moves.len());
    }

    #[test]
    fn not_able_to_move(){
        let figure = Bishop {
            ..Default::default()
        };
        let mut positions = Bitmap::<64>::new();
        positions.set(9, true);
        positions.set(11, true);
        positions.set(27, true);
        positions.set(25, true);
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &18);
        assert_eq!(0, moves.len()); 
    }

    #[test]
    fn able_to_move_in_two_directions(){
        let figure = Bishop {
            ..Default::default()
        };
        let mut positions = Bitmap::<64>::new();
        positions.set(29, true);
        positions.set(13, true);
        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let moves = figure.possible_moves(&board, &20);
        assert_eq!(6, moves.len()); 
    }


}