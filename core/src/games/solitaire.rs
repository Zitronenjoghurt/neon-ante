use crate::event::Key;
use crate::games::{Game, GameSignal, Outcome};
use crate::rules::engine::Engine;
use crate::rules::rng;
use crate::rules::solitaire::{Solitaire, SolitaireAction, SolitaireState, Source, Target};
use crate::theme::Theme;
use crate::ui::overlay;
use crate::ui::widgets::SolitaireTable;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pile {
    Waste,
    Tableau(usize),
}

const HINTS: [(&str, &str); 8] = [
    ("1-7", "column"),
    ("w", "waste"),
    ("f", "foundation"),
    ("d", "draw"),
    ("u", "undo"),
    ("n", "new"),
    ("t", "theme"),
    ("esc", "back"),
];

#[derive(Debug, Serialize, Deserialize)]
pub struct SolitaireGame {
    pub state: SolitaireState,
    pub selection: Option<Pile>,
    pub moves: u32,
    pub outcome: Outcome,
    #[serde(skip)]
    history: Vec<SolitaireState>,
}

impl Default for SolitaireGame {
    fn default() -> Self {
        Self::new()
    }
}

impl SolitaireGame {
    pub fn new() -> Self {
        let mut rng = rng::from_seed(seed());
        Self {
            state: SolitaireState::deal(&mut rng),
            selection: None,
            moves: 0,
            outcome: Outcome::Playing,
            history: Vec::new(),
        }
    }

    fn pick(&mut self, pile: Pile) {
        match self.selection {
            None => {
                if self.can_source(pile) {
                    self.selection = Some(pile);
                }
            }
            Some(selected) if selected == pile => self.selection = None,
            Some(selected) => {
                if self.try_move(selected, pile) {
                    self.selection = None;
                } else if self.can_source(pile) {
                    self.selection = Some(pile);
                }
            }
        }
    }

    fn can_source(&self, pile: Pile) -> bool {
        match pile {
            Pile::Waste => !self.state.waste.is_empty(),
            Pile::Tableau(i) => !self.state.tableau[i].cards.is_empty(),
        }
    }

    fn try_move(&mut self, source: Pile, dest: Pile) -> bool {
        let action = match (source, dest) {
            (Pile::Waste, Pile::Tableau(i)) => Some((Source::Waste, Target::Tableau(i))),
            (Pile::Tableau(a), Pile::Tableau(b)) => Some((Source::Tableau(a), Target::Tableau(b))),
            _ => None,
        };
        match action {
            Some((from, to)) => self.apply(SolitaireAction::Move { from, to }),
            None => false,
        }
    }

    fn send_to_foundation(&mut self) {
        match self.selection {
            Some(pile) => {
                let from = match pile {
                    Pile::Waste => Source::Waste,
                    Pile::Tableau(a) => Source::Tableau(a),
                };
                if self.apply(SolitaireAction::Move {
                    from,
                    to: Target::Foundation,
                }) {
                    self.selection = None;
                }
            }
            None => {
                self.apply(SolitaireAction::Collect);
            }
        }
    }

    fn apply(&mut self, action: SolitaireAction) -> bool {
        let snapshot = self.state.clone();
        if Solitaire.apply(&mut self.state, action).is_ok() {
            self.history.push(snapshot);
            if self.history.len() > 256 {
                self.history.remove(0);
            }
            self.moves += 1;
            self.refresh_outcome();
            true
        } else {
            false
        }
    }

    fn undo(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.state = previous;
            self.selection = None;
            self.moves = self.moves.saturating_sub(1);
            self.refresh_outcome();
        }
    }

    fn refresh_outcome(&mut self) {
        self.outcome = if Solitaire.is_over(&self.state) {
            Outcome::Won
        } else if Solitaire.has_moves(&self.state) {
            Outcome::Playing
        } else {
            Outcome::Lost
        };
    }

    fn foundation_cards(&self) -> u32 {
        self.state.foundations.iter().map(Vec::len).sum::<usize>() as u32
    }
}

impl Game for SolitaireGame {
    fn handle_key(&mut self, key: Key) -> GameSignal {
        match key {
            Key::Char(' ') | Key::Char('d') => {
                self.apply(SolitaireAction::Draw);
            }
            Key::Enter | Key::Char('f') => self.send_to_foundation(),
            Key::Char('u') => self.undo(),
            Key::Char('w') | Key::Char('0') => self.pick(Pile::Waste),
            Key::Char(c) if ('1'..='7').contains(&c) => {
                self.pick(Pile::Tableau(c as usize - '1' as usize));
            }
            Key::Esc => {
                return match self.selection.take() {
                    Some(_) => GameSignal::Stay,
                    None => GameSignal::Exit,
                };
            }
            _ => {}
        }
        GameSignal::Stay
    }

    fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        frame.render_widget(Block::new().style(Style::new().bg(theme.bg)), area);

        let [header, board, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(area);

        let header_line = Line::from(vec![
            Span::styled(
                " Solitaire ",
                Style::new().fg(theme.accent).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  moves {}", self.moves),
                Style::new().fg(theme.fg_dim),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(header_line).style(Style::new().bg(theme.bg)),
            header,
        );

        let table = SolitaireTable::new(&self.state, theme).selection(self.selection);
        frame.render_widget(table, board.inner(Margin::new(1, 0)));

        frame.render_widget(
            Paragraph::new(hint_line(theme)).style(Style::new().bg(theme.bg)),
            footer,
        );

        match self.outcome {
            Outcome::Won => overlay(frame, board, theme, "YOU WIN"),
            Outcome::Lost => overlay(frame, board, theme, "NO MOVES LEFT"),
            Outcome::Playing => {}
        }
    }

    fn name(&self) -> &'static str {
        "Solitaire"
    }

    fn outcome(&self) -> Outcome {
        self.outcome
    }

    fn moves(&self) -> u32 {
        self.moves
    }

    fn score(&self) -> u32 {
        let win_bonus = if self.outcome == Outcome::Won { 100 } else { 0 };
        self.foundation_cards() * 10 + win_bonus
    }

    fn snapshot(&self) -> Option<Vec<u8>> {
        rmp_serde::to_vec_named(self).ok()
    }
}

fn hint_line(theme: &Theme) -> Line<'static> {
    let mut spans = Vec::with_capacity(HINTS.len() * 3);
    spans.push(Span::raw(" "));
    for (i, (key, label)) in HINTS.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("   ", Style::new().fg(theme.fg_dim)));
        }
        spans.push(Span::styled(
            *key,
            Style::new().fg(theme.accent).add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!(" {label}"),
            Style::new().fg(theme.fg_dim),
        ));
    }
    Line::from(spans)
}

fn seed() -> u64 {
    web_time::SystemTime::now()
        .duration_since(web_time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}
