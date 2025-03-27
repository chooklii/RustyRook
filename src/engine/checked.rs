use std::collections::HashMap;

use crate::{
    board::board::Chessboard,
    figures::{color::Color, figures::Figure},
    helper::{
        movement::{
            self, figure_can_move_backward, figure_can_move_backward_and_left,
            figure_can_move_backward_and_right, figure_can_move_forward,
            figure_can_move_forward_and_left, figure_can_move_forward_and_right,
            figure_can_move_left, figure_can_move_right,
        },
        moves_by_field::MoveInEveryDirection,
    },
};

// get all the fields we can place a figure (not king) to prevent a active check by the opponent
pub fn get_fields_to_prevent_check(
    board: &Chessboard,
    king_position: &usize,
    opponent_moves: &Vec<usize>,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> Vec<usize> {
    let count_of_checks = opponent_moves
        .iter()
        .filter(|field| field.eq(&king_position))
        .count();
    // if there is more than one check only moving the king can save us
    if count_of_checks > 1 {
        return Vec::new();
    }
    if let Some(rook_checking_field) = check_and_get_rook_movement_check_field(
        board,
        king_position,
        opponent_moves,
        &moves_by_field,
    ) {
        return rook_checking_field;
    } else if let Some(bishop_checking_field) = check_and_get_bishop_movement_check_field(
        board,
        king_position,
        opponent_moves,
        &moves_by_field,
    ) {
        return bishop_checking_field;
    } else if let Some(knight_check_field) =
        check_and_get_knight_check_field(board, king_position, &moves_by_field)
    {
        return vec![knight_check_field];
    } else if let Some(pawn_check_field) = check_and_get_pawn_check_field(board, king_position) {
        return vec![pawn_check_field];
    }

    Vec::new()
}

fn is_rook_movement_figure(figure: &Figure) -> bool {
    figure.is_queen() || figure.is_rook()
}

fn is_bishop_movement_figure(figure: &Figure) -> bool {
    figure.is_queen() || figure.is_bishop()
}

fn check_and_get_bishop_movement_check_field(
    board: &Chessboard,
    king_position: &usize,
    opponent_moves: &Vec<usize>,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> Option<Vec<usize>> {
    if let Some(movement) = moves_by_field.get(king_position) {
        let left_fw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.left_forward,
            is_bishop_movement_figure);

        if left_fw.is_some(){
            return left_fw;
        }
        let right_fw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.right_forward,
            is_bishop_movement_figure,
        );
        if right_fw.is_some(){
            return right_fw;
        }
        let left_bw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.left_back,
            is_bishop_movement_figure
        );
        if left_bw.is_some(){
            return left_bw;
        }
        let right_bw = check_single_direction_check(
            board,
            opponent_moves,
            &movement.right_back,
            is_bishop_movement_figure,
        );
        if right_bw.is_some(){
            return right_bw;
        }
    }
    return None;
}

