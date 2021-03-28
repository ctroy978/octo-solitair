
use super::super::Suit;


#[derive(Debug)]
pub struct Card{
    ///a card must have a Suit and a rank u8
    pub suit: Suit,
    pub rank: u8,
}

impl Card{
    //Returns a card with a Suit and rank
    ///
    /// #Arguments
    ///
    /// *'suit' --a suit from the Suit enum
    /// *'rank' --a u8
    ///
    ///This is usually created in the card_factory
    ///in the top level mod.rs
    /// 
    pub fn new(suit: Suit, rank: u8) -> Self{
        Self{
            suit,
            rank,
        }
    }
    ///returns true if self.rank is greater than
    ///the other card.
    pub fn is_greater(&self, other: &Card) -> bool{
        self.rank > other.rank
    }
    ///Returns true if self.rank is equal to the
    ///other card
    pub fn is_equal(&self, other: &Card) -> bool{
        self.rank == other.rank
    }
    ///convert the rank u8 to a String
    /// #example
    /// 13 -> "King"
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
    ///Returns the rank of the card as a u8
    pub fn get_rank(&self) -> u8{
        self.rank
    }
    ///Returns the Suit of the card as an enum
    pub fn get_suit(&self) -> Suit{
        self.suit
    }
    ///Returns a String representation of the card for
    ///printing on screen. "Three of Spades"
    ///#Example
    ///println!("{}", c.print_card());
    pub fn print_card(&self)-> String{
        format!("{} of {:?}s", self.get_string_rank(), self.get_suit())
    }

}
