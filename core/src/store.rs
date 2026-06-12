use crate::games::GameId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRecord {
    pub game: String,
    pub won: bool,
    pub score: u32,
    pub moves: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGame {
    pub id: GameId,
    pub data: Vec<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    #[serde(default)]
    pub theme_index: usize,
    #[serde(default)]
    pub history: Vec<GameRecord>,
    #[serde(default)]
    pub active: Option<SavedGame>,
}

impl Store {
    pub fn record(&mut self, record: GameRecord) {
        self.history.push(record);
    }

    pub fn ranked(&self) -> Vec<&GameRecord> {
        let mut ranked: Vec<&GameRecord> = self.history.iter().collect();
        ranked.sort_by(|a, b| b.score.cmp(&a.score).then(a.moves.cmp(&b.moves)));
        ranked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(score: u32, moves: u32) -> GameRecord {
        GameRecord {
            game: "Solitaire".to_string(),
            won: score >= 520,
            score,
            moves,
        }
    }

    #[test]
    fn ranked_orders_by_score_then_fewer_moves() {
        let mut store = Store::default();
        store.record(record(300, 80));
        store.record(record(620, 90));
        store.record(record(620, 40));

        let ranked = store.ranked();

        assert_eq!(ranked[0].moves, 40);
        assert_eq!(ranked[1].moves, 90);
        assert_eq!(ranked[2].score, 300);
    }
}
