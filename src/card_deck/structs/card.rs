
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
    pub fn is_greater_rank(&self, other: &Card) -> bool{
        self.rank > other.rank
    }
    ///Returns true if self.rank is equal to the
    ///other card
    pub fn is_equal_rank(&self, other: &Card) -> bool{
        self.rank == other.rank
    }

    pub fn is_equal_suit(&self, other: &Card) -> bool{
        self.suit == other.suit
    }
    ///convert the rank u8 to a String
    /// #example
    /// 13 -> "King"
    pub fn get_string_rank(&self) -> String{
        match self.rank{
            13 => String::from("king"),
            12 => String::from("queen"),
            11 => String::from("jack"),
            1 => String::from("ace"),
            2 => String::from("two"),
            3 => String::from("three"),
            4 => String::from("four"),
            5 => String::from("five"),
            6 => String::from("six"),
            7 => String::from("seven"),
            8 => String::from("eight"),
            9 => String::from("nine"),
            10 => String::from("ten"),
            _ => String::from("joker"),
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
    ///Returns the card id for the player to identify card
    pub fn get_id(&self) -> String{
        let rank_id = match self.get_rank(){
            13 => "k".to_string(),
            12 => "q".to_string(),
            11 => "j".to_string(),
            10 => "10".to_string(),
            0 => "joker".to_string(),
            _ => self.get_rank().to_string(), 
        };
        let suit_id = self.print_suit().chars().next().unwrap();
        format!("{}{}",rank_id,suit_id)
    }
    ///Returns string of the suit
    pub fn print_suit(&self) -> String{
        match self.suit{
           Suit::Spade => String::from("spade"),
           Suit::Club => String::from("club"),
           Suit::Diamond => String::from("diamond"), 
           Suit::Heart => String::from("heart"),
        }
    }
    ///Returns a String representation of the card for
    ///printing on screen. "Three of Spades"
    ///#Example
    ///println!("{}", c.print_card());
    pub fn print_card(&self)-> String{
        format!("{} of {}s", self.get_string_rank(), self.print_suit())
    }

}
