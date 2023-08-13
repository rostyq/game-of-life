use crate::{state::State, world::World};

pub const A: State = State::Alive;
pub const D: State = State::Dead;

#[rustfmt::skip]
pub const GLIDER: [State; 9] = [
    D, D, A,
    A, D, A,
    D, A, A
];

#[rustfmt::skip]
pub const BLOCK: [State; 4] = [
    A, A,
    A, A,
];

#[inline]
pub fn glider() -> World {
    World::new(3, GLIDER.into_iter().collect())
}

#[inline]
pub fn block() -> World {
    World::new(2, BLOCK.into_iter().collect())
}
