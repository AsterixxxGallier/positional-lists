use enum_dispatch::enum_dispatch;
use slotmap::new_key_type;
use crate::{Position, MetaFrame, BaseFrame, Distances, EphemeralIndex};

pub(crate) mod meta;
pub(crate) mod base;
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
pub(crate) enum EitherFrame<P: Position> {
    Meta(MetaFrame<P>),
    Base(BaseFrame<P>),
}

impl<P: Position> EitherFrame<P> {
    pub(crate) fn unwrap_meta(&self) -> &MetaFrame<P> {
        match self {
            EitherFrame::Meta(frame) => frame,
            EitherFrame::Base(_) => unreachable!(),
        }
    }

    pub(crate) fn unwrap_meta_mut(&mut self) -> &mut MetaFrame<P> {
        match self {
            EitherFrame::Meta(frame) => frame,
            EitherFrame::Base(_) => unreachable!(),
        }
    }

    pub(crate) fn unwrap_base(&self) -> &BaseFrame<P> {
        match self {
            EitherFrame::Base(frame) => frame,
            EitherFrame::Meta(_) => unreachable!(),
        }
    }

    pub(crate) fn unwrap_base_mut(&mut self) -> &mut BaseFrame<P> {
        match self {
            EitherFrame::Base(frame) => frame,
            EitherFrame::Meta(_) => unreachable!(),
        }
    }
}
