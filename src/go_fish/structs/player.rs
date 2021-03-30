use crate::card_deck::structs::card::Card;
use crate::card_deck::structs::hand::Hand;


#[derive(Debug)]
pub struct Player{
    pub hand: Hand,
}


impl Player{
    pub fn new(hand: Hand) -> Self{
        Self{
            hand
        }
    }
}
