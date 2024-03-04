use arrayvec::ArrayVec;
use num_traits::zero;
use crate::Position;
use super::DISTANCES_CAPACITY;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Distances<P: Position> {
    // TODO document contents of this Vec
    /// May not contain negative values.
    pub(crate) distances: ArrayVec<P, DISTANCES_CAPACITY>,
    /// The least number such that `distances.len() < 1 << depth`.
    pub(crate) depth: usize,
}

impl<P: Position> Distances<P> {
    pub(crate) fn new() -> Self {
        Self { distances: ArrayVec::new(), depth: 0 }
    }

    pub(crate) fn check_invariants(&self) {
        // check that self.depth has the correct value
        debug_assert!(self.distances.len() < 1 << self.depth);
        debug_assert!(self.distances.len() >= (1 << self.depth) >> 1);
    }

    pub(crate) fn is_full(&self) -> bool {
        self.distances.is_full()
    }

    pub(crate) fn add_distance(&mut self, distance_from_last: P) {
        self.check_invariants();

        let mut distance = distance_from_last;
        let index = self.distances.len();
        for degree in 0..index.trailing_ones() {
            distance += self.distances[index - (1 << degree)];
        }

        self.distances.push(distance);

        if self.distances.len().is_power_of_two() {
            self.depth += 1;
        }
    }

    pub(crate) fn position(&self, index: usize) -> P {
        self.check_invariants();

        let mut position = zero();
        let mut current_index = 0;
        for degree in (0..self.depth).rev() {
            let next_index = current_index + (1 << degree);
            if next_index <= index {
                position += self.distances[next_index - 1];
                current_index = next_index;
            }
        }

        position
    }

    pub(crate) fn length(&self) -> P {
        self.position(DISTANCES_CAPACITY)
    }
}
