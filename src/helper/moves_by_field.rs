use std::cmp::min;

use rustc_hash::FxHashMap;

/*
 calculate all maximum possible moves in every direction once and then just use them when calculating the moves
*/
#[derive(Debug, Clone)]
pub struct MoveInEveryDirection {
    pub knight_moves: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
    pub forward: Vec<usize>,
    pub back: Vec<usize>,
    pub left_forward: Vec<usize>,
    pub left_back: Vec<usize>,
    pub right_forward: Vec<usize>,
    pub right_back: Vec<usize>,
}

impl Default for MoveInEveryDirection {
    fn default() -> MoveInEveryDirection {
        MoveInEveryDirection {
            knight_moves: Vec::new(),
            left: Vec::new(),
            right: Vec::new(),
            forward: Vec::new(),
            back: Vec::new(),
            left_back: Vec::new(),
            left_forward: Vec::new(),
            right_back: Vec::new(),
            right_forward: Vec::new(),
        }
    }
}

pub fn get_moves_for_each_field() -> FxHashMap<usize, MoveInEveryDirection> {
    let mut values: FxHashMap<usize, MoveInEveryDirection> = FxHashMap::default();

    for column in 0..8 {
        for row in 0..8 {
            let mut left = Vec::new();
            let mut right= Vec::new();
            let mut forward= Vec::new();
            let mut back= Vec::new();
            let mut left_forward = Vec::new();
            let mut left_back= Vec::new();
            let mut right_forward= Vec::new();
            let mut right_back= Vec::new();
            let index: usize = usize::from(column * 8 + row);

            for val in 0..row{
                left.push(index - val -1)
            };
            for val in 0..(7-row){
                right.push(index + val + 1)
            }

            for val in 1..=(7-column){
                forward.push(index + (val*8))
            }
            
            for val in 1..=column{
                back.push(index - (val*8))
            };

            // Bishop
            let left_fw_moves = min(left.len(), forward.len());
            let left_bw_moves = min(left.len(), back.len());
            let right_fw_moves = min(right.len(), forward.len());
            let right_bw_moves = min(right.len(), back.len()); 

            for val in 1..=left_fw_moves{
                left_forward.push(index + (val*7))
            };
            for val in 1..=left_bw_moves{
                left_back.push(index - (val*9))
            };
            for val in 1..=right_fw_moves{
                right_forward.push(index + (val*9))
            };
            for val in 1..=right_bw_moves{
                right_back.push(index - (val*7))
            };
            values.insert(
                index,
                MoveInEveryDirection {
                    knight_moves: get_knight_moves_for_field(index, &left, &right, &forward, &back),
                    // Bishop Movement
                    left_back,
                    left_forward,
                    right_back,
                    right_forward,
                    // Rook Movement
                    left,
                    right,
                    forward,
                    back,
                },
            );
        }
    }
    return values;
}

fn get_knight_moves_for_field(
    position: usize,
    left: &Vec<usize>,
    right: &Vec<usize>,
    forward: &Vec<usize>,
    back: &Vec<usize>,
) -> Vec<usize> {
    let mut moves = Vec::new();

    if left.len() >= 2 {
        if back.len() >= 1 {
            moves.push(position - 10);
        }
        if forward.len() >= 1 {
            moves.push(position + 6);
        }
    }
    if right.len() >= 2 {
        if back.len() >= 1 {
            moves.push(position - 6);
        }
        if forward.len() >= 1 {
            moves.push(position + 10);
        }
    }
    if back.len() >= 2 {
        if left.len() >= 1 {
            moves.push(position - 17);
        }
        if right.len() >= 1 {
            moves.push(position - 15);
        }
    }
    if forward.len() >= 2 {
        if left.len() >= 1 {
            moves.push(position + 15);
        }
        if right.len() >= 1 {
            moves.push(position + 17);
        }
    }

    moves
}
