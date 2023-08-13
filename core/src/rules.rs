use crate::state::State;

/// # Conway's Game Of Life Rules
///
/// * **Rule 1:** Any live cell with fewer than two live neighbours
/// dies, as if caused by underpopulation.
///
/// * **Rule 2:** Any live cell with two or three live neighbours
/// lives on to the next generation.
///
/// * **Rule 3:** Any live cell with more than three live
/// neighbours dies, as if by overpopulation.
///
/// * **Rule 4:** Any dead cell with exactly three live neighbours
/// becomes a live cell, as if by reproduction.
///
/// All other cells remain in the same state.
///
/// ## Panics
///
/// Panics in debug when `neighbors` more is greater than 8.
#[inline]
pub fn conway(state: State, neighbors: u8) -> State {
    debug_assert!(neighbors <= 8);
    match (state, neighbors) {
        (State::Alive, x) if x < 2 => State::Dead,
        (State::Alive, 2) | (State::Alive, 3) => State::Alive,
        (State::Alive, x) if x > 3 => State::Dead,
        (State::Dead, 3) => State::Alive,
        (otherwise, _) => otherwise,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_1() {
        assert_eq!(conway(State::Alive, 0), State::Dead);
        assert_eq!(conway(State::Alive, 1), State::Dead);
    }

    #[test]
    fn rule_2() {
        assert_eq!(conway(State::Alive, 2), State::Alive);
        assert_eq!(conway(State::Alive, 3), State::Alive);
    }

    #[test]
    fn rule_3() {
        for n in 4..=8 {
            assert_eq!(conway(State::Alive, n), State::Dead);
        }
    }

    #[test]
    fn rule_4() {
        assert_eq!(conway(State::Dead, 3), State::Alive);
    }

    #[test]
    fn other() {
        for n in 0..=8 {
            if n != 3 {
                assert_eq!(conway(State::Dead, n), State::Dead);
            }
        }
    }

    #[test]
    #[should_panic]
    fn neighbors_more_than_8() {
        conway(State::Dead, 9);
        conway(State::Alive, 9);
    }
}
