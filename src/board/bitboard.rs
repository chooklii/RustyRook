use std::usize;

#[derive(Clone, Debug, Copy)]
pub struct Bitboard{
    pub board: u64
}

impl Bitboard{

    pub fn new() -> Bitboard{
        Bitboard { board: 0 }
    }

    pub fn field_is_used(&self, position: usize) -> bool{
        (self.board >> position) & 1 != 0
    }

    pub fn set_field(&mut self, position: usize){
        self.board |= 1 << position;
    }

    pub fn remove_field(&mut self, position: usize){
        self.board &= !(1<<position);
    }

    // this should only be used for tests and statics - bad performance compared to iterate
    pub fn get_used_fields(&self) -> Vec<usize>{
        let mut fields = Vec::new();
        self.iterate_board(|position| fields.push(position));
        fields
    }

    // mainly used to find king which is only one 
    pub fn get_first_field(&self) -> usize{
        self.board.trailing_zeros() as usize
    }

    // todo perf analyze other options
    pub fn iterate_board(&self, mut adder: impl FnMut(usize)){
        let mut iterated_board = self.board;
        while iterated_board != 0{
            let zeros = iterated_board.trailing_zeros() as usize;
            adder(zeros);
            iterated_board &= iterated_board -1;
        }
    } 
}

#[cfg(test)]

mod tests{

    use super::*;

    #[test]
    fn test_adding_and_removing(){
        let mut board = Bitboard::new();
        board.set_field(5);
        assert_eq!(true, board.field_is_used(5));
        board.remove_field(5);
        assert_eq!(false, board.field_is_used(5));
    }

    #[test]
    fn test_getting_all_used_fields(){
        let mut board = Bitboard::new();
        board.set_field(5);
        board.set_field(10);
        board.set_field(15);
        board.remove_field(10);
        let used = board.get_used_fields();
        assert_eq!(2, used.len());

        board.set_field(10);
        let used = board.get_used_fields();
        assert_eq!(3, used.len());
    }
}