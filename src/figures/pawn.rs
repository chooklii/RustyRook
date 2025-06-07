use crate::board::bitboard::Bitboard;
use crate::board::board::Chessboard;
use crate::board::promotion::Promotion;
use crate::engine::engine::PossibleMove;
use crate::figures::color::Color;
use crate::{MOVES_BY_FIELD, PAWN_PROMOTION_FIELDS, PAWN_THREATS};

use super::piece::Piece;

fn can_move_two_fields(own_position: usize, own_color: Color) -> bool {
    match own_color {
        Color::White => (7..=15).contains(&own_position),
        Color::Black => (48..=55).contains(&own_position),
    }
}

fn take_left_position(one_step_forward: &usize, own_color: Color) -> usize {
    match own_color {
        Color::White => one_step_forward - 1,
        Color::Black => one_step_forward + 1,
    }
}

fn take_right_position(one_step_forward: &usize, own_color: Color) -> usize {
    match own_color {
        Color::White => one_step_forward + 1,
        Color::Black => one_step_forward - 1,
    }
}

fn en_passant_position_left(own_position: &usize, own_color: Color) -> usize {
    match own_color {
        Color::White => own_position - 1,
        Color::Black => own_position + 1,
    }
}

fn en_passant_position_right(own_position: &usize, own_color: Color) -> usize {
    match own_color {
        Color::White => own_position + 1,
        Color::Black => own_position - 1,
    }
}

fn figure_can_move_left(field: usize, color: &Color) -> bool {
    match color {
        Color::White => field % 8 != 0,
        Color::Black => field % 8 != 7,
    }
}

fn figure_can_move_right(field: usize, color: &Color) -> bool {
    match color {
        Color::White => field % 8 != 7,
        Color::Black => field % 8 != 0,
    }
}

fn figure_will_promote(field: usize, color: &Color) -> bool {
    match color {
        Color::White => field > 55,
        Color::Black => field < 8,
    }
}

fn calculate_forward_position(own_position: usize, own_color: Color, movement: usize) -> usize {
    match own_color {
        Color::Black => own_position - movement,
        Color::White => own_position + movement,
    }
}

// check if en passant would put our king into check (not captures by pinned peaces, as there are two between R/Q and K)
fn en_passant_no_check(
    board: &Chessboard,
    own_position: &usize,
    own_color: Color,
    en_passanted: &usize,
) -> bool {
    let own_king_board = board.get_pieces(own_color, Piece::King);
    let own_king_position = own_king_board.get_first_field();

    if let Some(moves) = MOVES_BY_FIELD.get(&own_king_position) {
        if moves.left.contains(own_position) && moves.left.contains(en_passanted) {
            return check_if_other_figure_in_between(
                board,
                &moves.left,
                en_passanted,
                own_position,
            );
        }
        if moves.right.contains(own_position) && moves.right.contains(en_passanted) {
            return check_if_other_figure_in_between(
                board,
                &moves.right,
                en_passanted,
                own_position,
            );
        }
    }
    true
}

fn check_if_other_figure_in_between(
    board: &Chessboard,
    moves: &Vec<usize>,
    en_passanted: &usize,
    own_position: &usize,
) -> bool {
    for single in moves {
        // ignore both pawns involved in en passant
        if single != en_passanted && single != own_position && board.positions.field_is_used(*single) {
            return !board.is_queen_or_rook(board.get_opponent_color(), *single);
        }
    }
    true
}

fn add_pawn_takes(
    board: &Chessboard,
    own_color: Color,
    own_position: usize,
    possible_moves: &mut Vec<PossibleMove>,
) {
    let possible_takes = &PAWN_THREATS[own_color as usize][own_position];

    let real_takes = Bitboard {
        board: possible_takes.board & board.get_opponents().board,
    };
    let takes_with_promotion = Bitboard {
        board: real_takes.board & PAWN_PROMOTION_FIELDS.board,
    };

    // we either have takes with promotion or regular takes - never both
    if takes_with_promotion.board != 0 {
        takes_with_promotion.iterate_board(|new_field| {
            add_promotion_to_possible_moves(own_position, new_field, possible_moves)
        });
    } else {
        real_takes.iterate_board(|new_field| {
            possible_moves.push(PossibleMove {
                from: own_position,
                to: new_field,
                promoted_to: None,
            })
        });
    }
}

