use arrayvec::ArrayVec;
use std::iter::once;
use num_traits::zero;
use crate::{Position, Frame, Distances, Embedding, Index};
use super::FRAME_CAPACITY;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BaseFrame<P: Position> {
    pub(crate) distances: Distances<P>,
    /// May not be empty.
    pub(crate) indices: ArrayVec<Index, FRAME_CAPACITY>,
    pub(crate) embedding: Embedding,
}

impl<P: Position> BaseFrame<P> {
    pub(crate) fn new_with_index(index: Index, embedding: Embedding) -> (Self, usize) {
        (Self {
            distances: Distances::new(),
            indices: ArrayVec::from_iter(once(index)),
            embedding,
        }, 0)
    }

    pub(crate) fn check_invariants(&self) {
        debug_assert!(!self.indices.is_empty());
    }

    pub(crate) fn is_full(&self) -> bool {
        self.indices.is_full()
    }

    /// `distance_from_last` must be non-negative.
    pub(crate) fn add_index(&mut self, index: Index, distance_from_last: P) -> usize {
        // Distances of zero are not allowed.
        debug_assert!(distance_from_last > zero());
        self.check_invariants();

        self.distances.increase_distance(self.indices.len() - 1, distance_from_last);
        self.indices.push(index);
        self.indices.len() - 1
    }
}

impl<P: Position> Frame<P> for BaseFrame<P> {
    fn distances(&self) -> &Distances<P> {
        &self.distances
    }

    fn level(&self) -> usize {
        0
    }

    fn embedding(&self) -> Embedding {
        self.embedding
    }

    fn embed(&mut self, embedding: Embedding) {
        self.embedding = embedding;
    }
}
