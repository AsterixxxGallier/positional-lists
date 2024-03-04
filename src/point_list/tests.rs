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

    println!("{}", list);
}

#[test]
fn test_add_element_and_position_2() {
    let mut list = PointList::new();
    for i in 0..(1 << 20) {
        let index = list.add_element((), 1);
        assert_eq!(list.len(), i + 1);
        assert_eq!(list.start(), 1);
        assert_eq!(list.position(index), Some(i + 1));
    }
}
