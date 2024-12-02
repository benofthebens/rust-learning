#[derive(Debug, Clone, Copy)]
pub enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    JACK,
    QUEEN,
    KING
}
impl Rank {
    pub fn iterator() -> impl Iterator<Item = Rank> {
        [
            Rank::ACE,
            Rank::TWO,
            Rank::THREE,
            Rank::FOUR,
            Rank::FIVE, 
            Rank::SIX, 
            Rank::SEVEN, 
            Rank::EIGHT, 
            Rank::NINE,
            Rank::JACK,
            Rank::QUEEN,
            Rank::KING,
        ].iter().copied() 
    
    }
}
