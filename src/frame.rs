use std::assert_matches::debug_assert_matches;
use num_traits::zero;
use slotmap::{new_key_type, SlotMap};
use crate::{Element, Position};
use crate::index::{EphemeralIndex, PersistentIndex};

new_key_type! { pub struct FrameKey; }

#[derive(Debug, Eq, PartialEq)]
pub enum Slot<E: Element> {
    Element {
        value: E,
        persistent_index: PersistentIndex,
    },
    Frame(FrameKey),
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
    pub fn new_with_element(element: E, persistent_index: PersistentIndex) -> (Self, usize) {
        (Self {
            distances: vec![],
            slots: vec![Slot::Element { value: element, persistent_index }],
            depth: 0,
        }, 0)
    }

    fn check_invariants(&self) {
        debug_assert!(!self.slots.is_empty());
        debug_assert_eq!(self.slots.len(), self.distances.len() + 1);
        debug_assert_matches!(self.slots.last(), Some(Slot::Element { .. }));
        // check that self.depth has the correct value
        debug_assert!(self.distances.len() < 1 << self.depth);
        debug_assert!(self.distances.len() >= (1 << self.depth) >> 1);
    }

    /// `distance_from_last` must be non-negative.
    pub fn add_element(&mut self, element: E, persistent_index: PersistentIndex, distance_from_last: P) -> usize {
        debug_assert!(distance_from_last > zero());

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

        self.slots.push(Slot::Element { value: element, persistent_index });

        index
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use crate::{Frame, Slot};
    use crate::index::PersistentIndex;

    #[test]
    fn test_add_element() {
        let (mut frame, index) = Frame::<usize, _>::new_with_element((), PersistentIndex::new(0));

        assert_eq!(index, 0);
        assert_eq!(frame.distances, vec![]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 0);

        let index = frame.add_element((), PersistentIndex::new(1), 1);

        assert_eq!(index, 1);
        assert_eq!(frame.distances, vec![1]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 1);

        let index = frame.add_element((), PersistentIndex::new(2), 2);

        assert_eq!(index, 2);
        assert_eq!(frame.distances, vec![1, 3]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(2) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 2);

        let index = frame.add_element((), PersistentIndex::new(3), 3);

        assert_eq!(index, 3);
        assert_eq!(frame.distances, vec![1, 3, 3]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(2) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(3) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 2);

        let index = frame.add_element((), PersistentIndex::new(4), 4);

        assert_eq!(index, 4);
        assert_eq!(frame.distances, vec![1, 3, 3, 10]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(2) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(3) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: (), persistent_index: PersistentIndex::new(4) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 3);
    }
}