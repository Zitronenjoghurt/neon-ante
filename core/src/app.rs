use crate::event::{Key, Msg};
use crate::games::{self, Game, GameId, GameSignal};
use crate::menu::{Menu, MenuSignal};
use crate::persistence::{self, NullBackend, PersistenceBackend};
use crate::store::{GameRecord, SavedGame, Store};
use crate::theme::Theme;
use ratatui::Frame;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Title,
    Playing,
    Scores,
}

#[derive(Debug)]
pub struct App {
    pub theme: Theme,
    pub should_quit: bool,
    pub screen: Screen,
    pub menu: Menu,
    pub game: Option<Box<dyn Game>>,
    pub store: Store,
    active: Option<GameId>,
    recorded: bool,
    persistence: Box<dyn PersistenceBackend>,
}

impl Default for App {
    fn default() -> Self {
        Self::new(Box::new(NullBackend))
    }
}

impl App {
    pub fn new(persistence: Box<dyn PersistenceBackend>) -> Self {
        let store = persistence::load(persistence.as_ref());
        let theme = Theme::ALL
            .get(store.theme_index)
            .copied()
            .unwrap_or_default();

        let mut game = None;
        let mut active = None;
        if let Some(saved) = &store.active
            && let Some(restored) = games::restore(saved.id, &saved.data)
        {
            game = Some(restored);
            active = Some(saved.id);
        }

        Self {
            theme,
            should_quit: false,
            screen: Screen::Title,
            menu: Menu::default(),
            game,
            store,
            active,
            recorded: false,
            persistence,
        }
    }

    pub fn update(&mut self, msg: Msg) {
        if let Msg::Key(key) = msg {
            self.handle_key(key);
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        crate::ui::render(self, frame);
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Char('q') => {
                self.should_quit = true;
                return;
            }
            Key::Char('t') => {
                self.cycle_theme(true);
                return;
            }
            _ => {}
        }
        match self.screen {
            Screen::Title => self.handle_title(key),
            Screen::Scores => self.handle_scores(key),
            Screen::Playing => self.handle_playing(key),
        }
    }

    fn handle_title(&mut self, key: Key) {
        match self.menu.handle_key(key) {
            MenuSignal::Launch(id) => self.launch(id),
            MenuSignal::OpenScores => self.screen = Screen::Scores,
            MenuSignal::CycleTheme(forward) => self.cycle_theme(forward),
            MenuSignal::Stay => {}
        }
    }

    fn handle_scores(&mut self, key: Key) {
        if matches!(key, Key::Esc) {
            self.screen = Screen::Title;
        }
    }

    fn handle_playing(&mut self, key: Key) {
        if matches!(key, Key::Char('n')) {
            if let Some(id) = self.active {
                self.game = Some(games::create(id));
                self.recorded = false;
            }
        } else if let Some(game) = &mut self.game
            && let GameSignal::Exit = game.handle_key(key)
        {
            self.screen = Screen::Title;
        }
        self.record_finished_game();
        self.persist();
    }

    fn launch(&mut self, id: GameId) {
        if self.active != Some(id) || self.game.is_none() {
            self.game = Some(games::create(id));
            self.active = Some(id);
            self.recorded = false;
        }
        self.screen = Screen::Playing;
        self.persist();
    }

    fn cycle_theme(&mut self, forward: bool) {
        let count = Theme::ALL.len();
        let step = if forward { 1 } else { count - 1 };
        self.store.theme_index = (self.store.theme_index + step) % count;
        self.theme = Theme::ALL[self.store.theme_index];
        self.persist();
    }

    fn record_finished_game(&mut self) {
        if self.recorded {
            return;
        }
        let record = self.game.as_ref().and_then(|game| {
            game.outcome().is_over().then(|| GameRecord {
                game: game.name().to_string(),
                won: game.outcome() == games::Outcome::Won,
                score: game.score(),
                moves: game.moves(),
            })
        });
        if let Some(record) = record {
            self.store.record(record);
            self.recorded = true;
        }
    }

    fn snapshot_active(&self) -> Option<SavedGame> {
        let id = self.active?;
        let game = self.game.as_ref()?;
        if game.outcome().is_over() {
            return None;
        }
        Some(SavedGame {
            id,
            data: game.snapshot()?,
        })
    }

    fn persist(&mut self) {
        self.store.active = self.snapshot_active();
        persistence::save(self.persistence.as_ref(), &self.store);
    }
}
