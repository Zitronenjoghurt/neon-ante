use std::path::PathBuf;
use std::time::{Duration, Instant};

use neon_ante_core::app::App;
use neon_ante_core::event::{Key, Msg};
use neon_ante_core::persistence::PersistenceBackend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[derive(Debug)]
struct FileBackend {
    path: PathBuf,
}

impl FileBackend {
    fn new() -> Self {
        Self {
            path: PathBuf::from("neon-ante.dat"),
        }
    }
}

impl PersistenceBackend for FileBackend {
    fn load(&self) -> Option<Vec<u8>> {
        std::fs::read(&self.path).ok()
    }

    fn save(&self, data: &[u8]) {
        let tmp = self.path.with_extension("tmp");
        if std::fs::write(&tmp, data).is_ok() {
            let _ = std::fs::rename(&tmp, &self.path);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(Box::new(FileBackend::new()));
    let mut last_tick = Instant::now();

    while !app.should_quit {
        terminal.draw(|frame| app.render(frame))?;
        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let Some(k) = map_key(key.code)
        {
            app.update(Msg::Key(k));
        }
        let delta_ms = last_tick.elapsed().as_millis() as u32;
        last_tick = Instant::now();
        app.update(Msg::Tick { delta_ms });
    }

    ratatui::restore();
    Ok(())
}

fn map_key(code: KeyCode) -> Option<Key> {
    Some(match code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Enter => Key::Enter,
        KeyCode::Esc => Key::Esc,
        _ => return None,
    })
}
