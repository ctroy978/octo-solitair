use super::card::Card;


#[derive(Debug)]
pub struct Deck{
    pub deck: Vec<Card>,
}

impl Deck{
    pub fn new(deck: Vec<Card>) -> Self{
        Self{
            deck
        }
    }
    pub fn deal(&mut self) -> Card{
        self.deck.pop().unwrap()
    }

    pub fn shuffle_deck(&mut self){
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
    pub fn insert_card(&mut self, card: Card){
        self.deck.insert(0, card);
    }
}
