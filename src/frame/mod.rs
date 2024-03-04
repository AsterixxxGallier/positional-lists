use enum_dispatch::enum_dispatch;
use slotmap::new_key_type;
use crate::{Position, Element, MetaFrame, ElementFrame, Distances, EphemeralIndex};

pub(crate) mod meta;
pub(crate) mod element;
pub(crate) mod distances;

#[cfg(test)]
mod tests;

pub(crate) const DISTANCES_DEPTH: usize = 9;
// Must be a power of two.
pub(crate) const DISTANCES_CAPACITY: usize = 1 << (DISTANCES_DEPTH - 1);
pub(crate) const FRAME_CAPACITY: usize = DISTANCES_CAPACITY + 1;

new_key_type! { pub(crate) struct FrameKey; }

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Embedding {
    InMetaFrame(EphemeralIndex),
    InList,
}

#[enum_dispatch]
pub(crate) trait Frame<P: Position> {
    fn distances(&self) -> &Distances<P>;
    fn level(&self) -> usize;
    fn embedding(&self) -> Embedding;
    fn embed(&mut self, embedding: Embedding);
}

#[enum_dispatch(Frame<P>)]
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum EitherFrame<P: Position, E: Element> {
    Meta(MetaFrame<P>),
    Element(ElementFrame<P, E>),
}

impl<P: Position, E: Element> EitherFrame<P, E> {
    pub(crate) fn unwrap_meta(&self) -> &MetaFrame<P> {
        match self {
            EitherFrame::Meta(frame) => frame,
            EitherFrame::Element(_) => unreachable!(),
        }
    }

    pub(crate) fn unwrap_meta_mut(&mut self) -> &mut MetaFrame<P> {
        match self {
            EitherFrame::Meta(frame) => frame,
            EitherFrame::Element(_) => unreachable!(),
        }
    }

    pub(crate) fn unwrap_element(&self) -> &ElementFrame<P, E> {
        match self {
            EitherFrame::Element(frame) => frame,
            EitherFrame::Meta(_) => unreachable!(),
        }
    }

    pub(crate) fn unwrap_element_mut(&mut self) -> &mut ElementFrame<P, E> {
        match self {
            EitherFrame::Element(frame) => frame,
            EitherFrame::Meta(_) => unreachable!(),
        }
    }
}
