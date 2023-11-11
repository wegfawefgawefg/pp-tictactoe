use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::game::Position;

// }
pub fn get_random_move(rng: &mut ThreadRng) -> Position {
    [
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 2, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: 1, y: 1 },
        Position { x: 2, y: 1 },
        Position { x: 0, y: 2 },
        Position { x: 1, y: 2 },
        Position { x: 2, y: 2 },
    ]
    .choose(rng)
    .unwrap()
    .clone()
}
