use std::fmt::Debug;
use slotmap::{new_key_type, SecondaryMap, SlotMap};
use num_traits::zero;
use crate::{EitherFrame, Element, Embedding, Frame, FrameKey, IndexInFrame, Position};

pub mod add_element;
pub mod debug;

#[cfg(test)]
mod tests;

new_key_type! { pub struct PointKey; }

pub struct PointList<P: Position, E: Element> {
    frames: SlotMap<FrameKey, EitherFrame<P>>,
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

        let index_in_frame = self.point_indices.get(key).copied()?;
        let mut frame = &self.frames[index_in_frame.frame];
        position += frame.distances().position(index_in_frame.index);
        while let Embedding::InMetaFrame(index_in_frame) = frame.embedding() {
            frame = &self.frames[index_in_frame.frame];
            position += frame.distances().position(index_in_frame.index);
        }
        Some(position)
    }
}
