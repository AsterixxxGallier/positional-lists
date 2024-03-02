use crate::frame::FrameKey;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EphemeralIndex {
    frame: FrameKey,
    index: usize,
}

impl EphemeralIndex {
    pub fn new(frame: FrameKey, index: usize) -> Self {
        Self { frame, index }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PersistentIndex {
    index: usize,
}

impl PersistentIndex {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}
