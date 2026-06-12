use rand::SeedableRng;
use rand::rngs::SmallRng;

pub type GameRng = SmallRng;

pub fn from_seed(seed: u64) -> GameRng {
    SmallRng::seed_from_u64(seed)
}
