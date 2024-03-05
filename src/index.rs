use crate::FrameKey;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct EphemeralIndex {
    pub(crate) frame: FrameKey,
    pub(crate) index: usize,
}

impl EphemeralIndex {
    pub(crate) fn new(frame: FrameKey, index: usize) -> Self {
        Self { frame, index }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Index {
    pub(crate) index: usize,
}

impl Index {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}
