use super::card::Card;


///A deck of cards represented here
#[derive(Debug)]
pub struct Deck{
    ///the deck must have a vector of Card
    pub deck: Vec<Card>,
}

impl Deck{
    ///returns a deck of cars
    ///
    ///#Arguments
    ///
    ///*deck -- a Vector of type Card
    ///
    ///best to create with call to 
    ///deck_factory in the create's mod.rs 
    ///file
    ///deck_factory()
    pub fn new(deck: Vec<Card>) -> Self{
        Self{
            deck
        }
    }
    ///Returns the top card on the deck, and removes that
    ///card from the deck
    pub fn deal(&mut self) -> Card{
        self.deck.pop().unwrap()
    }

    pub fn wrap_some<T>(value: T) -> Option<T>{
        let x: Option<T> = Some(value);
        return x
    }

    pub fn take_card(&mut self, card: &str) -> Option<Card>{
        for (i,c) in self.deck.iter().enumerate(){
            if c.get_id() == card{
                return Some(self.deck.remove(i));
            }
        }
        return None;
}


    ///shuffles the deck in place
    pub fn shuffle_deck(&mut self){
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
    ///returns card to the 0 position on the deck
    ///-- the bottom of the deck
    pub fn insert_card(&mut self, card: Card){
        self.deck.insert(0, card);
    }
}
