use std::cell::RefCell;
use std::rc::Rc;

use neon_ante_core::app::{App, Screen};
use neon_ante_core::event::{Key, Msg};
use neon_ante_core::games::Game;
use neon_ante_core::games::Outcome;
use neon_ante_core::games::solitaire::SolitaireGame;
use neon_ante_core::persistence::{self, PersistenceBackend};
use neon_ante_core::ratatui::Terminal;
use neon_ante_core::ratatui::backend::TestBackend;
use neon_ante_core::rules::cards::french::{Card, Rank, Suit};
use neon_ante_core::rules::solitaire::SolitaireState;
use neon_ante_core::store::{GameRecord, Store};

fn card(rank: Rank, suit: Suit) -> Card {
    Card { rank, suit }
}

fn full_foundation(suit: Suit) -> Vec<Card> {
    use Rank::*;
    [
        Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
    ]
    .into_iter()
    .map(|rank| card(rank, suit))
    .collect()
}

fn key(app: &mut App, key: Key) {
    app.update(Msg::Key(key));
}

fn game_moves(app: &App) -> u32 {
    app.game.as_ref().unwrap().moves()
}

fn render(app: &App, w: u16, h: u16) {
    let mut term = Terminal::new(TestBackend::new(w, h)).unwrap();
    term.draw(|f| app.render(f)).unwrap();
}

#[derive(Debug, Default, Clone)]
struct MemoryBackend(Rc<RefCell<Option<Vec<u8>>>>);

impl PersistenceBackend for MemoryBackend {
    fn load(&self) -> Option<Vec<u8>> {
        self.0.borrow().clone()
    }

    fn save(&self, data: &[u8]) {
        *self.0.borrow_mut() = Some(data.to_vec());
    }
}

#[test]
fn every_screen_renders() {
    let mut app = App::default();
    render(&app, 80, 30);

    key(&mut app, Key::Enter);
    render(&app, 80, 30);

    key(&mut app, Key::Esc);
    key(&mut app, Key::Down);
    key(&mut app, Key::Enter);
    assert_eq!(app.screen, Screen::Scores);
    render(&app, 80, 30);
}

#[test]
fn esc_returns_to_title_and_keeps_the_game_to_resume() {
    let mut app = App::default();
    key(&mut app, Key::Enter);
    key(&mut app, Key::Char('d'));
    assert_eq!(game_moves(&app), 1);

    key(&mut app, Key::Esc);
    assert_eq!(app.screen, Screen::Title);

    key(&mut app, Key::Enter);
    assert_eq!(app.screen, Screen::Playing);
    assert_eq!(game_moves(&app), 1);
}

#[test]
fn new_game_key_starts_fresh() {
    let mut app = App::default();
    key(&mut app, Key::Enter);
    key(&mut app, Key::Char('d'));
    key(&mut app, Key::Char('n'));
    assert_eq!(game_moves(&app), 0);
}

#[test]
fn scores_screen_opens_and_closes() {
    let mut app = App::default();
    key(&mut app, Key::Down);
    key(&mut app, Key::Enter);
    assert_eq!(app.screen, Screen::Scores);
    key(&mut app, Key::Esc);
    assert_eq!(app.screen, Screen::Title);
}

#[test]
fn pressing_t_changes_the_theme() {
    let mut app = App::default();
    let start = app.theme.name;
    key(&mut app, Key::Char('t'));
    assert_ne!(app.theme.name, start);
}

#[test]
fn finishing_a_game_records_it_to_the_store() {
    let mut app = App::default();
    let mut game = SolitaireGame::new();
    game.state = SolitaireState::default();
    for (i, suit) in Suit::ALL.into_iter().enumerate() {
        game.state.foundations[i] = full_foundation(suit);
    }
    let king = game.state.foundations[3].pop().unwrap();
    game.state.waste = vec![king];
    app.game = Some(Box::new(game));
    app.screen = Screen::Playing;

    key(&mut app, Key::Char('f'));

    assert_eq!(app.store.history.len(), 1);
    assert!(app.store.history[0].won);
    assert_eq!(app.store.history[0].game, "Solitaire");
}

