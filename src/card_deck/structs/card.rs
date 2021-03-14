
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
    pub fn get_rank(&self) -> u8{
        self.rank
    }
    pub fn get_suit(&self) -> Suit{
        self.suit
    }

}
