use crate::rng::GameRng;
use rand::seq::SliceRandom;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck<C>(Vec<C>);

impl<C> Default for Deck<C> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<C> Deck<C> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_cards(cards: Vec<C>) -> Self {
        Self(cards)
    }

    pub fn shuffle(&mut self, rng: &mut GameRng) {
        self.0.shuffle(rng);
    }

    pub fn draw(&mut self) -> Option<C> {
        self.0.pop()
    }

    pub fn push(&mut self, card: C) {
        self.0.push(card);
    }

    pub fn cards(&self) -> &[C] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_takes_from_the_top() {
        let mut deck = Deck::from_cards(vec![1, 2, 3]);
        assert_eq!(deck.draw(), Some(3));
        assert_eq!(deck.len(), 2);
    }

    #[test]
    fn shuffle_is_deterministic_per_seed() {
        let mut a = Deck::from_cards((0..52).collect::<Vec<u8>>());
        let mut b = a.clone();
        a.shuffle(&mut crate::rng::from_seed(7));
        b.shuffle(&mut crate::rng::from_seed(7));
        assert_eq!(a, b);

        let mut c = Deck::from_cards((0..52).collect::<Vec<u8>>());
        c.shuffle(&mut crate::rng::from_seed(8));
        assert_ne!(a, c);
    }
}
