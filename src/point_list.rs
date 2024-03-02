use slotmap::SlotMap;
use num_traits::zero;
use crate::{Element, Position};
use crate::frame::{Frame, FrameKey};
use crate::index::{EphemeralIndex, PersistentIndex};

pub struct PointList<P: Position, E: Element> {
    frames: SlotMap<FrameKey, Frame<P, E>>,
    root: Option<FrameKey>,
    start: P,
    len: usize,
    persistent_to_ephemeral: Vec<Option<EphemeralIndex>>,
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
            len: 0,
            persistent_to_ephemeral: vec![],
        }
    }

    fn next_persistent_index(&self) -> PersistentIndex {
        PersistentIndex::new(self.persistent_to_ephemeral.len())
    }

    pub fn add_element(&mut self, element: E, distance_from_last: P) -> PersistentIndex {
        self.len += 1;
        if let Some(root_key) = self.root {
            // Distances of zero are not allowed.
            assert!(distance_from_last > zero());

            let persistent_index = self.next_persistent_index();
            let frame = &mut self.frames[root_key];
            let index = frame.add_element(element, persistent_index, distance_from_last);
            let ephemeral_index = EphemeralIndex::new(root_key, index);
            self.persistent_to_ephemeral.push(Some(ephemeral_index));
            persistent_index
        } else {
            self.start = distance_from_last;

            let persistent_index = self.next_persistent_index();
            let (frame, index) = Frame::new_with_element(element, persistent_index);
            let root_key = self.frames.insert(frame);
            self.root = Some(root_key);
            let ephemeral_index = EphemeralIndex::new(root_key, index);
            self.persistent_to_ephemeral.push(Some(ephemeral_index));
            persistent_index
        }
    }
}
