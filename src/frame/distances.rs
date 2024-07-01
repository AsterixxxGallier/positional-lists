use std::ops::{Range, RangeBounds};
use std::slice;
use num_traits::zero;
use crate::Position;

pub(crate) const DISTANCES_DEPTH: usize = 3;
// Must be a power of two.
pub(crate) const DISTANCES_CAPACITY: usize = 1 << (DISTANCES_DEPTH - 1);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Distances<P: Position> {
    // TODO document contents of this array
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

    /// The distance between index and index + 1.
    pub(crate) fn distance(&self, index: usize) -> P {
        let mut distance = self.distances[index];
        for degree in 0..index.trailing_ones() {
            distance -= self.distances[index - (1 << degree)];
        }
        distance
    }

    pub(crate) fn remove(&mut self, index: usize) {
        self.splice(index..=index, 0)
    }

    /// Replaces the distances in `range` with `replace_with` zeroes.
    pub(crate) fn splice<R: RangeBounds<usize>>(&mut self, range: R, replace_with: usize) {
        // TODO more efficient implementation

        let mut simple = self.simple();

        let Range { start, end: splice_end } = slice::range(range, ..simple.len());
        let replacement_end = start + replace_with;
        let trailing_zeroes_start = simple.len() - (splice_end - start - replace_with);

        simple.copy_within(splice_end.., replacement_end);
        simple[start..replacement_end].fill(zero());
        simple[trailing_zeroes_start..].fill(zero());

        *self = Self::from_simple(simple);

        /*
        (old comment, maybe useful for efficient implementation of this function)

        000 001 010 011
        shift by 1
        (zero) (000) (001 - 000) (001 + 010)

        000 001 010 011
        shift by 2
        (zero) (zero) (000) (001)

        000 001 010 011
        shift by 3
        (zero) (zero) (zero) (000)

        000 001 010 011
        shift by 4
        (zero) (zero) (zero) (zero)

        000 001 010 011
        shift by -1
        (001 - 000) (010) (011 - (001 + 010)) (zero)

         a   b   c   d   e   f   g   h
        000 001 010 011 100 101 110 111
         a  a+b  c       e  e+f  g
                  a+b+c+d     a+b+c+d+e+f+g+h
        shift by 1
         a'  b'  c'  d'  e'  f'  g'  h'
         0   a   b   c   d   e   f   g
        000'001'010'011'100'101'110'111'
             a  a+b  c       e  e+f  g
                      a+b+c+d

        by > 0: work leftwards, because new values don't depend on old values to their right
         */
    }

    fn simple(&self) -> [P; DISTANCES_CAPACITY] {
        let mut simple = self.distances;
        for degree in (1..DISTANCES_DEPTH).rev() {
            for index in (((1 << degree) - 1)..DISTANCES_CAPACITY).step_by(1 << degree) {
                simple[index] -= simple[index - (1 << (degree - 1))];
            }
        }
        simple
    }

    fn from_simple(simple: [P; DISTANCES_CAPACITY]) -> Self {
        let mut distances = simple;
        for degree in 1..DISTANCES_DEPTH {
            for index in (((1 << degree) - 1)..DISTANCES_CAPACITY).step_by(1 << degree) {
                distances[index] += distances[index - (1 << (degree - 1))];
            }
        }
        Self { distances }
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::prelude::*;
    use crate::{Distances, DISTANCES_CAPACITY};

    #[test]
    fn test_increase_distance() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut distances = Distances::new();
        let mut simple_distances = [0; DISTANCES_CAPACITY];
        #[allow(clippy::needless_range_loop)]
        for index in 0..DISTANCES_CAPACITY {
            let value = rng.next_u32() as u64;
            distances.increase_distance(index, value);
            simple_distances[index] += value;
        }
        assert_eq!(distances.simple(), simple_distances);
        assert_eq!(Distances::from_simple(distances.simple()), distances);
        let reproduced_simple = (0..DISTANCES_CAPACITY).map(|i| distances.distance(i)).collect_vec();
        assert_eq!(simple_distances.to_vec(), reproduced_simple);
    }

    #[test]
    fn test_splice() {
        const HALF: usize = DISTANCES_CAPACITY >> 1;

        let first_half = Distances::from_simple(
            (0..DISTANCES_CAPACITY).map(|i| {
                if (..HALF).contains(&i) { 1 } else { 0 }
            }).collect_vec().try_into().unwrap()
        );

        let second_half = Distances::from_simple(
            (0..DISTANCES_CAPACITY).map(|i| {
                if (..HALF).contains(&i) { 0 } else { 1 }
            }).collect_vec().try_into().unwrap()
        );

        let mut distances = Distances::from_simple([1u32; DISTANCES_CAPACITY]);
        distances.splice(.., 0);
        assert_eq!(distances, Distances::new());

        let mut distances = Distances::from_simple([1u32; DISTANCES_CAPACITY]);
        distances.splice(.., DISTANCES_CAPACITY);
        assert_eq!(distances, Distances::new());

        let mut distances = Distances::from_simple([1u32; DISTANCES_CAPACITY]);
        distances.splice(..HALF, 0);
        assert_eq!(distances, first_half);

        let mut distances = Distances::from_simple([1u32; DISTANCES_CAPACITY]);
        distances.splice(..HALF, HALF);
        assert_eq!(distances, second_half);

        let mut distances = Distances::from_simple([1u32; DISTANCES_CAPACITY]);
        distances.splice(HALF.., 0);
        assert_eq!(distances, first_half);

        let mut distances = Distances::from_simple([1u32; DISTANCES_CAPACITY]);
        distances.splice(HALF.., HALF);
        assert_eq!(distances, first_half);
    }
}
