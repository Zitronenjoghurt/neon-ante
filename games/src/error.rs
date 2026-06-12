pub type GamesResult<T> = Result<T, GamesError>;

#[derive(Debug, thiserror::Error)]
pub enum GamesError {
    #[error("Invalid action: {0}")]
    InvalidAction(String),
}
