use super::action::{SolitaireAction, SolitaireEvent, Source, Target};
use super::state::{Column, SolitaireState, TABLEAU_COLUMNS};
use crate::cards::french::{Card, Rank};
use crate::engine::Engine;
use crate::error::{GamesError, GamesResult};

#[derive(Debug, Default, Clone, Copy)]
pub struct Solitaire;

impl Solitaire {
    pub fn has_moves(&self, state: &SolitaireState) -> bool {
        has_any_move(state)
    }
}

impl Engine for Solitaire {
    type State = SolitaireState;
    type Action = SolitaireAction;
    type Event = SolitaireEvent;

    fn apply(
        &self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> GamesResult<Vec<Self::Event>> {
        let mut events = match action {
            SolitaireAction::Draw => draw(state)?,
            SolitaireAction::Collect => collect(state)?,
            SolitaireAction::Move { from, to } => move_cards(state, from, to)?,
        };
        if is_won(state) {
            events.push(SolitaireEvent::Won);
        }
        Ok(events)
    }

    fn is_over(&self, state: &Self::State) -> bool {
        is_won(state)
    }
}

fn invalid(message: &str) -> GamesError {
    GamesError::InvalidAction(message.to_string())
}

fn order(rank: Rank) -> u8 {
    match rank {
        Rank::Ace => 1,
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ten => 10,
        Rank::Jack => 11,
        Rank::Queen => 12,
        Rank::King => 13,
    }
}

fn is_won(state: &SolitaireState) -> bool {
    state.foundations.iter().map(Vec::len).sum::<usize>() == 52
}

fn has_any_move(state: &SolitaireState) -> bool {
    for &card in state.stock.iter().chain(state.waste.iter()) {
        if foundation_target(state, card).is_some() {
            return true;
        }
        if state
            .tableau
            .iter()
            .any(|column| tableau_accepts(column, card))
        {
            return true;
        }
    }

    for column in &state.tableau {
        if column.face_down < column.cards.len()
            && let Some(&card) = column.cards.last()
            && foundation_target(state, card).is_some()
        {
            return true;
        }
    }

    for from in 0..TABLEAU_COLUMNS {
        for to in 0..TABLEAU_COLUMNS {
            if from == to {
                continue;
            }
            let source = &state.tableau[from];
            let destination = &state.tableau[to];
            if let Some(start) = movable_run_start(source, destination)
                && (!destination.cards.is_empty() || start > 0)
            {
                return true;
            }
        }
    }

    false
}

fn draw(state: &mut SolitaireState) -> GamesResult<Vec<SolitaireEvent>> {
    if let Some(card) = state.stock.pop() {
        state.waste.push(card);
        Ok(vec![SolitaireEvent::Drew])
    } else if !state.waste.is_empty() {
        state.stock = state.waste.drain(..).rev().collect();
        Ok(vec![SolitaireEvent::Recycled])
    } else {
        Err(invalid("stock and waste are empty"))
    }
}

fn collect(state: &mut SolitaireState) -> GamesResult<Vec<SolitaireEvent>> {
    if let Some(&card) = state.waste.last()
        && foundation_target(state, card).is_some()
    {
        return waste_to_foundation(state);
    }
    for column in 0..TABLEAU_COLUMNS {
        let source = &state.tableau[column];
        if source.face_down < source.cards.len()
            && let Some(&card) = source.cards.last()
            && foundation_target(state, card).is_some()
        {
            return tableau_to_foundation(state, column);
        }
    }
    Err(invalid("no card can move to a foundation"))
}

fn move_cards(
    state: &mut SolitaireState,
    from: Source,
    to: Target,
) -> GamesResult<Vec<SolitaireEvent>> {
    match (from, to) {
        (Source::Waste, Target::Foundation) => waste_to_foundation(state),
        (Source::Waste, Target::Tableau(i)) => waste_to_tableau(state, i),
        (Source::Tableau(a), Target::Foundation) => tableau_to_foundation(state, a),
        (Source::Tableau(a), Target::Tableau(b)) => tableau_to_tableau(state, a, b),
        (Source::Foundation(k), Target::Tableau(i)) => foundation_to_tableau(state, k, i),
        _ => Err(invalid("unsupported move")),
    }
}

fn foundation_accepts(pile: &[Card], card: Card) -> bool {
    match pile.last() {
        None => card.rank == Rank::Ace,
        Some(top) => top.suit == card.suit && order(card.rank) == order(top.rank) + 1,
    }
}

fn foundation_target(state: &SolitaireState, card: Card) -> Option<usize> {
    state
        .foundations
        .iter()
        .position(|pile| foundation_accepts(pile, card))
}

fn tableau_accepts(column: &Column, head: Card) -> bool {
    match column.cards.last() {
        None => head.rank == Rank::King,
        Some(top) => {
            column.face_down < column.cards.len()
                && order(top.rank) == order(head.rank) + 1
                && top.suit.is_red() != head.suit.is_red()
        }
    }
}

fn is_run(cards: &[Card]) -> bool {
    cards.windows(2).all(|pair| {
        order(pair[0].rank) == order(pair[1].rank) + 1
            && pair[0].suit.is_red() != pair[1].suit.is_red()
    })
}

fn movable_run_start(from: &Column, to: &Column) -> Option<usize> {
    (from.face_down..from.cards.len()).find(|&start| {
        let run = &from.cards[start..];
        is_run(run) && tableau_accepts(to, run[0])
    })
}

fn flip_if_needed(column: &mut Column) -> bool {
    if !column.cards.is_empty() && column.face_down == column.cards.len() {
        column.face_down -= 1;
        true
    } else {
        false
    }
}

fn waste_to_foundation(state: &mut SolitaireState) -> GamesResult<Vec<SolitaireEvent>> {
    let card = *state
        .waste
        .last()
        .ok_or_else(|| invalid("waste is empty"))?;
    let foundation =
        foundation_target(state, card).ok_or_else(|| invalid("no foundation accepts that card"))?;
    state.waste.pop();
    state.foundations[foundation].push(card);
    Ok(vec![SolitaireEvent::Moved])
}

fn waste_to_tableau(state: &mut SolitaireState, column: usize) -> GamesResult<Vec<SolitaireEvent>> {
    let card = *state
        .waste
        .last()
        .ok_or_else(|| invalid("waste is empty"))?;
    if !tableau_accepts(&state.tableau[column], card) {
        return Err(invalid("card does not fit that column"));
    }
    state.waste.pop();
    state.tableau[column].cards.push(card);
    Ok(vec![SolitaireEvent::Moved])
}

fn tableau_to_foundation(
    state: &mut SolitaireState,
    column: usize,
) -> GamesResult<Vec<SolitaireEvent>> {
    let source = &state.tableau[column];
    let card = *source
        .cards
        .last()
        .ok_or_else(|| invalid("column is empty"))?;
    if source.face_down >= source.cards.len() {
        return Err(invalid("top card is face down"));
    }
    let foundation =
        foundation_target(state, card).ok_or_else(|| invalid("no foundation accepts that card"))?;
    state.tableau[column].cards.pop();
    state.foundations[foundation].push(card);
    let mut events = vec![SolitaireEvent::Moved];
    if flip_if_needed(&mut state.tableau[column]) {
        events.push(SolitaireEvent::Flipped(column));
    }
    Ok(events)
}

fn tableau_to_tableau(
    state: &mut SolitaireState,
    from: usize,
    to: usize,
) -> GamesResult<Vec<SolitaireEvent>> {
    if from == to {
        return Err(invalid("cannot move a column onto itself"));
    }
    let start = movable_run_start(&state.tableau[from], &state.tableau[to])
        .ok_or_else(|| invalid("no legal run to move"))?;
    let run = state.tableau[from].cards.split_off(start);
    state.tableau[to].cards.extend(run);
    let mut events = vec![SolitaireEvent::Moved];
    if flip_if_needed(&mut state.tableau[from]) {
        events.push(SolitaireEvent::Flipped(from));
    }
    Ok(events)
}

fn foundation_to_tableau(
    state: &mut SolitaireState,
    foundation: usize,
    column: usize,
) -> GamesResult<Vec<SolitaireEvent>> {
    let card = *state.foundations[foundation]
        .last()
        .ok_or_else(|| invalid("foundation is empty"))?;
    if !tableau_accepts(&state.tableau[column], card) {
        return Err(invalid("card does not fit that column"));
    }
    state.foundations[foundation].pop();
    state.tableau[column].cards.push(card);
    Ok(vec![SolitaireEvent::Moved])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::french::Suit;

    fn card(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn ace_from_waste_starts_a_foundation() {
        let mut state = SolitaireState::default();
        state.waste.push(card(Rank::Ace, Suit::Spades));

        let events = Solitaire
            .apply(
                &mut state,
                SolitaireAction::Move {
                    from: Source::Waste,
                    to: Target::Foundation,
                },
            )
            .unwrap();

        assert!(events.contains(&SolitaireEvent::Moved));
        assert!(state.waste.is_empty());
        assert_eq!(state.foundations[0], vec![card(Rank::Ace, Suit::Spades)]);
    }

    #[test]
    fn tableau_move_reveals_the_buried_card() {
        let mut state = SolitaireState::default();
        state.tableau[0].cards = vec![
            card(Rank::Five, Suit::Clubs),
            card(Rank::Seven, Suit::Hearts),
        ];
        state.tableau[0].face_down = 1;
        state.tableau[1].cards = vec![card(Rank::Eight, Suit::Spades)];
        state.tableau[1].face_down = 0;

        Solitaire
            .apply(
                &mut state,
                SolitaireAction::Move {
                    from: Source::Tableau(0),
                    to: Target::Tableau(1),
                },
            )
            .unwrap();

        assert_eq!(state.tableau[0].cards, vec![card(Rank::Five, Suit::Clubs)]);
        assert_eq!(state.tableau[0].face_down, 0);
        assert_eq!(state.tableau[1].cards.len(), 2);
    }

    #[test]
    fn illegal_move_leaves_state_untouched() {
        let mut state = SolitaireState::default();
        state.waste.push(card(Rank::Five, Suit::Clubs));
        let before = state.clone();

        let result = Solitaire.apply(
            &mut state,
            SolitaireAction::Move {
                from: Source::Waste,
                to: Target::Foundation,
            },
        );

        assert!(result.is_err());
        assert_eq!(state, before);
    }

    #[test]
    fn empty_draw_recycles_the_waste() {
        let mut state = SolitaireState {
            waste: vec![card(Rank::Two, Suit::Clubs), card(Rank::Three, Suit::Clubs)],
            ..Default::default()
        };

        let events = Solitaire.apply(&mut state, SolitaireAction::Draw).unwrap();

        assert!(events.contains(&SolitaireEvent::Recycled));
        assert!(state.waste.is_empty());
        assert_eq!(state.stock.pop(), Some(card(Rank::Two, Suit::Clubs)));
    }

    #[test]
    fn a_full_board_is_won() {
        let mut state = SolitaireState::default();
        for (i, suit) in Suit::ALL.into_iter().enumerate() {
            state.foundations[i] = Rank::ALL.into_iter().map(|rank| card(rank, suit)).collect();
        }
        assert!(Solitaire.is_over(&state));
    }

    #[test]
    fn collect_sends_a_ready_card_to_a_foundation() {
        let mut state = SolitaireState {
            waste: vec![card(Rank::Ace, Suit::Hearts)],
            ..Default::default()
        };

        Solitaire
            .apply(&mut state, SolitaireAction::Collect)
            .unwrap();

        assert!(state.waste.is_empty());
        assert_eq!(state.foundations.iter().map(Vec::len).sum::<usize>(), 1);
    }

    #[test]
    fn collect_with_nothing_ready_is_rejected() {
        let mut state = SolitaireState {
            waste: vec![card(Rank::Five, Suit::Clubs)],
            ..Default::default()
        };
        assert!(
            Solitaire
                .apply(&mut state, SolitaireAction::Collect)
                .is_err()
        );
    }

    #[test]
    fn relocating_a_lone_king_to_an_empty_column_is_not_a_move() {
        let mut state = SolitaireState::default();
        state.tableau[0].cards = vec![card(Rank::King, Suit::Spades)];
        let reds = [
            card(Rank::Two, Suit::Hearts),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Six, Suit::Hearts),
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
        ];
        for (column, blocker) in state.tableau[2..].iter_mut().zip(reds) {
            column.cards = vec![blocker];
        }
        assert!(!Solitaire.has_moves(&state));
    }

    #[test]
    fn a_fresh_deal_has_moves() {
        let mut rng = crate::rng::from_seed(1);
        let state = SolitaireState::deal(&mut rng);
        assert!(Solitaire.has_moves(&state));
    }

    #[test]
    fn a_blocked_board_has_no_moves() {
        let mut state = SolitaireState::default();
        let blockers = [
            card(Rank::Two, Suit::Hearts),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Six, Suit::Hearts),
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
        ];
        for (column, blocker) in state.tableau.iter_mut().zip(blockers) {
            column.cards = vec![blocker];
            column.face_down = 0;
        }
        assert!(!Solitaire.has_moves(&state));
    }
}
