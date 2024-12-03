mod action;

use crate::card::Card;
use crate::player::action::Action;
#[derive(Clone)]
pub struct Player {
    hand: [Option<Card>; 2],
    name: String,
    current_action: Option<Action>
}
impl Player {
    pub fn new(hand: [Option<Card>;2], name: String) -> Self {
        Self {
            hand,
            name,
            current_action: None
        }
    }    
}
