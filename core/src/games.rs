pub mod solitaire;

use crate::event::Key;
use crate::theme::Theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Playing,
    Won,
    Lost,
}

impl Outcome {
    pub fn is_over(self) -> bool {
        !matches!(self, Outcome::Playing)
    }
}

pub enum GameSignal {
    Stay,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameId {
    Solitaire,
}

pub trait Game: std::fmt::Debug {
    fn handle_key(&mut self, key: Key) -> GameSignal;
    fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme);
    fn name(&self) -> &'static str;
    fn outcome(&self) -> Outcome;
    fn moves(&self) -> u32;
    fn score(&self) -> u32;
    fn snapshot(&self) -> Option<Vec<u8>>;
}

pub fn create(id: GameId) -> Box<dyn Game> {
    match id {
        GameId::Solitaire => Box::new(solitaire::SolitaireGame::new()),
    }
}

pub fn restore(id: GameId, data: &[u8]) -> Option<Box<dyn Game>> {
    match id {
        GameId::Solitaire => rmp_serde::from_slice::<solitaire::SolitaireGame>(data)
            .ok()
            .map(|game| Box::new(game) as Box<dyn Game>),
    }
}
