use num_traits::zero;

use crate::{Element, Embedding, PointKey, PointList, Position};
use crate::frame::{EitherFrame, Frame, FRAME_CAPACITY, FrameKey, IndexInFrame};

impl<P: Position, E: Element> PointList<P, E> {
    fn try_merge(&mut self, meta_frame_key: FrameKey, left_index: usize) -> Result<(), ()> {
        let right_index = left_index + 1;

        let meta_frame = self.frames[meta_frame_key].unwrap_meta();

        if right_index >= meta_frame.frames.len() {
            return Err(());
        }

        let left_frame_key = meta_frame.frames[left_index];
        let right_frame_key = meta_frame.frames[right_index];
        
        if dbg!(self.frames[left_frame_key].len()) + dbg!(self.frames[right_frame_key].len()) > dbg!(FRAME_CAPACITY) {
            return Err(());
        }

        let distance_between_frames = meta_frame.distances.distance(left_index) - self.frames[left_frame_key].distances().length();

        let right_frame = self.frames.remove(right_frame_key).unwrap();
        let left_frame = &mut self.frames[left_frame_key];

        match left_frame {
            EitherFrame::Meta(left_frame) => {
                let right_frame = right_frame.unwrap_meta_owned();

                // let meta_frame = self.frames[meta_frame_key].unwrap_meta();

                // alternative that might work (semi-pseudocode) and _might_ even be faster
                // (especially when exploiting SIMD for the bulk addition):
                // left_frame.distances.distances += right_frame.distances.splice(0..0, left_frame.len()).distances
                // same goes for below with the Base case
                left_frame.distances.increase_distance(left_frame.len() - 1, distance_between_frames);
                for i in 0..right_frame.len() - 1 {
                    left_frame.distances.increase_distance(left_frame.len() - 1 + i + 1, right_frame.distances.distance(i))
                }
                
                left_frame.frames.try_extend_from_slice(right_frame.frames.as_slice()).unwrap();
            }
            EitherFrame::Base(left_frame) => {
                let right_frame = right_frame.unwrap_base_owned();

                left_frame.distances.increase_distance(left_frame.len() - 1, distance_between_frames);
                for i in 0..right_frame.len() - 1 {
                    left_frame.distances.increase_distance(left_frame.len() - 1 + i + 1, right_frame.distances.distance(i))
                }
                
                left_frame.keys.try_extend_from_slice(right_frame.keys.as_slice()).unwrap();
            }
        }

        let meta_frame = self.frames[meta_frame_key].unwrap_meta_mut();
        meta_frame.frames.remove(right_index);
        let distance_between_left_and_right = meta_frame.distances.distance(left_index);
        meta_frame.distances.remove(right_index);
        meta_frame.distances.increase_distance(left_index, distance_between_left_and_right);

        Ok(())
    }

    fn lower(&mut self, embedding: Embedding) {
        if let Embedding::InMetaFrame(in_meta_frame) = embedding {
            let frame = self.frames[in_meta_frame.frame].unwrap_meta_mut();
            frame.level -= 1;
            let frame = self.frames[in_meta_frame.frame].unwrap_meta();
            self.lower(frame.embedding);
        }
    }

    fn try_merge_around(&mut self, key: FrameKey) {
        if let Embedding::InMetaFrame(in_meta_frame) = self.frames[key].embedding() {
            let index = in_meta_frame.index;
            if index > 0 &&
                self.try_merge(in_meta_frame.frame, index - 1)
                    .or_else(|_| self.try_merge(in_meta_frame.frame, index))
                    .is_ok()
                || index == 0 && self.try_merge(in_meta_frame.frame, index).is_ok() {
                // one key / frame was removed
                // we might now be able to merge around the meta frame
                self.try_merge_around(in_meta_frame.frame);
                // and now we might be able to dissolve the meta frame
                if self.frames[in_meta_frame.frame].unwrap_meta().frames.len() == 1 {
                    let meta_frame = self.frames.remove(in_meta_frame.frame).unwrap().unwrap_meta_owned();
                    self.lower(meta_frame.embedding);
                    *self.frames[meta_frame.frames[0]].embedding_mut() = meta_frame.embedding;
                }
            }
        } else {
            // nothing can be merged
        }
    }

    fn replenish_distance(&mut self, IndexInFrame { index, frame }: IndexInFrame, distance: P) {
        let distances = self.frames[frame].distances_mut();
        if index > 0 {
            distances.increase_distance(index - 1, distance);
        } else if let Embedding::InMetaFrame(index_in_meta_frame) = self.frames[frame].embedding() {
            self.replenish_distance(index_in_meta_frame, distance);
        }
    }

    pub fn remove_element(&mut self, key: PointKey) -> Option<E> {
        if self.is_empty() { return None; }

        let index_in_frame_of_removed = self.point_indices.get(key)?;
        let frame_key_of_removed = index_in_frame_of_removed.frame;
        let index_of_removed = index_in_frame_of_removed.index;
        // if self were empty, we would have returned None earlier
        let first_key = self.first_key().unwrap();
        let last_key = self.last_key().unwrap();

        // don't return None after this, as then len would have been decreased without an element
        // having been removed!
        self.len -= 1;

        /// if `key` is the only key in the list, the list should be reverted to the same state it
        /// was in directly after initialisation
        if key == first_key && key == last_key {
            self.root = None;
            self.frames.clear();
            self.start = zero();
            self.end = zero();
            self.point_indices.clear();
            return Some(self.elements.remove(key).unwrap());
        }

        let frame_of_removed = self.frames[frame_key_of_removed].unwrap_base_mut();
        let distance_after_removed = frame_of_removed.distances.distance(index_of_removed);

        /// if `key` is the first key in the list, adjust `self.start`
        if key == first_key {
            self.start += distance_after_removed;
        }

        /// if `key` is the last key in the list, adjust `self.end`
        if key == last_key {
            let distance_before_removed = frame_of_removed.distances.distance(index_of_removed - 1);
            self.end -= distance_before_removed;
        }

        frame_of_removed.distances.remove(index_of_removed);

        self.replenish_distance(*index_in_frame_of_removed, distance_after_removed);

        // redeclaration because it makes the borrow checker happy
        let frame_of_removed = self.frames[frame_key_of_removed].unwrap_base_mut();

        // remove key
        frame_of_removed.keys.remove(index_of_removed);

        // update point_indices
        self.point_indices.remove(key);
        for (index, &point_key) in frame_of_removed.keys.iter().enumerate().skip(index_of_removed) {
            self.point_indices[point_key].index = index;
        }

        self.try_merge_around(frame_key_of_removed);

        // TODO if possible, join frame with the next / previous frame to prevent excessive
        //  fragmentation of the PointList's structure (should be done)

        // FIXME this function does not yet protect against the creation of empty frames! (should be done)

        Some(self.elements.remove(key).unwrap())
    }
}
