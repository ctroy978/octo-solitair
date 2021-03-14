mod card_deck;

fn main() {
    //let c1 = card_deck::card_factory(card_deck::Suit::Club, 5);

    let mut d1 = card_deck::deck_factory();
    println!("{:?}", d1.deck[12]);
    d1.shuffle_deck();
    println!("{:?}", d1.deck[12]);

    let mut h1 = card_deck::structs::hand::Hand::new();
    h1.take(d1.deal());
    h1.take(d1.deal());
    h1.take(d1.deal());
    h1.take(d1.deal());
    h1.take(d1.deal());
    println!("{:?}", h1);
    h1.print_hand();


}
