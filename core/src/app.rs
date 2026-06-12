use crate::event::{Key, Msg};
use crate::theme::Theme;
use ratatui::Frame;

#[derive(Debug, Default)]
pub struct App {
    pub theme: Theme,
    pub should_quit: bool,
}

impl App {
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Key(key) => self.handle_key(key),
            Msg::Tick { .. } => {}
        }
    }
}

// Input handling
impl App {
    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Char('q') | Key::Esc => self.should_quit = true,
            _ => {}
        }
    }
}

// Rendering
impl App {
    pub fn render(&self, frame: &mut Frame) {
        crate::ui::render(self, frame);
    }
}
