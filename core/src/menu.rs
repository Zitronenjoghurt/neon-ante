use crate::event::Key;
use crate::games::GameId;

pub struct GameEntry {
    pub id: GameId,
    pub title: &'static str,
    pub blurb: &'static str,
    pub available: bool,
}

pub const ENTRIES: [GameEntry; 1] = [GameEntry {
    id: GameId::Solitaire,
    title: "Solitaire",
    blurb: "Build the foundations up, stack the tableau down.",
    available: true,
}];

pub enum MenuSignal {
    Stay,
    Launch(GameId),
    OpenScores,
    CycleTheme(bool),
}

#[derive(Debug, Default)]
pub struct Menu {
    pub focus: usize,
}

impl Menu {
    pub fn scores_row() -> usize {
        ENTRIES.len()
    }

    pub fn theme_row() -> usize {
        ENTRIES.len() + 1
    }

    pub fn rows() -> usize {
        ENTRIES.len() + 2
    }

    pub fn on_scores(&self) -> bool {
        self.focus == Self::scores_row()
    }

    pub fn on_theme(&self) -> bool {
        self.focus == Self::theme_row()
    }

    pub fn focused_game(&self) -> Option<&'static GameEntry> {
        ENTRIES.get(self.focus)
    }

    pub fn handle_key(&mut self, key: Key) -> MenuSignal {
        match key {
            Key::Up | Key::Char('k') => self.focus = self.focus.saturating_sub(1),
            Key::Down | Key::Char('j') => {
                if self.focus + 1 < Self::rows() {
                    self.focus += 1;
                }
            }
            Key::Left | Key::Char('h') if self.on_theme() => return MenuSignal::CycleTheme(false),
            Key::Right | Key::Char('l') if self.on_theme() => return MenuSignal::CycleTheme(true),
            Key::Enter | Key::Char(' ') => {
                if self.on_scores() {
                    return MenuSignal::OpenScores;
                }
                if let Some(game) = self.focused_game()
                    && game.available
                {
                    return MenuSignal::Launch(game.id);
                }
            }
            _ => {}
        }
        MenuSignal::Stay
    }
}
