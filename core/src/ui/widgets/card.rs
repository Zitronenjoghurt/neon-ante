use crate::rules::cards::french::{Card, Suit};
use crate::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Widget};

pub const CARD_WIDTH: u16 = 7;
pub const CARD_HEIGHT: u16 = 5;

pub struct CardWidget<'a> {
    card: &'a Card,
    theme: &'a Theme,
    face_up: bool,
    outline: Option<Color>,
}

impl<'a> CardWidget<'a> {
    pub fn new(card: &'a Card, theme: &'a Theme) -> Self {
        Self {
            card,
            theme,
            face_up: true,
            outline: None,
        }
    }

    pub fn face_up(mut self, face_up: bool) -> Self {
        self.face_up = face_up;
        self
    }

    pub fn outline(mut self, outline: Option<Color>) -> Self {
        self.outline = outline;
        self
    }
}

fn suit_color(suit: Suit, theme: &Theme) -> Color {
    if suit.is_red() {
        theme.suit_red
    } else {
        theme.suit_black
    }
}

impl Widget for CardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 3 || area.height < 3 {
            return;
        }

        let border = self.outline.unwrap_or(self.theme.fg_dim);
        let border_type = if self.outline.is_some() {
            BorderType::Thick
        } else {
            BorderType::Rounded
        };
        let block = Block::bordered()
            .border_type(border_type)
            .border_style(Style::new().fg(border))
            .style(Style::new().bg(self.theme.bg));
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        if !self.face_up {
            let back = Style::new().fg(self.theme.fg_dim).bg(self.theme.bg);
            for y in inner.top()..inner.bottom() {
                buf.set_string(inner.x, y, "▒".repeat(inner.width as usize), back);
            }
            return;
        }

        let color = suit_color(self.card.suit, self.theme);
        let style = Style::new().fg(color).bg(self.theme.bg);

        let blank = " ".repeat(inner.width as usize);
        for y in inner.top()..inner.bottom() {
            buf.set_string(inner.x, y, &blank, style);
        }

        let label = format!("{}{}", self.card.rank.symbol(), self.card.suit.symbol());
        let label_w = label.chars().count() as u16;

        buf.set_string(inner.x, inner.top(), &label, style);

        let cx = inner.x + inner.width / 2;
        let cy = inner.top() + inner.height / 2;
        buf.set_string(
            cx,
            cy,
            self.card.suit.symbol().to_string(),
            style.add_modifier(Modifier::BOLD),
        );

        let rx = inner.right().saturating_sub(label_w);
        buf.set_string(rx, inner.bottom().saturating_sub(1), &label, style);
    }
}
