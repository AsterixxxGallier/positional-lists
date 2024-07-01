use std::fmt::Debug;
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use num_traits::zero;
use crate::{EitherFrame, Element, Embedding, Frame, FrameKey, IndexInFrame, Position};

pub mod add_element;
pub mod remove_element;
pub mod debug;

#[cfg(test)]
mod tests;

new_key_type! { pub struct PointKey; }

// TODO remove elements list from PointList? (such that elements are stored outside the PointList)
#[derive(Clone)]
pub struct PointList<P: Position, E: Element> {
    frames: SlotMap<FrameKey, EitherFrame<P>>,
    // TODO put root, start and end all in the same Option (that is None for empty PointLists)?
    root: Option<FrameKey>,
    start: P,
    end: P,
    len: usize,
    point_indices: SlotMap<PointKey, IndexInFrame>,
    elements: SecondaryMap<PointKey, E>,
}

impl<P: Position, E: Element> Default for PointList<P, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Position, E: Element> PointList<P, E> {
    pub fn new() -> Self {
        Self {
            frames: SlotMap::with_key(),
            root: None,
            start: zero(),
            end: zero(),
            len: 0,
            point_indices: SlotMap::with_key(),
            elements: SecondaryMap::new(),
        }
    }

    /// The number of elements stored in this list.
    ///
    /// Not to be confused with [`Self::length`].
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn start(&self) -> P {
        self.start
    }

    pub fn end(&self) -> P {
        self.end
    }

    /// The distance between this list's first and last elements.
    /// Zero for empty lists.
    ///
    /// Not to be confused with [`Self::len`].
    pub fn length(&self) -> P {
        self.end - self.start
    }

    // TODO store first_key and last_key in fields for more performance?

    pub fn first_key(&self) -> Option<PointKey> {
        self.root.map(|root| self.first_key_of(root))
    }

    pub fn last_key(&self) -> Option<PointKey> {
        self.root.map(|root| self.last_key_of(root))
    }

    fn first_key_of(&self, frame_key: FrameKey) -> PointKey {
        match &self.frames[frame_key] {
            EitherFrame::Meta(frame) => self.first_key_of(frame.first_frame()),
            EitherFrame::Base(frame) => frame.first_key()
        }
    }

    fn last_key_of(&self, frame_key: FrameKey) -> PointKey {
        match &self.frames[frame_key] {
            EitherFrame::Meta(frame) => self.last_key_of(frame.last_frame()),
            EitherFrame::Base(frame) => frame.last_key()
        }
    }

    fn length_of(&self, frame_key: FrameKey) -> P {
        match &self.frames[frame_key] {
            EitherFrame::Meta(frame) =>
                frame.distances.length() + self.length_of(frame.last_frame()),
            EitherFrame::Base(frame) =>
                frame.distances.length()
        }
    }

    pub fn element(&self, key: PointKey) -> Option<&E> {
        self.elements.get(key)
    }

    pub fn element_mut(&mut self, key: PointKey) -> Option<&mut E> {
        self.elements.get_mut(key)
    }

    pub fn position(&self, key: PointKey) -> Option<P> {
        let mut position = self.start;

        let index_in_frame = self.point_indices.get(key)?;
        let mut frame = &self.frames[index_in_frame.frame];
        position += frame.distances().position(index_in_frame.index);
        while let Embedding::InMetaFrame(index_in_frame) = frame.embedding() {
            frame = &self.frames[index_in_frame.frame];
            position += frame.distances().position(index_in_frame.index);
        }
        Some(position)
    }

    // WIP

    /*fn replenish_distance(&mut self, key: PointKey, index_in_frame: IndexInFrame, distance: P) {
        if key == self.first_key().unwrap() {
            self.start += distance;
        } else if index_in_frame.index == 0 {

        } else {
            index_in_frame.frame.distances.increase_distance(index_of_removed - 1, distance);
        }
    }*/
}
