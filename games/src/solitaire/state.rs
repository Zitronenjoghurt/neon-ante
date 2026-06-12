use crate::cards::deck::Deck;
use crate::cards::french::Card;
use crate::rng::GameRng;

pub const TABLEAU_COLUMNS: usize = 7;
pub const FOUNDATION_COUNT: usize = 4;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Column {
    pub cards: Vec<Card>,
    pub face_down: usize,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolitaireState {
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub foundations: [Vec<Card>; FOUNDATION_COUNT],
    pub tableau: [Column; TABLEAU_COLUMNS],
}

impl Default for SolitaireState {
    fn default() -> Self {
        Self {
            stock: Vec::new(),
            waste: Vec::new(),
            foundations: std::array::from_fn(|_| Vec::new()),
            tableau: std::array::from_fn(|_| Column::default()),
        }
    }
}

impl SolitaireState {
    pub fn deal(rng: &mut GameRng) -> Self {
        let mut deck = Deck::standard();
        deck.shuffle(rng);

        let mut state = Self::default();
        for (col, column) in state.tableau.iter_mut().enumerate() {
            for _ in 0..=col {
                if let Some(card) = deck.draw() {
                    column.cards.push(card);
                }
            }
            column.face_down = col;
        }
        state.stock = deck.cards().to_vec();
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_lays_out_a_standard_klondike() {
        let mut rng = crate::rng::from_seed(1);
        let state = SolitaireState::deal(&mut rng);

        let tableau_total: usize = state.tableau.iter().map(|c| c.cards.len()).sum();
        assert_eq!(tableau_total, 28);
        assert_eq!(state.stock.len(), 24);

        for (i, column) in state.tableau.iter().enumerate() {
            assert_eq!(column.cards.len(), i + 1);
            assert_eq!(column.face_down, i);
        }
    }
}
