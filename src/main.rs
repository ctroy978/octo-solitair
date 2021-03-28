mod card_deck;

fn main() {

    let mut d1 = card_deck::deck_factory();
    println!("{:?}", d1);
    let mut h1 = card_deck::hand_factory();

    h1.take(d1.deal());
    h1.take(d1.deal());
    println!("{}", h1.print_hand());

}


