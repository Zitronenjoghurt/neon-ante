use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use neon_ante_core::app::App;
use neon_ante_core::event::{Key, Msg};
use ratzilla::backend::webgl2::WebGl2BackendOptions;
use ratzilla::event::KeyCode;
use ratzilla::ratatui::Terminal;
use ratzilla::ratatui::style::Color;
use ratzilla::{WebGl2Backend, WebRenderer};

fn main() -> io::Result<()> {
    let backend = WebGl2Backend::new_with_options(
        WebGl2BackendOptions::new()
            .disable_auto_css_resize()
            .canvas_padding_color(Color::Black),
    )?;
    let mut terminal = Terminal::new(backend)?;
    let app = Rc::new(RefCell::new(App::default()));

    terminal.on_key_event({
        let app = app.clone();
        move |event| {
            if let Some(k) = map_key(event.code) {
                app.borrow_mut().update(Msg::Key(k));
            }
        }
    })?;

    let mut last_tick = web_time::Instant::now();
    terminal.draw_web(move |frame| {
        let delta_ms = last_tick.elapsed().as_millis() as u32;
        last_tick = web_time::Instant::now();
        app.borrow_mut().update(Msg::Tick { delta_ms });
        app.borrow().render(frame);
    });

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
