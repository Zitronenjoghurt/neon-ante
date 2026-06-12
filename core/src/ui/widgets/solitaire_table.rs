use crate::games::solitaire::Pile;
use crate::rules::cards::french::Card;
use crate::rules::solitaire::{FOUNDATION_COUNT, SolitaireState, TABLEAU_COLUMNS};
use crate::theme::Theme;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;

use super::card::{CARD_HEIGHT, CARD_WIDTH};
use super::pile::PileWidget;

const GAP: u16 = 2;
const STOCK_SLOT: usize = 0;
const WASTE_SLOT: usize = 1;
const FOUNDATION_SLOTS: [usize; FOUNDATION_COUNT] = [3, 4, 5, 6];

pub struct SolitaireTable<'a> {
    state: &'a SolitaireState,
    theme: &'a Theme,
    selection: Option<Pile>,
}

impl<'a> SolitaireTable<'a> {
    pub fn new(state: &'a SolitaireState, theme: &'a Theme) -> Self {
        Self {
            state,
            theme,
            selection: None,
        }
    }

    pub fn selection(mut self, selection: Option<Pile>) -> Self {
        self.selection = selection;
        self
    }

    fn outline(&self, pile: Pile) -> Option<Color> {
        (self.selection == Some(pile)).then_some(self.theme.selected)
    }

    fn key_style(&self, selected: bool) -> Style {
        if selected {
            Style::new()
                .fg(self.theme.selected)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(self.theme.fg_dim)
        }
    }
}

fn columns(row: Rect) -> [Rect; TABLEAU_COLUMNS] {
    Layout::horizontal([Constraint::Length(CARD_WIDTH); TABLEAU_COLUMNS])
        .spacing(GAP)
        .flex(Flex::Start)
        .areas(row)
}

fn label(buf: &mut Buffer, area: Rect, text: &str, style: Style) {
    if area.height == 0 {
        return;
    }
    let width = text.chars().count() as u16;
    let x = area.x + area.width.saturating_sub(width) / 2;
    buf.set_string(x, area.y, text, style);
}

fn top_card(cards: &[Card]) -> &[Card] {
    if cards.is_empty() {
        &[]
    } else {
        &cards[cards.len() - 1..]
    }
}

impl Widget for SolitaireTable<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content_width =
            CARD_WIDTH * TABLEAU_COLUMNS as u16 + GAP * (TABLEAU_COLUMNS as u16 - 1);
        if area.width < CARD_WIDTH || area.height < CARD_HEIGHT + 4 {
            return;
        }

        let [board] = Layout::horizontal([Constraint::Length(content_width.min(area.width))])
            .flex(Flex::Center)
            .areas(area);

        let [top_keys, top, tableau_keys, tableau] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(CARD_HEIGHT),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(board);

        let top_key_cols = columns(top_keys);
        let top_cols = columns(top);
        let tableau_key_cols = columns(tableau_keys);
        let tableau_cols = columns(tableau);

        label(
            buf,
            top_key_cols[STOCK_SLOT],
            &format!("d {}", self.state.stock.len()),
            self.key_style(false),
        );
        label(
            buf,
            top_key_cols[WASTE_SLOT],
            &format!("w {}", self.state.waste.len()),
            self.key_style(self.selection == Some(Pile::Waste)),
        );

        PileWidget::new(top_card(&self.state.stock), self.theme)
            .face_down(usize::from(!self.state.stock.is_empty()))
            .render(top_cols[STOCK_SLOT], buf);

        PileWidget::new(top_card(&self.state.waste), self.theme)
            .outline(self.outline(Pile::Waste))
            .render(top_cols[WASTE_SLOT], buf);

        for (foundation, &slot) in FOUNDATION_SLOTS.iter().enumerate() {
            PileWidget::new(top_card(&self.state.foundations[foundation]), self.theme)
                .render(top_cols[slot], buf);
        }

        for i in 0..TABLEAU_COLUMNS {
            let selected = self.selection == Some(Pile::Tableau(i));
            label(
                buf,
                tableau_key_cols[i],
                &(i + 1).to_string(),
                self.key_style(selected),
            );
            let column = &self.state.tableau[i];
            PileWidget::new(&column.cards, self.theme)
                .face_down(column.face_down)
                .outline(self.outline(Pile::Tableau(i)))
                .render(tableau_cols[i], buf);
        }
    }
}
