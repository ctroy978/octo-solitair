pub mod structs;

use structs::deck::Deck;
use structs::card::Card;
use structs::hand::Hand;

const ACE: u8 = 1;
const KING: u8 = 13;

#[derive(Debug,Clone,Copy)]
pub enum Suit{
    Spade,
    Club,
    Heart,
    Diamond,
}

pub fn card_factory(suit: Suit, rank: u8) -> Card{
    Card::new(suit, rank)
}

pub fn deck_factory() -> Deck{
    let suits: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];
    let mut cards = Vec::new();

    for suit in suits.iter(){
        for i in ACE..KING + 1{
            cards.push(card_factory(*suit, i));
        }
    }
    Deck::new(cards)
}

