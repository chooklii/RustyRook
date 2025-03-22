pub fn figure_can_move_left(field: &usize) -> bool{
    field % 8 != 0
} 

pub fn figure_can_move_right(field: &usize) -> bool{
    field % 8 != 7
}

pub fn figure_can_move_forward(field: &usize) -> bool{
    field <= &55
}

pub fn figure_can_move_backward(field: &usize) -> bool{
    field >=&8
}


// bishop movements

pub fn figure_can_move_forward_and_left(field: &usize) -> bool{
    figure_can_move_forward(field) && figure_can_move_left(field)
}

pub fn figure_can_move_forward_and_right(field: &usize) -> bool{
    figure_can_move_forward(field) && figure_can_move_right(field)
}

pub fn figure_can_move_backward_and_left(field: &usize) -> bool{
    figure_can_move_backward(field) && figure_can_move_left(field)
}

pub fn figure_can_move_backward_and_right(field: &usize) -> bool{
    figure_can_move_backward(field) && figure_can_move_right(field)
}