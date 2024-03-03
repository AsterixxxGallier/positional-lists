use arrayvec::ArrayVec;
use std::iter::once;
use num_traits::zero;
use crate::{Position, Element, Frame, ElementFrame, Distances, Embedding, FrameKey};
use crate::frame::FRAME_CAPACITY;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct MetaFrame<P: Position> {
    pub(crate) distances: Distances<P>,
    /// `frames.len()` must equal `distances.distances.len() + 1`
    pub(crate) frames: ArrayVec<FrameKey, FRAME_CAPACITY>,
    pub(crate) level: usize,
    pub(crate) embedding: Embedding,
}

impl<P: Position> MetaFrame<P> {
    pub(crate) fn new_with_frame(key: FrameKey, level: usize, embedding: Embedding) -> (Self, usize) {
        (Self {
            distances: Distances::new(),
            frames: ArrayVec::from_iter(once(key)),
            level,
            embedding,
        }, 0)
    }

    pub(crate) fn check_invariants(&self) {
        debug_assert!(!self.frames.is_empty());
        debug_assert_eq!(self.frames.len(), self.distances.distances.len() + 1);
    }

    pub(crate) fn is_full(&self) -> bool {
        self.distances.is_full()
    }

    /// `distance_from_last` must be non-negative.
    pub(crate) fn add_frame(&mut self, key: FrameKey, distance_from_last: P) -> usize {
        debug_assert!(distance_from_last > zero());

        self.check_invariants();

        self.distances.add_distance(distance_from_last);

        self.frames.push(key);

        self.frames.len() - 1
    }

    pub(crate) fn last_frame(&self) -> FrameKey {
        self.check_invariants();

        // !self.frames.is_empty() is an invariant
        *self.frames.last().unwrap()
    }
}

impl<P: Position> Frame<P> for MetaFrame<P> {
    fn distances(&self) -> &Distances<P> {
        &self.distances
    }

    fn level(&self) -> usize {
        self.level
    }

    fn embedding(&self) -> Embedding {
        self.embedding
    }

    fn embed(&mut self, embedding: Embedding) {
        self.embedding = embedding;
    }
}

impl<P: Position, E: Element> Frame<P> for ElementFrame<P, E> {
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