fn check_and_get_rook_movement_check_field(
    board: &Chessboard,
    king_position: &usize,
    opponent_moves: &Vec<usize>,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> Option<Vec<usize>> {
    if let Some(moves) = moves_by_field.get(king_position) {
        let left = check_single_direction_check(
            board,
            opponent_moves,
            &moves.left,
            is_rook_movement_figure,
        );
        if left.is_some(){
            return left;
        }
        // right
        let right = check_single_direction_check(
            board,
            opponent_moves,
            &moves.right,
            is_rook_movement_figure,
        );
        if right.is_some(){
            return right;
        }
        // forward
        let forward =  check_single_direction_check(
            board,
            opponent_moves,
            &moves.forward,
            is_rook_movement_figure,
        );
        if forward.is_some(){
            return forward;
        }
        // backward
        let backward = check_single_direction_check(
            board,
            opponent_moves,
            &moves.back,
            is_rook_movement_figure,
        );
        if backward.is_some(){
            return backward
        }
    }

    return None;
}

fn check_single_direction_check(
    board: &Chessboard,
    opponent_moves: &Vec<usize>,
    moves: &Vec<usize>,
    figure_check: fn(&Figure) -> bool,
) -> Option<Vec<usize>> {
    let mut fields_to_prevent_check: Vec<usize> = Vec::new();
    for movement in moves {
        if board.positions.get(*movement) {
            if let Some(opponent) = board.get_opponents(&board.current_move).get(movement) {
                if figure_check(opponent) {
                    fields_to_prevent_check.push(*movement);
                    return Some(fields_to_prevent_check);
                }
                // field is used by opponent - but not a figure threadning us
                return None;
            }
            // field is used by our own figure
            return None;
        } else if !opponent_moves.contains(movement) {
            // opponent does not attack this field thus there can not be a attacker in this row
            return None;
        }
        fields_to_prevent_check.push(*movement);
    }
    return None;
}

fn check_and_get_knight_check_field(
    board: &Chessboard,
    king_position: &usize,
    moves_by_field: &HashMap<usize, MoveInEveryDirection>,
) -> Option<usize> {
    if let Some(moves) = moves_by_field.get(king_position) {
        for field in moves.knight_moves.iter() {
            if field_is_used_by_opponent_knight(board, *field) {
                return Some(*field);
            }
        }
    }
    return None;
}

fn field_is_used_by_opponent_knight(board: &Chessboard, position: usize) -> bool {
    if let Some(figure) = board.get_opponents(&board.current_move).get(&position) {
        return figure.is_knight();
    }
    return false;
}

fn check_and_get_pawn_check_field(board: &Chessboard, position: &usize) -> Option<usize> {
    match board.current_move {
        Color::White => check_and_get_pawn_check_field_white(board, position),
        Color::Black => check_and_get_pawn_check_field_black(board, position),
    }
}

fn check_and_get_pawn_check_field_black(board: &Chessboard, position: &usize) -> Option<usize> {
    // king is - for whatever reason :D - on a1-h1
    if !figure_can_move_backward(&position) {
        return None;
    }
    // left
    if let Some(figure) = board
        .get_opponents(&board.current_move)
        .get(&(position - 9))
    {
        if figure.is_pawn() {
            return Some(position - 9);
        };
    }
    // right
    if let Some(figure) = board
        .get_opponents(&board.current_move)
        .get(&(position - 7))
    {
        if figure.is_pawn() {
            return Some(position - 7);
        };
    }
    return None;
}

fn check_and_get_pawn_check_field_white(board: &Chessboard, position: &usize) -> Option<usize> {
    if !figure_can_move_forward(&position) {
        return None;
    }
    // left
    if let Some(figure) = board
        .get_opponents(&board.current_move)
        .get(&(position + 7))
    {
        if figure.is_pawn() {
            return Some(position + 7);
        };
    }
    // right
    if let Some(figure) = board
        .get_opponents(&board.current_move)
        .get(&(position + 9))
    {
        if figure.is_pawn() {
            return Some(position + 9);
        };
    }
    return None;
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use bitmaps::Bitmap;

    use crate::{
        figures::{
            bishop::Bishop, color::Color, figures::Figure, king::King, knight::Knight, pawn::Pawn,
            rook::Rook,
        },
        helper::moves_by_field::get_moves_for_each_field,
    };

    use super::*;

    #[test]
    fn test_rook_check() {
        let possible_moves = get_moves_for_each_field();
        
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            ..Default::default()
        };

        board.positions.set(19, true);
        board.positions.set(16, true);
        board.positions.set(1, true);

        board.white_figures.insert(
            19,
            Figure::King(King {
                ..Default::default()
            }),
        );
        board.black_figures.insert(
            16,
            Figure::Rook(Rook {
                color: Color::Black,
                ..Default::default()
            }),
        );

        let mut opponent_moves: Vec<usize> = Vec::new();
        opponent_moves.push(19);
        opponent_moves.push(18);
        opponent_moves.push(17);
        assert_eq!(
            3,
            check_and_get_rook_movement_check_field(&board, &19, &opponent_moves, &possible_moves)
                .unwrap()
                .len()
        );
    }

    #[test]
    fn test_bishop_check() {
        let possible_moves = get_moves_for_each_field();
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            ..Default::default()
        };

        board.positions.set(19, true);
        board.positions.set(55, true);

        board.white_figures.insert(
            19,
            Figure::King(King {
                ..Default::default()
            }),
        );
        board.black_figures.insert(
            55,
            Figure::Bishop(Bishop {
                color: Color::Black,
                ..Default::default()
            }),
        );

        let mut opponent_moves: Vec<usize> = Vec::new();
        opponent_moves.push(46);
        opponent_moves.push(37);
        opponent_moves.push(28);
        opponent_moves.push(19);
        assert_eq!(
            4,
            check_and_get_bishop_movement_check_field(&board, &19, &opponent_moves, &possible_moves)
                .unwrap()
                .len()
        );
    }

    #[test]
    fn test_pawn_check_white() {
        let moves_by_field = get_moves_for_each_field();
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            ..Default::default()
        };

        board.positions.set(19, true);
        board.positions.set(26, true);

        board.white_figures.insert(
            19,
            Figure::King(King {
                ..Default::default()
            }),
        );
        board.black_figures.insert(
            26,
            Figure::Pawn(Pawn {
                color: Color::Black,
                ..Default::default()
            }),
        );

        let opponent_moves: Vec<usize> = Vec::new();
        let result = get_fields_to_prevent_check(&board, &19, &opponent_moves, &moves_by_field);
        assert_eq!(1, result.len());
        assert_eq!(true, result.contains(&26))
    }

    #[test]
    fn test_knigh_check_black() {
        let moves_by_field = get_moves_for_each_field();
        let mut board = Chessboard {
            positions: Bitmap::<64>::new(),
            white_figures: HashMap::new(),
            black_figures: HashMap::new(),
            current_move: Color::Black,
            ..Default::default()
        };

        board.positions.set(8, true);
        board.positions.set(18, true);

        board.black_figures.insert(
            8,
            Figure::King(King {
                color: Color::Black,
                ..Default::default()
            }),
        );
        board.white_figures.insert(
            18,
            Figure::Knight(Knight {
                color: Color::White,
                ..Default::default()
            }),
        );

        let opponent_moves: Vec<usize> = Vec::new();
        let result = get_fields_to_prevent_check(&board, &8, &opponent_moves, &moves_by_field);
        assert_eq!(1, result.len());
        assert_eq!(true, result.contains(&18))
    }
}
