use num_traits::zero;
use crate::Position;

pub(crate) const DISTANCES_DEPTH: usize = 9;
// Must be a power of two.
pub(crate) const DISTANCES_CAPACITY: usize = 1 << (DISTANCES_DEPTH - 1);

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Distances<P: Position> {
    // TODO document contents of this Vec
    /// May not contain negative values.
    pub(crate) distances: [P; DISTANCES_CAPACITY],
}

impl<P: Position> Distances<P> {
    pub(crate) fn new() -> Self {
        Self { distances: [zero(); DISTANCES_CAPACITY] }
    }

    pub(crate) fn increase_distance(&mut self, index: usize, change: P) {
        for degree in 0..DISTANCES_DEPTH {
            if index >> degree & 1 == 0 {
                let distance_index = index | ((1 << degree) - 1);
                self.distances[distance_index] += change;
            }
        }
    }

    pub(crate) fn position(&self, index: usize) -> P {
        let mut position = zero();

        // TODO the last iteration (degree = DISTANCES_DEPTH) is only useful for index = DISTANCES_
        //  CAPACITY - 1; optimize by reducing DISTANCES_DEPTH and DISTANCES_CAPACITY by one each?
        //  would make the length function significantly less efficient => alternative optimizations
        //  possible?
        for degree in 0..DISTANCES_DEPTH {
            let next_index = (index >> degree | 1) << degree;
            if next_index <= index {
                position += self.distances[next_index - 1];
            }
        }

        position
    }

    pub(crate) fn length(&self) -> P {
        self.distances[DISTANCES_CAPACITY - 1]
    }
}
