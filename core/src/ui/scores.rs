use crate::app::App;
use crate::ui::centered_rect;
use ratatui::Frame;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

const MAX_ROWS: usize = 10;

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
            "HIGH SCORES",
            Style::new().fg(t.accent).add_modifier(Modifier::BOLD),
        )
        .centered(),
        Line::from(""),
    ];

    let ranked = app.store.ranked();
    if ranked.is_empty() {
        lines.push(Line::styled("No games played yet.", Style::new().fg(t.fg_dim)).centered());
    } else {
        for (rank, record) in ranked.iter().take(MAX_ROWS).enumerate() {
            let result = if record.won { "WON " } else { "lost" };
            let result_color = if record.won { t.selected } else { t.fg_dim };
            lines.push(Line::from(vec![
                Span::styled(format!("  {:>2}. ", rank + 1), Style::new().fg(t.fg_dim)),
                Span::styled(format!("{:<12}", record.game), Style::new().fg(t.fg)),
                Span::styled(format!("{result}  "), Style::new().fg(result_color)),
                Span::styled(
                    format!("{:>4} pts", record.score),
                    Style::new().fg(t.accent),
                ),
                Span::styled(
                    format!("   {:>3} moves", record.moves),
                    Style::new().fg(t.fg_dim),
                ),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::styled("esc  back", Style::new().fg(t.fg_dim)).centered());

    frame.render_widget(Paragraph::new(lines).style(Style::new().bg(t.bg)), inner);
}
