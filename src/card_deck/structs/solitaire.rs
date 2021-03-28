use super::card::Card;
use super::deck::Deck;
use super::hand::Hand;

#[derive(Debug)]
pub struct Solitaire{
    pub draw: Deck,
    pub discard: Hand,
    pub spade_pile: Hand,
    pub table_one: Hand,
    pub table_two: Hand,
}

impl Solitaire{
    pub fn new(draw: Deck, discard: Hand, 
               spade_pile: Hand, table_one: Hand,
               table_two: Hand)-> Self{
        Self{
            draw,
            discard,
            spade_pile,
            table_one,
            table_two,
        }

    } 
}
