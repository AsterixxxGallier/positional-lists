use arrayvec::ArrayVec;
use std::iter::once;
use num_traits::zero;
use crate::{Position, Element, Distances, Embedding, PersistentIndex};
use super::FRAME_CAPACITY;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ElementFrame<P: Position, E: Element> {
    pub(crate) distances: Distances<P>,
    /// `elements.len()` must equal `distances.distances.len() + 1`
    pub(crate) elements: ArrayVec<E, FRAME_CAPACITY>,
    /// `persistent_indices.len()` must equal `elements.len()`
    pub(crate) persistent_indices: ArrayVec<PersistentIndex, FRAME_CAPACITY>,
    pub(crate) embedding: Embedding,
}

impl<P: Position, E: Element> ElementFrame<P, E> {
    pub(crate) fn new_with_element(element: E, persistent_index: PersistentIndex, embedding: Embedding) -> (Self, usize) {
        (Self {
            distances: Distances::new(),
            elements: ArrayVec::from_iter(once(element)),
            persistent_indices: ArrayVec::from_iter(once(persistent_index)),
            embedding,
        }, 0)
    }

    pub(crate) fn check_invariants(&self) {
        debug_assert!(!self.elements.is_empty());
        debug_assert_eq!(self.elements.len(), self.distances.distances.len() + 1);
        debug_assert_eq!(self.persistent_indices.len(), self.elements.len());
    }

    pub(crate) fn is_full(&self) -> bool {
        self.distances.is_full()
    }

    /// `distance_from_last` must be non-negative.
    pub(crate) fn add_element(&mut self, element: E, persistent_index: PersistentIndex, distance_from_last: P) -> usize {
        debug_assert!(distance_from_last > zero());

        self.check_invariants();

        self.distances.add_distance(distance_from_last);

        self.elements.push(element);
        self.persistent_indices.push(persistent_index);

        self.elements.len() - 1
    }
}
