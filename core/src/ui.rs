use crate::app::App;
use ratatui::Frame;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph};

pub fn render(app: &App, frame: &mut Frame) {
    let t = &app.theme;
    let p = Paragraph::new("NEON ANTE — press q to quit")
        .centered()
        .style(Style::new().fg(t.fg).bg(t.bg))
        .block(
            Block::bordered()
                .style(Style::new().bg(t.bg))
                .border_style(Style::new().fg(t.fg_dim)),
        );
    frame.render_widget(p, frame.area());
}
