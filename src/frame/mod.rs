use enum_dispatch::enum_dispatch;
use slotmap::new_key_type;
use crate::{Position, MetaFrame, BaseFrame, Distances, DISTANCES_CAPACITY};

pub(crate) mod meta;
pub(crate) mod base;
pub(crate) mod distances;

#[cfg(test)]
mod tests;

pub(crate) const FRAME_CAPACITY: usize = DISTANCES_CAPACITY + 1;

new_key_type! { pub(crate) struct FrameKey; }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct IndexInFrame {
    pub(crate) frame: FrameKey,
    pub(crate) index: usize,
}

impl IndexInFrame {
    pub(crate) fn new(frame: FrameKey, index: usize) -> Self {
        Self { frame, index }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Embedding {
    InMetaFrame(IndexInFrame),
    InList,
}

#[enum_dispatch]
pub(crate) trait Frame<P: Position> {
    fn len(&self) -> usize;
    fn distances(&self) -> &Distances<P>;
    fn distances_mut(&mut self) -> &mut Distances<P>;
    fn level(&self) -> usize;
    fn embedding(&self) -> Embedding;
    fn embedding_mut(&mut self) -> &mut Embedding;
    fn embed(&mut self, embedding: Embedding);
}

#[enum_dispatch(Frame<P>)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum EitherFrame<P: Position> {
    Meta(MetaFrame<P>),
    Base(BaseFrame<P>),
}

impl<P: Position> EitherFrame<P> {
    pub(crate) fn unwrap_meta_owned(self) -> MetaFrame<P> {
        match self {
            EitherFrame::Meta(frame) => frame,
            EitherFrame::Base(_) => unreachable!(),
        }
    }
    
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

    pub(crate) fn unwrap_base_owned(self) -> BaseFrame<P> {
        match self {
            EitherFrame::Base(frame) => frame,
            EitherFrame::Meta(_) => unreachable!(),
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