pub fn get_possible_pawn_moves(
    board: &Chessboard,
    own_position: usize,
    own_color: Color,
    possible_moves: &mut Vec<PossibleMove>
){

    let one_step_forward = calculate_forward_position(own_position, own_color, 8);
    if let Some(possible_en_passant) = board.en_passant {
        if figure_can_move_left(own_position, &own_color) && en_passant_position_left(&own_position, own_color) == possible_en_passant && en_passant_no_check(board, &own_position, own_color, &possible_en_passant) {
            let take_left_position = take_left_position(&one_step_forward, own_color);
            possible_moves.push(PossibleMove {
                from: own_position,
                to: take_left_position,
                promoted_to: None,
            });
        }
        if figure_can_move_right(own_position, &own_color) {
            if let Some(possible_en_passant) = board.en_passant {
                if en_passant_position_right(&own_position, own_color) == possible_en_passant
                    && en_passant_no_check(board, &own_position, own_color, &possible_en_passant)
                {
                    let take_right_position = take_right_position(&one_step_forward, own_color);
                    possible_moves.push(PossibleMove {
                        to: take_right_position,
                        from: own_position,
                        promoted_to: None,
                    });
                }
            }
        }
    }
    // one field forward
    if !board.positions.field_is_used(one_step_forward){
        if !figure_will_promote(one_step_forward, &own_color) {
            possible_moves.push(PossibleMove {
                to: one_step_forward,
                from: own_position,
                promoted_to: None,
            });
        }
        // two fields forward
        if can_move_two_fields(own_position, own_color) {
            let two_steps_forward = calculate_forward_position(own_position, own_color, 16);

            if !board.positions.field_is_used(two_steps_forward) {
                possible_moves.push(PossibleMove {
                    to: two_steps_forward,
                    from: own_position,
                    promoted_to: None,
                });
            }
        }
    }
}

fn add_promotion_to_possible_moves(
    old_field: usize,
    new_field: usize,
    possible_moves: &mut Vec<PossibleMove>,
) {
    possible_moves.push(PossibleMove {
        to: new_field,
        from: old_field,
        promoted_to: Some(Promotion::Queen),
    });
    possible_moves.push(PossibleMove {
        to: new_field,
        from: old_field,
        promoted_to: Some(Promotion::Knight),
    });
    possible_moves.push(PossibleMove {
        to: new_field,
        from: old_field,
        promoted_to: Some(Promotion::Bishop),
    });
    possible_moves.push(PossibleMove {
        to: new_field,
        from: old_field,
        promoted_to: Some(Promotion::Rook),
    });
}

pub fn get_fields_threatened_by_pawn(own_position: usize, own_color: Color) -> Bitboard {
    PAWN_THREATS[own_color as usize][own_position]
}

pub fn get_possible_pawn_takes_and_promotion(
    board: &Chessboard,
    own_position: usize,
    own_color: Color,
    possible_moves: &mut Vec<PossibleMove>,
) {
    add_pawn_takes(board, own_color, own_position, possible_moves);
    let one_step_forward = calculate_forward_position(own_position, own_color, 8);
    if figure_will_promote(one_step_forward, &own_color)
        && !board.positions.field_is_used(one_step_forward)
    {
        add_promotion_to_possible_moves(own_position, one_step_forward, possible_moves);
    }
}

