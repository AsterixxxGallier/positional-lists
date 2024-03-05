use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use slotmap::{Key, new_key_type, SecondaryMap, SlotMap};
use num_traits::zero;
use crate::{Position, Element, Frame, EitherFrame, MetaFrame, BaseFrame, Distances, Embedding, FrameKey, IndexInFrame, DISTANCES_DEPTH};

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

impl<P: Position, E: Element> Debug for PointList<P, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return f.write_str("[empty PointList]");
        }

        writeln!(f, "len: {}", self.len)?;
        writeln!(f, "start: {}", self.start)?;
        writeln!(f, "end: {}", self.end)?;
        writeln!(f, "root: {:?}", self.root.unwrap().data())?;
        writeln!(f)?;

        let distance_width = self.end.to_string().len();
        let point_key_width = self.point_indices.keys().map(|key| format!("{:?}", key.data()).len()).max().unwrap();
        let frame_key_width = self.frames.keys().map(|key| format!("{:?}", key.data()).len()).max().unwrap();
        let width = distance_width.max(point_key_width).max(frame_key_width);

        for (key, frame) in &self.frames {
            writeln!(f, "{:?} (level {}):", key.data(), frame.level())?;
            let distances: &Distances<P> = frame.distances();
            for degree in (0..DISTANCES_DEPTH).rev() {
                let distances = &distances.distances;
                for (index, &distance) in distances.iter().enumerate() {
                    let index_degree = index.trailing_ones() as usize;
                    match index_degree.cmp(&degree) {
                        Ordering::Equal => write!(f, "{:·>width$} ", distance, width = width)?,
                        Ordering::Greater => write!(f, "{:>width$} ", "╎", width = width)?,
                        Ordering::Less => if (index >> degree) & 1 == 0 {
                            write!(f, "{}", "·".repeat(width + 1))?;
                        } else {
                            write!(f, "{}", " ".repeat(width + 1))?;
                        },
                    }
                }
                writeln!(f)?;
            }
            match frame {
                EitherFrame::Meta(frame) => {
                    for key in &frame.frames {
                        write!(f, "{:>width$?} ", key.data(), width = width)?;
                    }
                }
                EitherFrame::Base(frame) => {
                    for key in &frame.keys {
                        write!(f, "{:>width$?} ", key.data(), width = width)?;
                    }
                }
            }
            writeln!(f)?;
            writeln!(f)?;
        }

        for (key, &index_in_frame) in &self.point_indices {
            writeln!(f, "{:>width$?}: {:?} ({:?}/{})", key.data(), self.element(key).unwrap(), index_in_frame.frame.data(), index_in_frame.index, width = width)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
