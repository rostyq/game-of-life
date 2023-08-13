use std::fmt::Display;

use crate::rules::conway;
use crate::state::State;
use crate::utils::{to_index, to_position};

#[derive(Clone, Debug)]
pub struct World {
    width: u32,
    height: u32,
    pub states: Vec<State>,
}

impl World {
    #[inline]
    pub fn new(width: u32, states: Vec<State>) -> Self {
        let size = states.len() as u32;
        assert!(size % width == 0);

        let height = size / width;
        Self {
            width,
            height,
            states,
        }
    }

    #[inline]
    pub fn empty(width: u32, height: u32) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            states: vec![State::default(); size as usize],
        }
    }

    #[inline]
    pub unsafe fn neighbors_unchecked(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].into_iter() {
            for delta_column in [self.width - 1, 0, 1].into_iter() {
                if delta_row == 0 && delta_row == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;

                count += *self.get(neighbor_row, neighbor_column).unwrap_unchecked();
            }
        }

        count
    }

    #[inline]
    pub fn neighbors(&self, row: u32, column: u32) -> Option<u8> {
        if column < self.width && row < self.height {
            Some(unsafe { self.neighbors_unchecked(row, column) })
        } else {
            None
        }
    }

    #[inline]
    pub fn get(&self, row: u32, column: u32) -> Option<&State> {
        let index = to_index(row, column, self.width);
        self.states.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, row: u32, column: u32) -> Option<&mut State> {
        let index = to_index(row, column, self.width);
        self.states.get_mut(index)
    }

    #[inline]
    pub fn update(&mut self) {
        let copy = self.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let neighbors = unsafe { copy.neighbors_unchecked(row, column) };
                let state = unsafe { self.get_mut(row, column).unwrap_unchecked() };
                *state = conway(*state, neighbors);
            }
        }
    }

    #[inline]
    pub fn put(&mut self, row: u32, column: u32, world: World) {
        let size = world.width as usize;

        for (index, state) in world.into_iter().enumerate() {
            let (_column, _row) = to_position(index, size);

            if let Some(value) = self.get_mut(row + _row, column + _column) {
                *value = state;
            }
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.states.len()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const State {
        self.states.as_ptr()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &State> {
        self.states.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut State> {
        self.states.iter_mut()
    }
}

#[cfg(feature = "rand")]
impl World {
    #[inline]
    pub fn random<R: rand::Rng>(
        rng: &mut R,
        prob: f64,
        width: u32,
        height: u32,
    ) -> Result<Self, rand::distributions::BernoulliError> {
        use rand::prelude::Distribution;

        let size = (width * height) as usize;
        let mut states = Vec::with_capacity(size);
        let dist = rand::distributions::Bernoulli::new(prob)?;

        for _ in 0..size {
            states.push(dist.sample(rng).then_some(State::Alive).unwrap_or_default());
        }

        Ok(Self {
            width,
            height,
            states,
        })
    }
}

impl IntoIterator for World {
    type Item = State;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.states.into_iter()
    }
}

impl Display for World {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.states.chunks(self.width as usize) {
            for state in row {
                write!(f, "{}", state)?
            }
            write!(f, "{}", "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width() {
        let width = 2;
        let height = 1;
        let w = World::empty(width, height);

        assert_eq!(w.width(), width);
    }

    #[test]
    fn height() {
        let width = 2;
        let height = 1;
        let w = World::empty(width, height);

        assert_eq!(w.height(), height);
    }

    #[test]
    fn empty() {
        let width = 2;
        let height = 1;
        let w = World::empty(width, height);

        assert_eq!(w.states.len(), (width * height) as usize);
        for s in w.into_iter() {
            assert_eq!(s, State::default());
        }
    }

    #[test]
    fn get() {
        let width = 2;
        let height = 1;
        let w = World::empty(width, height);

        assert!(w.get(height - 1, width - 1).is_some());
        assert!(w.get(height, width).is_none());
    }

    #[test]
    fn neighbors() {
        use crate::patterns::{A, D};
        #[rustfmt::skip]
        let w = World::new(3, vec![
            D, A, D,
            A, D, A,
            D, A, D
        ]);

        assert_eq!(w.neighbors(0, 0), Some(3));
        assert_eq!(w.neighbors(1, 1), Some(2));
        assert_eq!(w.neighbors(2, 2), Some(3));

        assert_eq!(w.neighbors(1, 0), Some(3));
        assert_eq!(w.neighbors(0, 1), Some(3));
        assert_eq!(w.neighbors(2, 0), Some(4));

        assert_eq!(w.neighbors(0, 2), Some(4));
        assert_eq!(w.neighbors(2, 1), Some(3));
        assert_eq!(w.neighbors(1, 2), Some(3));
    }

    #[test]
    fn update() {}
}
