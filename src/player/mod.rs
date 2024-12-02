mod action;

use crate::card::Card;
use crate::player::action::Action;

pub struct Player {
    hand: [Card; 2],
    name: String,
    current_action: Option<Action>
}
impl Player {
    pub fn new(hand: [Card;2], name: String) -> Self {
        Self {
            hand,
            name,
            current_action: None
        }
    }    
}
