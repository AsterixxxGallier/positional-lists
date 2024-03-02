use std::assert_matches::debug_assert_matches;
use num_traits::zero;
use slotmap::new_key_type;
use crate::{Element, Position, PersistentIndex};
use crate::index::EphemeralIndex;

new_key_type! { pub(crate) struct FrameKey; }

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Slot<E: Element> {
    Element {
        value: E,
        persistent_index: PersistentIndex,
    },
    Frame(FrameKey),
    Empty,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Embedding {
    InOuterFrame(EphemeralIndex),
    InList,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Frame<P: Position, E: Element> {
    // TODO document contents of this Vec
    /// May not contain negative values.
    pub(crate) distances: Vec<P>,
    /// May not be an empty Vec. Last slot must contain an element.
    pub(crate) slots: Vec<Slot<E>>,
    /// The least number such that `distances.len() < 1 << depth`.
    pub(crate) depth: usize,
    pub(crate) embedding: Embedding,
}

impl<P: Position, E: Element> Frame<P, E> {
    pub(crate) fn new_with_element(element: E, persistent_index: PersistentIndex, embedding: Embedding) -> (Self, usize) {
        (Self {
            distances: vec![],
            slots: vec![Slot::Element { value: element, persistent_index }],
            depth: 0,
            embedding,
        }, 0)
    }

    pub(crate) fn check_invariants(&self) {
        debug_assert!(!self.slots.is_empty());
        debug_assert_eq!(self.slots.len(), self.distances.len() + 1);
        debug_assert_matches!(self.slots.last(), Some(Slot::Element { .. }));
        // check that self.depth has the correct value
        debug_assert!(self.distances.len() < 1 << self.depth);
        debug_assert!(self.distances.len() >= (1 << self.depth) >> 1);
    }

    /// `distance_from_last` must be non-negative.
    pub(crate) fn add_element(&mut self, element: E, persistent_index: PersistentIndex, distance_from_last: P) -> usize {
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

        self.slots.len() - 1
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
}

#[cfg(test)]
mod tests {
    use crate::{Frame, Slot, PersistentIndex, Embedding};

    #[test]
    fn test_add_element() {
        let (mut frame, index) = Frame::<usize, char>::new_with_element('a', PersistentIndex::new(0), Embedding::InList);

        assert_eq!(index, 0);
        assert_eq!(frame.distances, vec![]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'a', persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 0);

        let index = frame.add_element('b', PersistentIndex::new(1), 1);

        assert_eq!(index, 1);
        assert_eq!(frame.distances, vec![1]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'a', persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'b', persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 1);

        let index = frame.add_element('c', PersistentIndex::new(2), 2);

        assert_eq!(index, 2);
        assert_eq!(frame.distances, vec![1, 3]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'a', persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'b', persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'c', persistent_index: PersistentIndex::new(2) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 2);

        let index = frame.add_element('d', PersistentIndex::new(3), 3);

        assert_eq!(index, 3);
        assert_eq!(frame.distances, vec![1, 3, 3]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'a', persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'b', persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'c', persistent_index: PersistentIndex::new(2) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'd', persistent_index: PersistentIndex::new(3) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 2);

        let index = frame.add_element('e', PersistentIndex::new(4), 4);

        assert_eq!(index, 4);
        assert_eq!(frame.distances, vec![1, 3, 3, 10]);
        let mut iter = frame.slots.iter();
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'a', persistent_index: PersistentIndex::new(0) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'b', persistent_index: PersistentIndex::new(1) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'c', persistent_index: PersistentIndex::new(2) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'd', persistent_index: PersistentIndex::new(3) }));
        assert_eq!(iter.next(), Some(&Slot::Element { value: 'e', persistent_index: PersistentIndex::new(4) }));
        assert_eq!(iter.next(), None);
        assert_eq!(frame.depth, 3);
    }
}