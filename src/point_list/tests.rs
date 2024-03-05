use itertools::Itertools;
use num_traits::Zero;
use crate::{IndexInFrame, PointList};

#[test]
fn test_add_element_and_position() {
    let mut list = PointList::<usize, char>::new();

    assert!(list.frames.is_empty());
    assert!(list.root.is_none());
    assert!(list.start.is_zero());
    assert!(list.len.is_zero());
    assert!(list.point_indices.is_empty());

    let a_key = list.add_element('a', 4);

    let root_key = list.frames.keys().exactly_one().unwrap();
    assert_eq!(list.root, Some(root_key));
    assert_eq!(list.start, 4);
    assert_eq!(list.len, 1);
    assert_eq!(list.point_indices.len(), 1);
    assert_eq!(list.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list.element(a_key), Some(&'a'));
    assert_eq!(list.position(a_key), Some(4));

    let b_key = list.add_element('b', 2);

    let root_key = list.frames.keys().exactly_one().unwrap();
    assert_eq!(list.root, Some(root_key));
    assert_eq!(list.start, 4);
    assert_eq!(list.len, 2);
    assert_eq!(list.point_indices.len(), 2);
    assert_eq!(list.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list.point_indices[b_key], IndexInFrame::new(root_key, 1));
    assert_eq!(list.element(a_key), Some(&'a'));
    assert_eq!(list.position(a_key), Some(4));
    assert_eq!(list.element(b_key), Some(&'b'));
    assert_eq!(list.position(b_key), Some(6));

    let c_key = list.add_element('c', 3);

    let root_key = list.frames.keys().exactly_one().unwrap();
    assert_eq!(list.root, Some(root_key));
    assert_eq!(list.start, 4);
    assert_eq!(list.len, 3);
    assert_eq!(list.point_indices.len(), 3);
    assert_eq!(list.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list.point_indices[b_key], IndexInFrame::new(root_key, 1));
    assert_eq!(list.point_indices[c_key], IndexInFrame::new(root_key, 2));
    assert_eq!(list.element(a_key), Some(&'a'));
    assert_eq!(list.position(a_key), Some(4));
    assert_eq!(list.element(b_key), Some(&'b'));
    assert_eq!(list.position(b_key), Some(6));
    assert_eq!(list.element(c_key), Some(&'c'));
    assert_eq!(list.position(c_key), Some(9));

    let d_key = list.add_element('d', 1);

    let root_key = list.frames.keys().exactly_one().unwrap();
    assert_eq!(list.root, Some(root_key));
    assert_eq!(list.start, 4);
    assert_eq!(list.len, 4);
    assert_eq!(list.point_indices.len(), 4);
    assert_eq!(list.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list.point_indices[b_key], IndexInFrame::new(root_key, 1));
    assert_eq!(list.point_indices[c_key], IndexInFrame::new(root_key, 2));
    assert_eq!(list.point_indices[d_key], IndexInFrame::new(root_key, 3));
    assert_eq!(list.element(a_key), Some(&'a'));
    assert_eq!(list.position(a_key), Some(4));
    assert_eq!(list.element(b_key), Some(&'b'));
    assert_eq!(list.position(b_key), Some(6));
    assert_eq!(list.element(c_key), Some(&'c'));
    assert_eq!(list.position(c_key), Some(9));
    assert_eq!(list.element(d_key), Some(&'d'));
    assert_eq!(list.position(d_key), Some(10));

    // println!("{}", list);
}

#[test]
fn test_add_element_and_position_2() {
    let mut list = PointList::new();
    for i in 0..(1 << 17) {
        let key = list.add_element((), 1);
        assert_eq!(list.len(), i + 1);
        assert_eq!(list.start(), 1);
        assert_eq!(list.position(key), Some(i + 1));
    }
    // std::fs::write("out.txt", format!("{}", list)).unwrap();
}
