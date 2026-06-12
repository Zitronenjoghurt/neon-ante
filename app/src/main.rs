use std::time::{Duration, Instant};

use neon_ante_core::app::App;
use neon_ante_core::event::{Key, Msg};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
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