pub fn get_possible_pawn_moves_to_prevent_check(
    board: &Chessboard,
    own_position: usize,
    own_color: Color,
    prevent_check_fields: Bitboard,
    possible_moves: &mut Vec<PossibleMove>,
) {
    // Takes
    let possible_takes = &PAWN_THREATS[own_color as usize][own_position];

    let real_takes = Bitboard {
        board: possible_takes.board & board.get_opponents().board & prevent_check_fields.board,
    };
    let takes_with_promotion = Bitboard {
        board: real_takes.board & PAWN_PROMOTION_FIELDS.board & prevent_check_fields.board,
    };

    // we either have takes with promotion or regular takes - never both
    if takes_with_promotion.board != 0 {
        takes_with_promotion.iterate_board(|new_field| {
            add_promotion_to_possible_moves(own_position, new_field, possible_moves)
        });
    } else {
        real_takes.iterate_board(|new_field| {
            possible_moves.push(PossibleMove {
                from: own_position,
                to: new_field,
                promoted_to: None,
            })
        });
    }

    let one_step_forward = calculate_forward_position(own_position, own_color, 8);
    if let Some(possible_en_passant) = board.en_passant {
        // en passant can only prevent check if there is exactly one field we can prevent check
        // (and that is the figure checking aka pawn)
        if prevent_check_fields.board.count_ones() == 1 {
            // both fields are null checked above
            let checked_by_field = prevent_check_fields.get_first_field();
            let en_passant_field = board.en_passant.unwrap();
            // they are not the same -> no en passant to prevent possible
            if checked_by_field == en_passant_field {
                if figure_can_move_left(own_position, &own_color) && en_passant_position_left(&own_position, own_color) == possible_en_passant && en_passant_no_check(
                            board,
                            &own_position,
                            own_color,
                            &possible_en_passant,
                        ) {
                    let take_left_position = take_left_position(&one_step_forward, own_color);
                    possible_moves.push(PossibleMove {
                        from: own_position,
                        to: take_left_position,
                        promoted_to: None,
                    });
                }
                if figure_can_move_right(own_position, &own_color) {
                    if let Some(possible_en_passant) = board.en_passant {
                        if en_passant_position_right(&own_position, own_color)
                            == possible_en_passant
                            && en_passant_no_check(
                                board,
                                &own_position,
                                own_color,
                                &possible_en_passant,
                            )
                        {
                            let take_right_position =
                                take_right_position(&one_step_forward, own_color);
                            possible_moves.push(PossibleMove {
                                to: take_right_position,
                                from: own_position,
                                promoted_to: None,
                            });
                        }
                    }
                }
            }
        }
    }
    // one field forward
    if !board.positions.field_is_used(one_step_forward) {
        if prevent_check_fields.field_is_used(one_step_forward) {
            if figure_will_promote(one_step_forward, &own_color) {
                add_promotion_to_possible_moves(own_position, one_step_forward, possible_moves);
            } else {
                possible_moves.push(PossibleMove {
                    to: one_step_forward,
                    from: own_position,
                    promoted_to: None,
                });
            }
        }

        // two fields forward
        if can_move_two_fields(own_position, own_color) {
            let two_steps_forward = calculate_forward_position(own_position, own_color, 16);

            if prevent_check_fields.field_is_used(two_steps_forward) && !board.positions.field_is_used(two_steps_forward) {
                possible_moves.push(PossibleMove {
                    to: two_steps_forward,
                    from: own_position,
                    promoted_to: None,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::board::bitboard::Bitboard;

    use super::*;

    #[test]
    fn test_normal_move() {
        let mut positions = Bitboard::new();

        positions.set_field(12);

        let board = Chessboard {
            positions,
            ..Default::default()
        };

        let mut moves = Vec::new();
        get_possible_pawn_moves(&board, 12, Color::White, &mut moves);

        assert_eq!(2, moves.len());
    }

    #[test]
    fn test_take_from_a_to_h() {
        let mut board = Chessboard::empty(Color::White);
        board.positions.set_field(16);
        board.positions.set_field(23);
        board.positions.set_field(24);
        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(23);
        let mut moves = Vec::new();
        get_possible_pawn_moves(&board, 16, Color::White, &mut moves);

        // should not be able to take from Field 16(A3) to 23(H3)
        assert_eq!(0, moves.len());
    }

    // Black H Pawn is not working - test driven fix
    #[test]
    fn test_black_h_pawn_with_free_path() {
        let mut board = Chessboard {
            current_move: Color::Black,
            ..Default::default()
        };
        board.set_to_default();
        let mut moves = Vec::new();
        get_possible_pawn_moves(&board, 55, Color::Black, &mut moves);

        assert_eq!(2, moves.len());
    }

    #[test]
    fn test_en_passant_left() {
        let mut board = Chessboard::empty(Color::White);
        board.en_passant = Some(34);
        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(34);
        board.positions.set_field(34);
        board.used_positions[Color::Black as usize].set_field(34);

        let mut moves = Vec::new();
        get_possible_pawn_moves(&board, 35, Color::White, &mut moves);
        let move_fields: Vec<usize> = moves.into_iter().map(|x| x.to).collect();
        assert_eq!(true, move_fields.contains(&42));
    }

    #[test]
    fn test_en_passant_right() {
        let mut positions = Bitboard::new();
        positions.set_field(26);

        let mut board = Chessboard {
            positions,
            current_move: Color::Black,
            en_passant: Some(27),
            ..Default::default()
        };
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(27);

        let mut moves = Vec::new();
        get_possible_pawn_moves(&board, 26, Color::Black, &mut moves);
        let move_fields: Vec<usize> = moves.into_iter().map(|x| x.to).collect();
        assert_eq!(true, move_fields.contains(&19));
    }

    #[test]
    fn test_en_passant_right_not_possible_as_it_would_put_us_in_check() {
        let mut positions = Bitboard::new();

        positions.set_field(26);
        positions.set_field(27);
        positions.set_field(31);
        let mut board = Chessboard {
            positions,
            current_move: Color::Black,
            en_passant: Some(27),
            ..Default::default()
        };
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(27);
        board.figures[Color::White as usize][Piece::Rook as usize].set_field(31);
        board.figures[Color::Black as usize][Piece::King as usize].set_field(24);
        board.used_positions[Color::White as usize].set_field(27);
        board.used_positions[Color::White as usize].set_field(31);
        board.used_positions[Color::Black as usize].set_field(24);

        let mut moves = Vec::new();
        get_possible_pawn_moves(&board, 26, Color::Black, &mut moves);
        let move_fields: Vec<usize> = moves.into_iter().map(|x| x.to).collect();
        assert_eq!(false, move_fields.contains(&19));
    }

    #[test]
    fn test_promotion_white() {
        let mut board = Chessboard::empty(Color::White);
        board.figures[Color::White as usize][Piece::Pawn as usize].set_field(52);

        let mut moves = Vec::new();
        get_possible_pawn_takes_and_promotion(&board, 52, Color::White, &mut moves);
        assert_eq!(4, moves.len())
    }

    #[test]
    fn test_promotion_black_with_capture() {
        let mut board = Chessboard::empty(Color::Black);
        board.used_positions[Color::White as usize].set_field(6);
        board.figures[Color::White as usize][Piece::Knight as usize].set_field(6);
        board.figures[Color::Black as usize][Piece::Pawn as usize].set_field(15);

        let mut moves = Vec::new();
        get_possible_pawn_takes_and_promotion(&board, 15, Color::Black, &mut moves);
        assert_eq!(8, moves.len())
    }
}
