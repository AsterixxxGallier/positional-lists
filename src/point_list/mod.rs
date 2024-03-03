use slotmap::SlotMap;
use num_traits::zero;
use crate::{Position, Element, Frame, EitherFrame, MetaFrame, ElementFrame, Embedding, FrameKey, EphemeralIndex, PersistentIndex};

#[derive(Debug)]
pub struct PointList<P: Position, E: Element> {
    frames: SlotMap<FrameKey, EitherFrame<P, E>>,
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

// for PointList::add_element
enum AddElementStrategy {
    NewElementFrameNecessary {
        frame_with_full_last_frame: FrameKey,
        last_frame: FrameKey,
    },
    ElementCanBeAddedToExistingElementFrame {
        element_frame: FrameKey,
    },
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

    fn element_can_be_added_to(&self, frame_key: FrameKey) -> bool {
        match &self.frames[frame_key] {
            EitherFrame::Meta(frame) =>
                !frame.is_full() || self.element_can_be_added_to(frame.last_frame()),
            EitherFrame::Element(frame) =>
                !frame.is_full(),
        }
    }

    fn ensure_element_can_be_added_to_root(&mut self) {
        let root_key = self.root.unwrap();
        if !self.element_can_be_added_to(root_key) {
            // put frame into a MetaFrame
            let level = self.frames[root_key].level() + 1;
            let (new_root, index) = MetaFrame::new_with_frame(root_key, level, Embedding::InList);
            let new_root_key = self.frames.insert(new_root.into());
            self.frames[root_key].embed(Embedding::InMetaFrame(EphemeralIndex::new(new_root_key, index)));
            self.root = Some(new_root_key);
        }
    }

    fn add_element_frame_with_element(&mut self, element: E) -> FrameKey {
        let (element_frame, index) =
            ElementFrame::new_with_element(element, self.next_persistent_index(), Embedding::InList);
        let element_frame_key = self.frames.insert(element_frame.into());
        let ephemeral_index = EphemeralIndex::new(element_frame_key, index);
        self.persistent_to_ephemeral.push(Some(ephemeral_index));
        element_frame_key
    }

    fn add_element_strategy(&self) -> AddElementStrategy {
        let mut frame_key = self.root.unwrap();
        loop {
            match &self.frames[frame_key] {
                EitherFrame::Meta(meta_frame) => {
                    if self.element_can_be_added_to(meta_frame.last_frame()) {
                        frame_key = meta_frame.last_frame();
                    } else {
                        break AddElementStrategy::NewElementFrameNecessary {
                            frame_with_full_last_frame: frame_key,
                            last_frame: meta_frame.last_frame(),
                        };
                    }
                }
                EitherFrame::Element(_) => {
                    break AddElementStrategy::ElementCanBeAddedToExistingElementFrame {
                        element_frame: frame_key,
                    };
                }
            }
        }
    }

    pub fn add_element(&mut self, element: E, distance_from_last: P) -> PersistentIndex {
        self.len += 1;

        let persistent_index = self.next_persistent_index();

        if self.root.is_some() {
            // Distances of zero are not allowed.
            assert!(distance_from_last > zero());

            self.ensure_element_can_be_added_to_root();

            match self.add_element_strategy() {
                AddElementStrategy::NewElementFrameNecessary {
                    frame_with_full_last_frame,
                    last_frame
                } => {
                    let mut current_frame = self.add_element_frame_with_element(element);

                    // wrap current_frame in MetaFrames until it and last_frame have the same level
                    for level in 1..=self.frames[last_frame].level() {
                        let (meta_frame, index) = MetaFrame::new_with_frame(current_frame, level, Embedding::InList);
                        let meta_frame_key = self.frames.insert(meta_frame.into());
                        self.frames[current_frame].embed(Embedding::InMetaFrame(EphemeralIndex::new(meta_frame_key, index)));
                        current_frame = meta_frame_key;
                    }

                    // add current_frame to frame_with_full_last_frame
                    let index = self.frames[frame_with_full_last_frame].unwrap_meta().frames.len();
                    self.frames[current_frame].embed(Embedding::InMetaFrame(EphemeralIndex::new(frame_with_full_last_frame, index)));
                    self.frames[frame_with_full_last_frame].unwrap_meta_mut().add_frame(current_frame, distance_from_last);
                }
                AddElementStrategy::ElementCanBeAddedToExistingElementFrame {
                    element_frame
                } => {
                    let frame = self.frames[element_frame].unwrap_element_mut();
                    let index = frame.add_element(element, persistent_index, distance_from_last);
                    let ephemeral_index = EphemeralIndex::new(element_frame, index);
                    self.persistent_to_ephemeral.push(Some(ephemeral_index));
                }
            }
        } else {
            self.start = distance_from_last;
            self.root = Some(self.add_element_frame_with_element(element));
        }

        persistent_index
    }

    fn ephemeral(&self, index: PersistentIndex) -> Option<EphemeralIndex> {
        self.persistent_to_ephemeral[index.index]
    }

    fn frame(&self, index: EphemeralIndex) -> &EitherFrame<P, E> {
        &self.frames[index.frame]
    }

    fn element(&self, index: PersistentIndex) -> Option<&E> {
        let ephemeral = self.ephemeral(index)?;
        Some(&self.frame(ephemeral).unwrap_element().elements[ephemeral.index])
    }

    pub fn position(&self, index: PersistentIndex) -> Option<P> {
        let mut position = self.start;
        let index = self.ephemeral(index)?;
        let mut frame = self.frame(index);
        position += frame.distances().position(index.index);
        while let Embedding::InMetaFrame(index) = frame.embedding() {
            frame = self.frame(index);
            position += frame.distances().position(index.index);
        }
        Some(position)
    }
}

#[cfg(test)]
mod tests;
