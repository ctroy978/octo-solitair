
use super::card::Card;

#[derive(Debug)]
pub struct Hand{
    pub hand: Vec<Card>,
}

impl Hand{
    pub fn new()-> Self{
        let hand = Vec::new();
        Self{
            hand,
        }
    }
    pub fn take(&mut self, card: Card){
        self.hand.push(card);
    }
    pub fn print_hand(&mut self){
        for card in self.hand.iter(){
            print!("{}s | ",card.print_card());
        }
    }
}

