
use super::card::Card;

///The hand is a vector of cards held 
///by the player
#[derive(Debug)]
pub struct Hand{
    ///A hand must be a vector of type Card
    pub hand: Vec<Card>,
}

impl Hand{
    ///Returns an empty hand   
    pub fn new()-> Self{
        let hand = Vec::new();
        Self{
            hand,
        }
    }
    ///puts one card into hand
    pub fn take(&mut self, card: Card){
        self.hand.push(card);
    }
    ///removes a card from hand
    pub fn discard(&mut self)-> Card{
        self.hand.pop().unwrap()
    }
    ///Returns a String representation of the hand for
    ///printing on screen. 
    ///println!("{}", c.print_hand());
    pub fn print_hand(&mut self)-> String{
        let mut string_hand = String::new();
        string_hand.push_str("| ");
        for c in &self.hand{
            string_hand.push_str(&c.print_card());
            string_hand.push_str(" | ");
        }
        string_hand
        }
}

