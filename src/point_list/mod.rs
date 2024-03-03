use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use slotmap::{Key, SlotMap};
use num_traits::zero;
use crate::{Position, Element, Frame, EitherFrame, MetaFrame, ElementFrame, Embedding, FrameKey, EphemeralIndex, PersistentIndex};
use crate::frame::distances::Distances;

pub struct PointList<P: Position, E: Element> {
    frames: SlotMap<FrameKey, EitherFrame<P, E>>,
    root: Option<FrameKey>,
    start: P,
    end: P,
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
            end: zero(),
            len: 0,
            persistent_to_ephemeral: vec![],
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

            self.end += distance_from_last;

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
                    // TODO double-check the self.end - self.start part
                    self.frames[frame_with_full_last_frame].unwrap_meta_mut().add_frame(current_frame, distance_from_last + (self.end - self.start));
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
            self.end = distance_from_last;
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

impl<P: Position, E: Element> Debug for PointList<P, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointList")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("len", &self.len)
            .field_with(
                "persistent_to_ephemeral",
                |f| {
                    writeln!(f)?;
                    for (persistent, ephemeral) in self.persistent_to_ephemeral.iter().enumerate() {
                        match ephemeral {
                            Some(ephemeral) => writeln!(f, "{:5}: {:?}/{}", persistent, ephemeral.frame, ephemeral.index)?,
                            None => writeln!(f, "{:5}: removed", persistent)?,
                        }
                    }
                    Ok(())
                },
            )
            .field_with("root", |f| write!(f, "{:?}", self.root))
            .field_with(
                "frames",
                |f| {
                    for (key, frame) in &self.frames {
                        writeln!(f, "{:?}: ", key)?;
                        match frame {
                            EitherFrame::Meta(frame) => frame.fmt(f)?,
                            EitherFrame::Element(frame) => frame.fmt(f)?,
                        }
                    }
                    Ok(())
                },
            )
            .finish()
    }
}

impl<P: Position, E: Element> Display for PointList<P, E> {
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
        let index_width = (self.persistent_to_ephemeral.len() - 1).to_string().len();
        let key_width = self.frames.keys().map(|key| format!("{:?}", key.data()).len()).max().unwrap();
        let width = distance_width.max(index_width).max(key_width);

        for (key, frame) in &self.frames {
            writeln!(f, "{:?} (level {}):", key.data(), frame.level())?;
            let distances: &Distances<P> = frame.distances();
            for degree in (0..distances.depth).rev() {
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
                EitherFrame::Element(frame) => {
                    for index in &frame.persistent_indices {
                        write!(f, "{:>width$?} ", index.index, width = width)?;
                    }
                }
            }
            writeln!(f)?;
            writeln!(f)?;
        }

        for (persistent, ephemeral) in self.persistent_to_ephemeral.iter().enumerate() {
            if let Some(ephemeral) = ephemeral {
                writeln!(f, "{:>width$}: {:?} ({:?}/{})", persistent, self.element(PersistentIndex::new(persistent)).unwrap(), ephemeral.frame.data(), ephemeral.index, width = width)?;
            } else {
                writeln!(f, "{:>width$}: removed", persistent, width = width)?;
            }
        }

        /*
        _______________
        _______     421
        ___ 324 ___
        231     212
        000 001 002 003

        ············421
        ····324       ╎ ····713
        231   ╎ 212   ╎ 465   ╎
        000 001 010 011 100 101
         */

        Ok(())
    }
}

#[cfg(test)]
mod tests;
