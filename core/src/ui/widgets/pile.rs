use crate::rules::cards::french::Card;
use crate::theme::Theme;
use crate::ui::widgets::card::{CARD_HEIGHT, CARD_WIDTH, CardWidget};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Widget};

const FACE_DOWN_PEEK: u16 = 1;
const FACE_UP_PEEK: u16 = 2;

pub struct PileWidget<'a> {
    cards: &'a [Card],
    theme: &'a Theme,
    face_down: usize,
    outline: Option<Color>,
}

impl<'a> PileWidget<'a> {
    pub fn new(cards: &'a [Card], theme: &'a Theme) -> Self {
        Self {
            cards,
            theme,
            face_down: 0,
            outline: None,
        }
    }

    pub fn face_down(mut self, face_down: usize) -> Self {
        self.face_down = face_down;
        self
    }

    pub fn outline(mut self, outline: Option<Color>) -> Self {
        self.outline = outline;
        self
    }

    fn peek(face_up: bool) -> u16 {
        if face_up {
            FACE_UP_PEEK
        } else {
            FACE_DOWN_PEEK
        }
    }

    pub fn height(len: usize, face_down: usize) -> u16 {
        if len == 0 {
            return CARD_HEIGHT;
        }
        let mut height = CARD_HEIGHT;
        for i in 0..len.saturating_sub(1) {
            height += Self::peek(i >= face_down);
        }
        height
    }
}

impl Widget for PileWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = CARD_WIDTH.min(area.width);
        if width < 3 || area.height < 3 {
            return;
        }

        if self.cards.is_empty() {
            let slot = Rect {
                height: CARD_HEIGHT.min(area.height),
                width,
                ..area
            };
            let border_type = if self.outline.is_some() {
                BorderType::Thick
            } else {
                BorderType::Rounded
            };
            Block::bordered()
                .border_type(border_type)
                .border_style(Style::new().fg(self.outline.unwrap_or(self.theme.fg_dim)))
                .style(Style::new().bg(self.theme.bg))
                .render(slot, buf);
            return;
        }

        let last = self.cards.len() - 1;
        let mut y = area.top();
        for (i, card) in self.cards.iter().enumerate() {
            if y + 3 > area.bottom() {
                break;
            }
            let face_up = i >= self.face_down;
            let rect = Rect {
                x: area.x,
                y,
                width,
                height: CARD_HEIGHT.min(area.bottom() - y),
            };
            let outline = if i == last { self.outline } else { None };
            CardWidget::new(card, self.theme)
                .face_up(face_up)
                .outline(outline)
                .render(rect, buf);

            if i != last {
                y += Self::peek(face_up);
            }
        }
    }
}
