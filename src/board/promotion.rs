use crate::figures::{bishop::Bishop, figures::Figure, knight::Knight, queen::Queen, rook::Rook};


#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum Promotion{
    Queen,
    Rook,
    Knight,
    Bishop
}

pub fn convert_promotion_to_figure(promoted_to: Promotion) -> Figure{
    match promoted_to{
        Promotion::Queen => Figure::Queen(Queen {}),
        Promotion::Knight => Figure::Knight(Knight {}),
        Promotion::Bishop => Figure::Bishop(Bishop {}),
        Promotion::Rook => Figure::Rook(Rook { ..Default::default() })
    }
}

pub fn convert_promotion_to_output_string(promoted_to: &Option<Promotion>) -> String{
    if promoted_to.is_none(){
        return String::from("");
    }
    let promotion = promoted_to.unwrap();

    return match promotion{
        Promotion::Queen => String::from("Q"),
        Promotion::Knight => String::from("K"),
        Promotion::Bishop => String::from("B"),
        Promotion::Rook => String::from("R")
    }
}

pub fn convert_input_string_to_promotion(promotion_str: Option<&str>) -> Option<Promotion>{
    if promotion_str.is_none(){
        return None
    };
    // null checked prior
    let promotion_string = promotion_str.unwrap();

    return match promotion_string{
        "q" | "Q" => Some(Promotion::Queen),
        "k" | "K" => Some(Promotion::Knight),
        "b" | "B" => Some(Promotion::Bishop),
        "r" | "R" => Some(Promotion::Rook),
        _ => None
    }
}