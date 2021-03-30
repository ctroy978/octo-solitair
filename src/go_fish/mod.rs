pub mod structs;
pub mod play;

use structs::player::Player;

use crate::card_deck;
use crate::card_deck::Suit;
use crate::card_deck::structs::card::Card;
use crate::card_deck::structs::hand::Hand;
use crate::card_deck::structs::deck::Deck;


pub fn player_factory(hand: Hand)-> Player{
    let player = Player::new(hand);
    player
}



///makes all the cards for a game of fish
///and fills a deck with them -- 1-10 only
pub fn fish_deck_factory() -> Deck{
    let suits: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];
    let mut cards = Vec::new();

    for suit in suits.iter(){
        for i in 1..11{
            cards.push(card_deck::card_factory(*suit, i));
        }
    }
    Deck::new(cards)
}



