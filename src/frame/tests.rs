use arrayvec::ArrayVec;
use crate::{ElementFrame, Distances, Embedding, PersistentIndex};

#[test]
fn test_add_element() {
    let (mut frame, index) = ElementFrame::<usize, char>::new_with_element('a', PersistentIndex::new(0), Embedding::InList);

    assert_eq!(index, 0);
    assert_eq!(frame.distances, Distances {
        distances: ArrayVec::new(),
        depth: 0,
    });
    let mut iter = frame.elements.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), None);
    let mut iter = frame.persistent_indices.iter();
    assert_eq!(iter.next(), Some(&PersistentIndex::new(0)));
    assert_eq!(iter.next(), None);

    let index = frame.add_element('b', PersistentIndex::new(1), 1);

    assert_eq!(index, 1);
    assert_eq!(frame.distances, Distances {
        distances: ArrayVec::from_iter([1].into_iter()),
        depth: 1,
    });
    let mut iter = frame.elements.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), None);
    let mut iter = frame.persistent_indices.iter();
    assert_eq!(iter.next(), Some(&PersistentIndex::new(0)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(1)));
    assert_eq!(iter.next(), None);

    let index = frame.add_element('c', PersistentIndex::new(2), 2);

    assert_eq!(index, 2);
    assert_eq!(frame.distances, Distances {
        distances: ArrayVec::from_iter([1, 3].into_iter()),
        depth: 2,
    });
    let mut iter = frame.elements.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'c'));
    assert_eq!(iter.next(), None);
    let mut iter = frame.persistent_indices.iter();
    assert_eq!(iter.next(), Some(&PersistentIndex::new(0)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(1)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(2)));
    assert_eq!(iter.next(), None);

    let index = frame.add_element('d', PersistentIndex::new(3), 3);

    assert_eq!(index, 3);
    assert_eq!(frame.distances, Distances {
        distances: ArrayVec::from_iter([1, 3, 3].into_iter()),
        depth: 2,
    });
    let mut iter = frame.elements.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'c'));
    assert_eq!(iter.next(), Some(&'d'));
    assert_eq!(iter.next(), None);
    let mut iter = frame.persistent_indices.iter();
    assert_eq!(iter.next(), Some(&PersistentIndex::new(0)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(1)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(2)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(3)));
    assert_eq!(iter.next(), None);

    let index = frame.add_element('e', PersistentIndex::new(4), 4);

    assert_eq!(index, 4);
    assert_eq!(frame.distances, Distances {
        distances: ArrayVec::from_iter([1, 3, 3, 10].into_iter()),
        depth: 3,
    });
    let mut iter = frame.elements.iter();
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'c'));
    assert_eq!(iter.next(), Some(&'d'));
    assert_eq!(iter.next(), Some(&'e'));
    assert_eq!(iter.next(), None);
    let mut iter = frame.persistent_indices.iter();
    assert_eq!(iter.next(), Some(&PersistentIndex::new(0)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(1)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(2)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(3)));
    assert_eq!(iter.next(), Some(&PersistentIndex::new(4)));
    assert_eq!(iter.next(), None);
}
