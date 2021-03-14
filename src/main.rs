mod card_deck;

fn main() {
    //let c1 = card_deck::card_factory(card_deck::Suit::Club, 5);

    let mut d1 = card_deck::deck_factory();
    let c1 = d1.deal();
    println!("{:?}", c1);

}
