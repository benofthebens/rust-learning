mod card;
mod player;

use crate::player::Player;
use crate::card::Card;
use rand::Rng;

fn main() {
    let deck = Card::create_deck(); 
    let random_card_index = rand::thread_rng().gen_range(0..52);
    
    println!("{:?}",deck.get(random_card_index));
}
 
  
   
    
