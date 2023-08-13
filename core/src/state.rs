use std::{ops::{Add, AddAssign}, fmt::Display};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Dead = 0,
    Alive = 1,
}

impl Default for State {
    #[inline]
    fn default() -> Self {
        Self::Dead
    }
}

impl State {
    #[inline]
    pub fn is_alive(self) -> bool {
        match self {
            Self::Alive => true,
            Self::Dead => false,
        }
    }

    #[inline]
    pub fn swap(&mut self) {
        match self {
            Self::Alive => *self = Self::Dead,
            Self::Dead => *self = Self::Alive
        }
    }
}

impl From<State> for bool {
    #[inline]
    fn from(value: State) -> Self {
        value.is_alive()
    }
}

pub enum StateError {
    NonBinaryValue
}

impl TryFrom<u8> for State {
    type Error = StateError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(State::Dead),
            1 => Ok(State::Alive),
            _ => Err(StateError::NonBinaryValue),
        }
    }
}

impl Display for State {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Dead => '▯',
            Self::Alive => '▮',
        })
    }
}

impl Add<State> for u8 {
    type Output = u8;

    #[inline]
    fn add(self, rhs: State) -> Self::Output {
        self + rhs as u8
    }
}

impl Add<State> for u64 {
    type Output = u64;

    #[inline]
    fn add(self, rhs: State) -> Self::Output {
        self + rhs as u64
    }
}

impl AddAssign<State> for u8 {
    #[inline]
    fn add_assign(&mut self, rhs: State) {
        *self += rhs as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_alive() {
        assert!(!State::Dead.is_alive());
        assert!(State::Alive.is_alive());
    }

    #[test]
    fn assign_add() {
        let mut n = 0u8;

        n += State::Dead;
        assert_eq!(n, n);

        n += State::Alive;
        assert_eq!(n, 1);
    }

    #[test]
    fn add() {
        let cells = vec![State::Alive, State::Dead, State::Alive];

        let count = cells.into_iter().fold(0u8, |n, c| n + c);
        assert_eq!(count, 2);
    }
}
