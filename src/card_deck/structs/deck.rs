use super::card::Card;


#[derive(Debug)]
pub struct Deck{
    pub deck: Vec<Card>,
}

impl Deck{
    pub fn new(deck: Vec<Card>) -> Self{
        Self{
            deck
        }
    }
    pub fn deal(&mut self) -> Card{
        self.deck.pop().unwrap()
    }
}
