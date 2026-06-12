use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use neon_ante_core::app::App;
use neon_ante_core::event::{Key, Msg};
use neon_ante_core::persistence::PersistenceBackend;
use ratzilla::backend::webgl2::{FontAtlasConfig, WebGl2BackendOptions};
use ratzilla::event::KeyCode;
use ratzilla::ratatui::style::Color;
use ratzilla::ratatui::Terminal;
use ratzilla::{WebGl2Backend, WebRenderer};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    let backend = WebGl2Backend::new_with_options(
        WebGl2BackendOptions::new()
            .disable_auto_css_resize()
            .canvas_padding_color(Color::Black)
            .font_atlas_config(FontAtlasConfig::dynamic(
                &["DejaVu Sans Mono", "monospace"],
                16.0,
            )),
    )?;
    let mut terminal = Terminal::new(backend)?;
    let app = Rc::new(RefCell::new(App::new(Box::new(LocalStorageBackend))));

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

const STORAGE_KEY: &str = "neon-ante";

#[derive(Debug)]
struct LocalStorageBackend;

impl LocalStorageBackend {
    fn storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok()?
    }
}

impl PersistenceBackend for LocalStorageBackend {
    fn load(&self) -> Option<Vec<u8>> {
        let raw = Self::storage()?.get_item(STORAGE_KEY).ok().flatten()?;
        BASE64.decode(raw).ok()
    }

    fn save(&self, data: &[u8]) {
        if let Some(storage) = Self::storage() {
            let _ = storage.set_item(STORAGE_KEY, &BASE64.encode(data));
        }
    }
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
