use crate::error::GamesResult;

pub trait Engine {
    type State;
    type Action;
    type Event;

    fn apply(&self, state: &mut Self::State, action: Self::Action)
    -> GamesResult<Vec<Self::Event>>;

    fn is_over(&self, state: &Self::State) -> bool;
}