#[test]
fn two_number_keys_move_a_column_run() {
    let mut game = SolitaireGame::new();
    game.state = SolitaireState::default();
    game.state.tableau[0].cards = vec![card(Rank::Six, Suit::Hearts)];
    game.state.tableau[1].cards = vec![card(Rank::Seven, Suit::Spades)];

    game.handle_key(Key::Char('1'));
    game.handle_key(Key::Char('2'));

    assert!(game.state.tableau[0].cards.is_empty());
    assert_eq!(game.state.tableau[1].cards.len(), 2);
}

#[test]
fn f_auto_sends_an_ace_to_a_foundation() {
    let mut game = SolitaireGame::new();
    game.state = SolitaireState::default();
    game.state.waste = vec![card(Rank::Ace, Suit::Spades)];

    game.handle_key(Key::Char('f'));

    assert!(game.state.waste.is_empty());
    assert_eq!(
        game.state.foundations[0],
        vec![card(Rank::Ace, Suit::Spades)]
    );
}

#[test]
fn placing_the_last_card_wins_and_scores() {
    let mut game = SolitaireGame::new();
    game.state = SolitaireState::default();
    for (i, suit) in Suit::ALL.into_iter().enumerate() {
        game.state.foundations[i] = full_foundation(suit);
    }
    let king = game.state.foundations[3].pop().unwrap();
    game.state.waste = vec![king];

    game.handle_key(Key::Char('f'));

    assert_eq!(game.outcome(), Outcome::Won);
    assert_eq!(game.score(), 52 * 10 + 100);
}

#[test]
fn a_move_into_a_dead_end_is_lost() {
    let mut game = SolitaireGame::new();
    game.state = SolitaireState::default();
    game.state.tableau[0].cards = vec![card(Rank::Two, Suit::Hearts)];
    game.state.tableau[1].cards = vec![card(Rank::Three, Suit::Spades)];

    game.handle_key(Key::Char('1'));
    game.handle_key(Key::Char('2'));

    assert_eq!(game.outcome(), Outcome::Lost);
}

#[test]
fn an_in_progress_game_survives_a_restart() {
    let backend = MemoryBackend::default();

    let mut app = App::new(Box::new(backend.clone()));
    key(&mut app, Key::Enter);
    key(&mut app, Key::Char('d'));
    assert_eq!(game_moves(&app), 1);
    drop(app);

    let restarted = App::new(Box::new(backend.clone()));
    assert!(restarted.game.is_some());
    assert_eq!(restarted.game.as_ref().unwrap().moves(), 1);
}

#[test]
fn a_finished_game_is_not_restored_as_active() {
    let backend = MemoryBackend::default();
    let mut app = App::new(Box::new(backend.clone()));
    key(&mut app, Key::Enter);

    let mut game = SolitaireGame::new();
    game.state = SolitaireState::default();
    for (i, suit) in Suit::ALL.into_iter().enumerate() {
        game.state.foundations[i] = full_foundation(suit);
    }
    let king = game.state.foundations[3].pop().unwrap();
    game.state.waste = vec![king];
    app.game = Some(Box::new(game));
    key(&mut app, Key::Char('f'));
    drop(app);

    let restarted = App::new(Box::new(backend));
    assert!(restarted.game.is_none());
    assert_eq!(restarted.store.history.len(), 1);
}

#[test]
fn persistence_round_trips_the_store() {
    let backend = MemoryBackend::default();
    let mut store = Store {
        theme_index: 3,
        ..Default::default()
    };
    store.record(GameRecord {
        game: "Solitaire".to_string(),
        won: true,
        score: 620,
        moves: 40,
    });

    persistence::save(&backend, &store);
    let loaded = persistence::load(&backend);

    assert_eq!(loaded.theme_index, 3);
    assert_eq!(loaded.history.len(), 1);
    assert_eq!(loaded.history[0].score, 620);
}
