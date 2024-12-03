mod card;
mod player;

use crate::player::Player;
use crate::card::Card;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let deck = Card::create_deck(); 
    let mut players: [Option<Player>; 4] = Default::default();

    let mut chosen_cards: HashMap<usize, Card> = HashMap::new();
    println!("{:?}", deck.len());
    for player in players {

        let mut player_deck: [Option<Card>; 2] = [None; 2];
        let mut i = 0;
        loop {
            if i == 2 {
                break;
            }
            let random_card_index = rand::thread_rng().gen_range(0..52); 
            if chosen_cards.contains_key(&random_card_index) {
                continue;
            }
            chosen_cards.insert(
                random_card_index + 1,
                deck[random_card_index]
            );
            player_deck[i] = Some(deck[random_card_index]);
    
            i += 1;
        }
        
    }
    println!("{:?}", chosen_cards);
    
}
 
  
   
    
