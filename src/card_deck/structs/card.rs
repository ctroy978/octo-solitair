
use super::super::Suit;


#[derive(Debug)]
pub struct Card{
    pub suit: Suit,
    pub rank: u8,
}

impl Card{
    pub fn new(suit: Suit, rank: u8) -> Self{
        Self{
            suit,
            rank,
        }
    }
    pub fn is_greater(&self, other: &Card) -> bool{
        self.rank > other.rank
    }
    pub fn is_equal(&self, other: &Card) -> bool{
        self.rank == other.rank
    }
    pub fn get_string_rank(&self) -> String{
        match self.rank{
            13 => String::from("King"),
            12 => String::from("Queen"),
            11 => String::from("Jack"),
            1 => String::from("Ace"),
            2 => String::from("Two"),
            3 => String::from("Three"),
            4 => String::from("Four"),
            5 => String::from("Five"),
            6 => String::from("Six"),
            7 => String::from("Seven"),
            8 => String::from("Eight"),
            9 => String::from("Nine"),
            10 => String::from("Ten"),
            _ => String::from("Joker"),
        }
    }
    pub fn get_rank(&self) -> u8{
        self.rank
    }
    pub fn get_suit(&self) -> Suit{
        self.suit
    }
    pub fn print_card(&self)-> String{
        format!("{} of {:?}s", self.get_string_rank(), self.get_suit())
    }

}
