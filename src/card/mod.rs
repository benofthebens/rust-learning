pub mod suit;
pub mod rank;

pub use rank::Rank;
pub use suit::Suit;

#[derive(Debug,Copy,Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self{
        Self {
            rank,
            suit
        }
    }
    pub fn create_deck() -> Vec<Self> {
        let mut deck = Vec::new();
        for suit in Suit::iterator() {

            for rank in Rank::iterator() {
                deck.push(Card::new(rank,suit));
            }
        }
        deck
    }
    pub fn get_rank(&self) -> Rank {
        self.rank
    }
    pub fn get_suit(&self) -> Suit {
        self.suit
    }

}
