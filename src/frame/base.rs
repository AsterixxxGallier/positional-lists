use arrayvec::ArrayVec;
use std::iter::once;
use num_traits::zero;
use crate::{Position, Frame, Distances, Embedding, PointKey, FRAME_CAPACITY};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct BaseFrame<P: Position> {
    pub(crate) distances: Distances<P>,
    /// May not be empty.
    pub(crate) keys: ArrayVec<PointKey, FRAME_CAPACITY>,
    pub(crate) embedding: Embedding,
}

impl<P: Position> BaseFrame<P> {
    pub(crate) fn new_with_key(key: PointKey, embedding: Embedding) -> (Self, usize) {
        (Self {
            distances: Distances::new(),
            keys: ArrayVec::from_iter(once(key)),
            embedding,
        }, 0)
    }

    pub(crate) fn check_invariants(&self) {
        debug_assert!(!self.keys.is_empty());
    }

    pub(crate) fn is_full(&self) -> bool {
        self.keys.is_full()
    }

    /// `distance_from_last` must be positive.
    pub(crate) fn add_key(&mut self, key: PointKey, distance_from_last: P) -> usize {
        // Distances of zero are not allowed.
        debug_assert!(distance_from_last > zero());
        self.check_invariants();

        self.distances.increase_distance(self.keys.len() - 1, distance_from_last);
        self.keys.push(key);
        self.keys.len() - 1
    }

    pub(crate) fn first_key(&self) -> PointKey {
        self.check_invariants();

        // !self.keys.is_empty() is an invariant
        *self.keys.first().unwrap()
    }

    pub(crate) fn last_key(&self) -> PointKey {
        self.check_invariants();

        // !self.keys.is_empty() is an invariant
        *self.keys.last().unwrap()
    }
}

impl<P: Position> Frame<P> for BaseFrame<P> {
    fn len(&self) -> usize {
        self.keys.len()
    }

    fn distances(&self) -> &Distances<P> {
        &self.distances
    }

    fn distances_mut(&mut self) -> &mut Distances<P> {
        &mut self.distances
    }

    fn level(&self) -> usize {
        0
    }

    fn embedding(&self) -> Embedding {
        self.embedding
    }

    fn embedding_mut(&mut self) -> &mut Embedding {
        &mut self.embedding
    }

    fn embed(&mut self, embedding: Embedding) {
        self.embedding = embedding;
    }
}
