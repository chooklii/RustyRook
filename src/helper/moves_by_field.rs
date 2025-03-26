use std::{cmp::min, collections::HashMap};

/*
 calculate all maximum possible moves in every direction once and then just use them when calculating the moves
*/
#[derive(Debug, Clone)]
pub struct MoveInEveryDirection {
    pub knight_moves: Vec<usize>,
    pub left: usize,
    pub right: usize,
    pub forward: usize,
    pub back: usize,
    pub left_forward: usize,
    pub left_back: usize,
    pub right_forward: usize,
    pub right_back: usize,
}

impl Default for MoveInEveryDirection {
    fn default() -> MoveInEveryDirection {
        MoveInEveryDirection {
            knight_moves: Vec::new(),
            left: 0,
            right: 0,
            forward: 0,
            back: 0,
            left_back: 0,
            left_forward: 0,
            right_back: 0,
            right_forward: 0,
        }
    }
}

pub fn get_moves_for_each_field() -> HashMap<usize, MoveInEveryDirection> {
    let mut values: HashMap<usize, MoveInEveryDirection> = HashMap::new();

    for column in 0..8 {
        for row in 0..8 {
            let left: usize = row;
            let right: usize = 7 - row;
            let forward: usize = 7 - column;
            let back: usize = column;
            let index: usize = usize::from(column * 8 + row);

            values.insert(
                index,
                MoveInEveryDirection {
                    knight_moves: get_knight_moves_for_field(index, left, right, forward, back),
                    // Rook Movement
                    left,
                    right,
                    forward,
                    back,
                    // Bishop Movement
                    left_back: min(left, back),
                    left_forward: min(left, forward),
                    right_back: min(right, back),
                    right_forward: min(right, forward),
                },
            );
        }
    }
    return values;
}

fn get_knight_moves_for_field(
    position: usize,
    left: usize,
    right: usize,
    forward: usize,
    back: usize,
) -> Vec<usize> {
    let mut moves = Vec::new();

    if left >= 2 {
        if back >= 1 {
            moves.push(position - 10);
        }
        if forward >= 1 {
            moves.push(position + 6);
        }
    }
    if right >= 2 {
        if back >= 1 {
            moves.push(position - 6);
        }
        if forward >= 1 {
            moves.push(position + 10);
        }
    }
    if back >= 2 {
        if left >= 1 {
            moves.push(position - 17);
        }
        if right >= 1 {
            moves.push(position - 15);
        }
    }
    if forward >= 2 {
        if left >= 1 {
            moves.push(position + 15);
        }
        if right >= 1 {
            moves.push(position + 17);
        }
    }

    moves
}
