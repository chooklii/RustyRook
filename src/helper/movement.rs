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

mod tests {
    use super::*;

    #[test]
    fn test_move_left(){
        assert_eq!(false, figure_can_move_left(&8));
        assert_eq!(true, figure_can_move_left(&15));
        assert_eq!(false, figure_can_move_left(&56));
        assert_eq!(false, figure_can_move_left(&32));
        assert_eq!(true, figure_can_move_left(&25));
        assert_eq!(true, figure_can_move_left(&30));
    }

    #[test]
    fn test_move_right(){
        assert_eq!(false, figure_can_move_right(&7));
        assert_eq!(false, figure_can_move_right(&15));
        assert_eq!(false, figure_can_move_right(&31));
        assert_eq!(false, figure_can_move_right(&39));
        assert_eq!(true, figure_can_move_right(&18));
        assert_eq!(true, figure_can_move_right(&38));
        assert_eq!(true, figure_can_move_right(&16));
    }

    #[test]
    fn test_move_forward(){
        assert_eq!(true, figure_can_move_forward(&27));
        assert_eq!(true, figure_can_move_forward(&27));
        assert_eq!(true, figure_can_move_forward(&0));
        assert_eq!(true, figure_can_move_forward(&0));
        assert_eq!(false, figure_can_move_forward(&60));
    }

    #[test]
    fn test_move_backward(){
        assert_eq!(true, figure_can_move_backward(&27));
        assert_eq!(true, figure_can_move_backward(&27));
        assert_eq!(false, figure_can_move_backward(&0));
        assert_eq!(false, figure_can_move_backward(&0));
        assert_eq!(true, figure_can_move_backward(&60)); 
    }
}