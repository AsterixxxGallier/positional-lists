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

    fn element_cannot_be_added_to(&self, frame_key: FrameKey) -> bool {
        match &self.frames[frame_key] {
            EitherFrame::Meta(frame) =>
                frame.is_full() && self.element_cannot_be_added_to(frame.last_frame()),
            EitherFrame::Element(frame) =>
                frame.is_full(),
        }
    }

    pub fn add_element(&mut self, element: E, distance_from_last: P) -> PersistentIndex {
        self.len += 1;

        let persistent_index = self.next_persistent_index();

        // TODO tidy up this code section (self.frames.insert_with_key should help!)

        if let Some(root_key) = self.root {
            // Distances of zero are not allowed.
            assert!(distance_from_last > zero());

            if self.element_cannot_be_added_to(root_key) {
                // put frame into a MetaFrame
                let (new_root, index) = MetaFrame::new_with_frame(root_key, self.frames[root_key].level() + 1, Embedding::InList);
                let new_root_key = self.frames.insert(new_root.into());
                self.frames[root_key].embed(Embedding::InMetaFrame(EphemeralIndex::new(new_root_key, index)));
                self.root = Some(new_root_key);
            }

            enum X {
                NewElementFrameNecessary {
                    frame_with_full_last_frame: FrameKey,
                    last_frame: FrameKey,
                },
                ElementCanBeAddedToExistingElementFrame {
                    element_frame: FrameKey,
                },
            }

            let x = {
                let mut frame_key = self.root.unwrap();

                loop {
                    match &self.frames[frame_key] {
                        EitherFrame::Meta(meta_frame)
                        if self.element_cannot_be_added_to(meta_frame.last_frame()) => {
                            break X::NewElementFrameNecessary {
                                frame_with_full_last_frame: frame_key,
                                last_frame: meta_frame.last_frame(),
                            };
                        }
                        EitherFrame::Meta(meta_frame) => {
                            frame_key = meta_frame.last_frame();
                        }
                        EitherFrame::Element(_) => {
                            break X::ElementCanBeAddedToExistingElementFrame {
                                element_frame: frame_key,
                            };
                        }
                    }
                }
            };

            match x {
                X::NewElementFrameNecessary {
                    frame_with_full_last_frame,
                    last_frame
                } => {
                    let (element_frame, index) = ElementFrame::new_with_element(element, persistent_index, Embedding::InList);
                    let element_frame_key = self.frames.insert(element_frame.into());
                    let ephemeral_index = EphemeralIndex::new(element_frame_key, index);
                    self.persistent_to_ephemeral.push(Some(ephemeral_index));

                    let mut frame_key = element_frame_key;
                    for level in 1..=self.frames[last_frame].level() {
                        // last_frame is a MetaFrame
                        let (meta_frame, index) = MetaFrame::new_with_frame(frame_key, level, Embedding::InList);
                        let meta_frame_key = self.frames.insert(meta_frame.into());
                        self.frames[frame_key].embed(Embedding::InMetaFrame(EphemeralIndex::new(meta_frame_key, index)));
                        frame_key = meta_frame_key;
                    }

                    let index = self.frames[frame_with_full_last_frame].unwrap_meta().frames.len();
                    self.frames[frame_key].embed(Embedding::InMetaFrame(EphemeralIndex::new(frame_with_full_last_frame, index)));
                    let frame = self.frames[frame_with_full_last_frame].unwrap_meta_mut();
                    frame.add_frame(frame_key, distance_from_last);
                }
                X::ElementCanBeAddedToExistingElementFrame {
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

            let (frame, index) = ElementFrame::new_with_element(element, persistent_index, Embedding::InList);
            let root_key = self.frames.insert(frame.into());
            self.root = Some(root_key);
            let ephemeral_index = EphemeralIndex::new(root_key, index);
            self.persistent_to_ephemeral.push(Some(ephemeral_index));
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
mod tests {
    use itertools::Itertools;
    use num_traits::Zero;
    use crate::{EphemeralIndex, PointList};

    #[test]
    fn test_add_element_and_position() {
        let mut list = PointList::<usize, char>::new();

        assert!(list.frames.is_empty());
        assert!(list.root.is_none());
        assert!(list.start.is_zero());
        assert!(list.len.is_zero());
        assert!(list.persistent_to_ephemeral.is_empty());

        let a_index = list.add_element('a', 4);

        let root_key = list.frames.keys().exactly_one().unwrap();
        assert_eq!(list.root, Some(root_key));
        assert_eq!(list.start, 4);
        assert_eq!(list.len, 1);
        assert_eq!(list.persistent_to_ephemeral, vec![Some(EphemeralIndex::new(root_key, 0))]);
        assert_eq!(list.element(a_index), Some(&'a'));
        assert_eq!(list.position(a_index), Some(4));

        let b_index = list.add_element('b', 2);

        let root_key = list.frames.keys().exactly_one().unwrap();
        assert_eq!(list.root, Some(root_key));
        assert_eq!(list.start, 4);
        assert_eq!(list.len, 2);
        assert_eq!(list.persistent_to_ephemeral, vec![
            Some(EphemeralIndex::new(root_key, 0)),
            Some(EphemeralIndex::new(root_key, 1)),
        ]);
        assert_eq!(list.element(a_index), Some(&'a'));
        assert_eq!(list.position(a_index), Some(4));
        assert_eq!(list.element(b_index), Some(&'b'));
        assert_eq!(list.position(b_index), Some(6));

        let c_index = list.add_element('c', 3);

        let root_key = list.frames.keys().exactly_one().unwrap();
        assert_eq!(list.root, Some(root_key));
        assert_eq!(list.start, 4);
        assert_eq!(list.len, 3);
        assert_eq!(list.persistent_to_ephemeral, vec![
            Some(EphemeralIndex::new(root_key, 0)),
            Some(EphemeralIndex::new(root_key, 1)),
            Some(EphemeralIndex::new(root_key, 2)),
        ]);
        assert_eq!(list.element(a_index), Some(&'a'));
        assert_eq!(list.position(a_index), Some(4));
        assert_eq!(list.element(b_index), Some(&'b'));
        assert_eq!(list.position(b_index), Some(6));
        assert_eq!(list.element(c_index), Some(&'c'));
        assert_eq!(list.position(c_index), Some(9));

        let d_index = list.add_element('d', 1);

        let root_key = list.frames.keys().exactly_one().unwrap();
        assert_eq!(list.root, Some(root_key));
        assert_eq!(list.start, 4);
        assert_eq!(list.len, 4);
        assert_eq!(list.persistent_to_ephemeral, vec![
            Some(EphemeralIndex::new(root_key, 0)),
            Some(EphemeralIndex::new(root_key, 1)),
            Some(EphemeralIndex::new(root_key, 2)),
            Some(EphemeralIndex::new(root_key, 3)),
        ]);
        assert_eq!(list.element(a_index), Some(&'a'));
        assert_eq!(list.position(a_index), Some(4));
        assert_eq!(list.element(b_index), Some(&'b'));
        assert_eq!(list.position(b_index), Some(6));
        assert_eq!(list.element(c_index), Some(&'c'));
        assert_eq!(list.position(c_index), Some(9));
        assert_eq!(list.element(d_index), Some(&'d'));
        assert_eq!(list.position(d_index), Some(10));
    }
}
