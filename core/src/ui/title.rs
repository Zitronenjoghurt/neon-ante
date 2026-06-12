use crate::app::App;
use crate::menu::ENTRIES;
use crate::ui::centered_rect;
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

pub fn render(app: &App, frame: &mut Frame) {
    let t = &app.theme;
    let area = frame.area();
    frame.render_widget(Block::new().style(Style::new().bg(t.bg)), area);

    let panel = centered_rect(area, 54, 18);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(t.accent))
        .style(Style::new().bg(t.bg));
    let inner = block.inner(panel);
    frame.render_widget(block, panel);

    let mut lines = vec![
        Line::from(""),
        Line::styled(
            "N E O N   A N T E",
            Style::new().fg(t.accent).add_modifier(Modifier::BOLD),
        )
        .centered(),
        Line::styled("terminal card games", Style::new().fg(t.fg_dim)).centered(),
        Line::from(""),
    ];

    for (i, entry) in ENTRIES.iter().enumerate() {
        let focused = app.menu.focus == i;
        let label = if entry.available {
            entry.title.to_string()
        } else {
            format!("{}  ·  soon", entry.title)
        };
        let style = match (focused, entry.available) {
            (true, _) => Style::new().fg(t.selected).add_modifier(Modifier::BOLD),
            (false, true) => Style::new().fg(t.fg),
            (false, false) => Style::new().fg(t.fg_dim),
        };
        lines.push(row(focused, label, style, t.selected));
    }

    let scores_focused = app.menu.on_scores();
    lines.push(row(
        scores_focused,
        "High Scores".to_string(),
        text_style(scores_focused, t.fg, t.selected),
        t.selected,
    ));

    let theme_focused = app.menu.on_theme();
    lines.push(row(
        theme_focused,
        format!("Theme  ‹ {} ›", t.name),
        text_style(theme_focused, t.fg, t.selected),
        t.selected,
    ));

    lines.push(Line::from(""));
    lines.push(Line::styled(hint(app), Style::new().fg(t.fg_dim)).centered());
    lines.push(Line::from(""));
    lines.push(
        Line::styled(
            "↑/↓ move    ←/→ theme    ⏎ select    q quit",
            Style::new().fg(t.fg_dim),
        )
        .centered(),
    );

    frame.render_widget(Paragraph::new(lines).style(Style::new().bg(t.bg)), inner);
}

fn hint(app: &App) -> &'static str {
    if app.menu.focus < ENTRIES.len() {
        ENTRIES[app.menu.focus].blurb
    } else if app.menu.on_scores() {
        "Past games, ranked by score."
    } else {
        "←/→ to change the theme."
    }
}

fn text_style(focused: bool, fg: Color, selected: Color) -> Style {
    if focused {
        Style::new().fg(selected).add_modifier(Modifier::BOLD)
    } else {
        Style::new().fg(fg)
    }
}

fn row(focused: bool, label: String, style: Style, marker_color: Color) -> Line<'static> {
    let marker = if focused { "▶  " } else { "   " };
    Line::from(vec![
        Span::styled(marker, Style::new().fg(marker_color)),
        Span::styled(label, style),
    ])
    .centered()
}
