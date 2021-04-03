use std::collections::HashMap;

use super::card::Card;



#[derive(Debug)]
pub struct Hand{
    ///key will be card's id
    hand: HashMap<String, Card>,
}

impl Hand{
    pub fn new() -> Self{
        let hand = HashMap::new();
        Self{
            hand,
        }
    }

    pub fn take_card(&mut self, card: Card){
        self.hand.insert(card.get_id(), card);
    }

    pub fn discard(&mut self, card: &str)-> Option<Card> {
        self.hand.remove(card)
    }

    pub fn show_rank_match(&mut self) -> Vec<&str>{
        let mut s = Vec::new();
        s.push("hi");
        s.push("bye");
        s
    }
    


    pub fn have_card(&mut self, card: &str)-> bool{
        match self.hand.get(card){
            Some(v) => true,
            None => false,
        }
    }

    pub fn print_hand(&self)-> String{
        let mut string_hand = String::new();
        string_hand.push_str("| ");
        for (key, value) in &self.hand{
            string_hand.push_str(&value.print_card());
            string_hand.push_str(" | ")
        }
        string_hand
    }
}













/*
///The hand is a vector of cards held 
///by the player
#[derive(Debug)]
pub struct Hand{
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
*/
