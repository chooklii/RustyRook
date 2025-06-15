use crate::figures::piece::Piece;


#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Promotion{
    Queen,
    Rook,
    Knight,
    Bishop
}

pub fn convert_promotion_to_figure(promoted_to: Promotion) -> Piece{
    match promoted_to{
        Promotion::Queen => Piece::Queen,
        Promotion::Knight => Piece::Knight,
        Promotion::Bishop => Piece::Bishop,
        Promotion::Rook => Piece::Rook
    }
}

pub fn convert_input_string_to_promotion(promotion_str: Option<&str>) -> Option<Promotion>{
    promotion_str?;
    // null checked prior
    let promotion_string = promotion_str.unwrap();

    match promotion_string{
        "q" | "Q" => Some(Promotion::Queen),
        "k" | "K" => Some(Promotion::Knight),
        "b" | "B" => Some(Promotion::Bishop),
        "r" | "R" => Some(Promotion::Rook),
        _ => None
    }
}

pub fn convert_promotion_to_output_string(promoted_to: Option<Promotion>) -> String{
    if promoted_to.is_none(){
        return String::from("");
    }
    let promotion = promoted_to.unwrap();

    match promotion{
        Promotion::Queen => String::from("Q"),
        Promotion::Knight => String::from("N"),
        Promotion::Bishop => String::from("B"),
        Promotion::Rook => String::from("R")
    }
}