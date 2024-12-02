#[derive(Debug, Copy, Clone)]
pub enum Suit {
    SPADES,
    CLUBS,
    HEARTS,
    DIAMONDS
}
impl Suit {
    pub fn iterator() -> impl Iterator<Item = Suit> {
        [Suit::HEARTS, Suit::DIAMONDS, Suit::CLUBS, Suit::SPADES].iter().copied()
    }
}
