use std::fmt::{Debug, Formatter};
use std::cmp::Ordering;
use slotmap::Key;
use crate::{Element, PointList, Position};
use crate::frame::distances::{Distances, DISTANCES_DEPTH};
use crate::frame::{EitherFrame, Frame};

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
