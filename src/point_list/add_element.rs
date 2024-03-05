use num_traits::zero;
use crate::{PointList, PointKey, BaseFrame, EitherFrame, Element, Embedding, Frame, FrameKey, IndexInFrame, MetaFrame, Position};

// for PointList::add_element
enum AddKeyStrategy {
    NewBaseFrameNecessary {
        frame_with_full_last_frame: FrameKey,
        last_frame: FrameKey,
    },
    KeyCanBeAddedToExistingBaseFrame {
        base_frame: FrameKey,
    },
}

impl<P: Position, E: Element> PointList<P, E> {
    fn key_can_be_added_to(&self, frame_key: FrameKey) -> bool {
        match &self.frames[frame_key] {
            EitherFrame::Meta(frame) =>
                !frame.is_full() || self.key_can_be_added_to(frame.last_frame()),
            EitherFrame::Base(frame) =>
                !frame.is_full(),
        }
    }

    fn ensure_key_can_be_added_to_root(&mut self) {
        let root_key = self.root.unwrap();
        if !self.key_can_be_added_to(root_key) {
            // put frame into a MetaFrame
            let level = self.frames[root_key].level() + 1;
            let (new_root, index) =
                MetaFrame::new_with_frame(root_key, level, Embedding::InList);
            let new_root_key = self.frames.insert(new_root.into());
            self.frames[root_key].embed(Embedding::InMetaFrame(IndexInFrame::new(new_root_key, index)));
            self.root = Some(new_root_key);
        }
    }

    fn add_base_frame(&mut self) -> (FrameKey, PointKey) {
        let mut frame_key = None;
        let point_key = self.point_indices.insert_with_key(|point_key| {
            let (base_frame, index) =
                BaseFrame::new_with_key(point_key, Embedding::InList);
            let base_frame_key = self.frames.insert(base_frame.into());
            frame_key = Some(base_frame_key);
            IndexInFrame::new(base_frame_key, index)
        });
        (frame_key.unwrap(), point_key)
    }

    fn add_key_strategy(&self) -> AddKeyStrategy {
        let mut frame_key = self.root.unwrap();
        loop {
            match &self.frames[frame_key] {
                EitherFrame::Meta(meta_frame) => {
                    if self.key_can_be_added_to(meta_frame.last_frame()) {
                        frame_key = meta_frame.last_frame();
                    } else {
                        break AddKeyStrategy::NewBaseFrameNecessary {
                            frame_with_full_last_frame: frame_key,
                            last_frame: meta_frame.last_frame(),
                        };
                    }
                }
                EitherFrame::Base(_) => {
                    break AddKeyStrategy::KeyCanBeAddedToExistingBaseFrame {
                        base_frame: frame_key,
                    };
                }
            }
        }
    }

    pub fn add_element(&mut self, element: E, distance_from_last: P) -> PointKey {
        self.len += 1;

        if self.root.is_some() {
            // Distances of zero are not allowed.
            assert!(distance_from_last > zero());

            self.end += distance_from_last;

            self.ensure_key_can_be_added_to_root();

            match self.add_key_strategy() {
                AddKeyStrategy::NewBaseFrameNecessary {
                    frame_with_full_last_frame,
                    last_frame
                } => {
                    let (mut current_frame, point_key) = self.add_base_frame();

                    self.elements.insert(point_key, element);

                    // wrap current_frame in MetaFrames until it and last_frame have the same level
                    for level in 1..=self.frames[last_frame].level() {
                        let (meta_frame, index) = MetaFrame::new_with_frame(current_frame, level, Embedding::InList);
                        let meta_frame_key = self.frames.insert(meta_frame.into());
                        self.frames[current_frame].embed(Embedding::InMetaFrame(IndexInFrame::new(meta_frame_key, index)));
                        current_frame = meta_frame_key;
                    }

                    // add current_frame to frame_with_full_last_frame
                    let index = self.frames[frame_with_full_last_frame].unwrap_meta().frames.len();
                    self.frames[current_frame].embed(Embedding::InMetaFrame(IndexInFrame::new(frame_with_full_last_frame, index)));
                    let length_of_last_frame = self.length_of(last_frame);
                    let frame = self.frames[frame_with_full_last_frame].unwrap_meta_mut();
                    frame.add_frame(current_frame, distance_from_last + length_of_last_frame);

                    point_key
                }
                AddKeyStrategy::KeyCanBeAddedToExistingBaseFrame {
                    base_frame: element_frame
                } => {
                    let frame = self.frames[element_frame].unwrap_base_mut();
                    let point_key = self.point_indices.insert_with_key(|point_key| {
                        let index = frame.add_key(point_key, distance_from_last);
                        IndexInFrame::new(element_frame, index)
                    });

                    self.elements.insert(point_key, element);

                    point_key
                }
            }
        } else {
            self.start = distance_from_last;
            self.end = distance_from_last;
            let (frame, point_key) = self.add_base_frame();
            self.root = Some(frame);

            self.elements.insert(point_key, element);

            point_key
        }
    }
}
