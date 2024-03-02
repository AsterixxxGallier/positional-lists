use std::assert_matches::debug_assert_matches;
use num_traits::zero;
use crate::{Position, Element};

pub type FrameIndex = usize;

#[derive(Debug)]
pub enum Slot<E: Element> {
    Element(E),
    Frame(FrameIndex),
    Empty,
}

pub struct Frame<P: Position, E: Element> {
    // TODO document contents of this Vec
    /// May not contain negative values.
    distances: Vec<P>,
    /// May not be an empty Vec. Last slot must contain an element.
    slots: Vec<Slot<E>>,

    /// The least number such that `distances.len() < 1 << depth`.
    depth: usize,
}

impl<P: Position, E: Element> Frame<P, E> {
    pub fn new_with_element(element: E) -> Self {
        Self {
            distances: vec![],
            slots: vec![Slot::Element(element)],
            depth: 0,
        }
    }

    /// `distance_from_last` must be non-negative.
    pub fn add_element(&mut self, element: E, distance_from_last: P) {
        debug_assert!(distance_from_last > zero());

        debug_assert!(!self.slots.is_empty());
        debug_assert_matches!(self.slots.last(), Some(Slot::Element(_)));
        // Check that self.depth has the correct value.
        debug_assert!(self.distances.len() < 1 << self.depth);
        debug_assert!(self.distances.len() >= (1 << self.depth) >> 1);

        let mut distance = distance_from_last;
        let index = self.distances.len();
        for degree in 0..index.trailing_ones() {
            distance += self.distances[index - (1 << degree)];
        }

        self.distances.push(distance);

        if self.distances.len().is_power_of_two() {
            self.depth += 1;
        }

        self.slots.push(Slot::Element(element));
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use crate::{Frame, Slot};

    #[test]
    fn test_add_element() {
        let mut frame: Frame<usize, ()> = Frame::new_with_element(());

        assert_eq!(frame.distances, vec![]);
        assert_eq!(frame.slots.len(), 1);
        frame.slots.iter().for_each(|slot| assert_matches!(slot, &Slot::Element(())));
        assert_eq!(frame.depth, 0);

        frame.add_element((), 1);

        assert_eq!(frame.distances, vec![1]);
        assert_eq!(frame.slots.len(), 2);
        frame.slots.iter().for_each(|slot| assert_matches!(slot, &Slot::Element(())));
        assert_eq!(frame.depth, 1);

        frame.add_element((), 2);

        assert_eq!(frame.distances, vec![1, 3]);
        assert_eq!(frame.slots.len(), 3);
        frame.slots.iter().for_each(|slot| assert_matches!(slot, &Slot::Element(())));
        assert_eq!(frame.depth, 2);

        frame.add_element((), 3);

        assert_eq!(frame.distances, vec![1, 3, 3]);
        assert_eq!(frame.slots.len(), 4);
        frame.slots.iter().for_each(|slot| assert_matches!(slot, &Slot::Element(())));
        assert_eq!(frame.depth, 2);

        frame.add_element((), 4);

        assert_eq!(frame.distances, vec![1, 3, 3, 10]);
        assert_eq!(frame.slots.len(), 5);
        frame.slots.iter().for_each(|slot| assert_matches!(slot, &Slot::Element(())));
        assert_eq!(frame.depth, 3);
    }
}