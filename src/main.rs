mod card_deck;
mod go_fish;

fn main() {

    let mut d1 = card_deck::deck_factory();
    let mut h1 = card_deck::hand_factory();
    let mut h2 = card_deck::hand_factory();

    h2.take_card(d1.deal());
    h2.take_card(d1.deal());
    h2.take_card(d1.deal());
    
    println!("H2: {}", h2.print_hand());
    println!("have kofd {}", h2.have_card("kd"));
    
    let discarded = if let Some(v) =  h2.discard("kd"){
        v
    }else{
        panic!("Discard failed: That card does not exist.")
    };
    
    
    println!("after discard: {}", h2.print_hand());
    println!("card discarded: {:?}", discarded);


    let mut f1 = go_fish::player_factory(h1);
    let mut fd = go_fish::fish_deck_factory();
    f1.hand.take_card(fd.deal());
    f1.hand.take_card(fd.deal());

    go_fish::play::play_fish();
}


