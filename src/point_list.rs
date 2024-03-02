use slotmap::SlotMap;
use num_traits::zero;
use crate::{Element, Position, Frame, FrameKey, EphemeralIndex, PersistentIndex, Embedding, Slot};

#[derive(Debug)]
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
            let (frame, index) = Frame::new_with_element(element, persistent_index, Embedding::InList);
            let root_key = self.frames.insert(frame);
            self.root = Some(root_key);
            let ephemeral_index = EphemeralIndex::new(root_key, index);
            self.persistent_to_ephemeral.push(Some(ephemeral_index));
            persistent_index
        }
    }

    fn ephemeral(&self, index: PersistentIndex) -> Option<EphemeralIndex> {
        self.persistent_to_ephemeral[index.index]
    }

    fn frame(&self, index: EphemeralIndex) -> &Frame<P, E> {
        &self.frames[index.frame]
    }

    fn slot(&self, index: PersistentIndex) -> Option<&Slot<E>> {
        let ephemeral = self.ephemeral(index)?;
        Some(&self.frame(ephemeral).slots[ephemeral.index])
    }

    pub fn element(&self, index: PersistentIndex) -> Option<&E> {
        match self.slot(index)? {
            Slot::Element { value, .. } => Some(value),
            Slot::Frame(_) => unreachable!(),
            Slot::Empty => None
        }
    }

    pub fn position(&self, index: PersistentIndex) -> Option<P> {
        let mut position = self.start;
        let index = self.ephemeral(index)?;
        let mut frame = self.frame(index);
        position += frame.position(index.index);
        while let Embedding::InOuterFrame(index) = frame.embedding {
            frame = self.frame(index);
            position += frame.position(index.index);
        }
        Some(position)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use num_traits::Zero;
    use crate::{PointList, EphemeralIndex};

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
