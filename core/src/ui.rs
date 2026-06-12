pub mod scores;
pub mod title;
pub mod widgets;

use crate::app::{App, Screen};
use crate::theme::Theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};

pub fn render(app: &App, frame: &mut Frame) {
    match app.screen {
        Screen::Title => title::render(app, frame),
        Screen::Scores => scores::render(app, frame),
        Screen::Playing => {
            if let Some(game) = &app.game {
                game.render(frame, frame.area(), &app.theme);
            }
        }
    }
}

pub(crate) fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let [row] = Layout::vertical([Constraint::Length(height.min(area.height))])
        .flex(Flex::Center)
        .areas(area);
    let [cell] = Layout::horizontal([Constraint::Length(width.min(area.width))])
        .flex(Flex::Center)
        .areas(row);
    cell
}

pub(crate) fn overlay(frame: &mut Frame, area: Rect, theme: &Theme, title: &str) {
    let panel = centered_rect(area, 34, 7);
    frame.render_widget(Clear, panel);

    let block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::new().fg(theme.selected))
        .style(Style::new().bg(theme.bg));
    let inner = block.inner(panel);
    frame.render_widget(block, panel);

    let lines = vec![
        Line::from(""),
        Line::styled(
            title,
            Style::new().fg(theme.selected).add_modifier(Modifier::BOLD),
        )
        .centered(),
        Line::from(""),
        Line::styled("n new game    ·    esc menu", Style::new().fg(theme.fg_dim)).centered(),
    ];
    frame.render_widget(
        Paragraph::new(lines).style(Style::new().bg(theme.bg)),
        inner,
    );
}
