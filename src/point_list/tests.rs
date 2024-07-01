use itertools::Itertools;
use num_traits::{Zero, zero};
use crate::{IndexInFrame, PointList, Position, Element, PointKey};

/*#[test]
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
    assert_eq!(list.end, 4);
    assert_eq!(list.len, 1);
    assert_eq!(list.point_indices.len(), 1);
    assert_eq!(list.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list.element(a_key), Some(&'a'));
    assert_eq!(list.position(a_key), Some(4));

    let b_key = list.add_element('b', 2);

    let root_key = list.frames.keys().exactly_one().unwrap();
    assert_eq!(list.root, Some(root_key));
    assert_eq!(list.start, 4);
    assert_eq!(list.end, 6);
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
    assert_eq!(list.end, 9);
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
    assert_eq!(list.end, 10);
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

    let mut list_without_a = list.clone();

    list_without_a.remove_element(a_key);

    let root_key = list_without_a.frames.keys().exactly_one().unwrap();
    assert_eq!(list_without_a.root, Some(root_key));
    assert_eq!(list_without_a.start, 6);
    assert_eq!(list.end, 10);
    assert_eq!(list_without_a.len, 3);
    assert_eq!(list_without_a.point_indices.len(), 3);
    assert_eq!(list_without_a.point_indices[b_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list_without_a.point_indices[c_key], IndexInFrame::new(root_key, 1));
    assert_eq!(list_without_a.point_indices[d_key], IndexInFrame::new(root_key, 2));
    assert_eq!(list_without_a.element(a_key), None);
    assert_eq!(list_without_a.position(a_key), None);
    assert_eq!(list_without_a.element(b_key), Some(&'b'));
    assert_eq!(list_without_a.position(b_key), Some(6));
    assert_eq!(list_without_a.element(c_key), Some(&'c'));
    assert_eq!(list_without_a.position(c_key), Some(9));
    assert_eq!(list_without_a.element(d_key), Some(&'d'));
    assert_eq!(list_without_a.position(d_key), Some(10));

    let mut list_without_b = list.clone();

    list_without_b.remove_element(b_key);

    let root_key = list_without_b.frames.keys().exactly_one().unwrap();
    assert_eq!(list_without_b.root, Some(root_key));
    assert_eq!(list_without_b.start, 4);
    assert_eq!(list_without_b.end, 10);
    assert_eq!(list_without_b.len, 3);
    assert_eq!(list_without_b.point_indices.len(), 3);
    assert_eq!(list_without_b.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list_without_b.point_indices[c_key], IndexInFrame::new(root_key, 1));
    assert_eq!(list_without_b.point_indices[d_key], IndexInFrame::new(root_key, 2));
    assert_eq!(list_without_b.element(a_key), Some(&'a'));
    assert_eq!(list_without_b.position(a_key), Some(4));
    assert_eq!(list_without_b.element(b_key), None);
    assert_eq!(list_without_b.position(b_key), None);
    assert_eq!(list_without_b.element(c_key), Some(&'c'));
    assert_eq!(list_without_b.position(c_key), Some(9));
    assert_eq!(list_without_b.element(d_key), Some(&'d'));
    assert_eq!(list_without_b.position(d_key), Some(10));

    let mut list_without_c = list.clone();

    list_without_c.remove_element(c_key);

    let root_key = list_without_c.frames.keys().exactly_one().unwrap();
    assert_eq!(list_without_c.root, Some(root_key));
    assert_eq!(list_without_c.start, 4);
    assert_eq!(list_without_c.end, 10);
    assert_eq!(list_without_c.len, 3);
    assert_eq!(list_without_c.point_indices.len(), 3);
    assert_eq!(list_without_c.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list_without_c.point_indices[b_key], IndexInFrame::new(root_key, 1));
    assert_eq!(list_without_c.point_indices[d_key], IndexInFrame::new(root_key, 2));
    assert_eq!(list_without_c.element(a_key), Some(&'a'));
    assert_eq!(list_without_c.position(a_key), Some(4));
    assert_eq!(list_without_c.element(b_key), Some(&'b'));
    assert_eq!(list_without_c.position(b_key), Some(6));
    assert_eq!(list_without_c.element(c_key), None);
    assert_eq!(list_without_c.position(c_key), None);
    assert_eq!(list_without_c.element(d_key), Some(&'d'));
    assert_eq!(list_without_c.position(d_key), Some(10));

    let mut list_without_d = list.clone();

    list_without_d.remove_element(d_key);

    let root_key = list_without_d.frames.keys().exactly_one().unwrap();
    assert_eq!(list_without_d.root, Some(root_key));
    assert_eq!(list_without_d.start, 4);
    assert_eq!(list_without_d.end, 9);
    assert_eq!(list_without_d.len, 3);
    assert_eq!(list_without_d.point_indices.len(), 3);
    assert_eq!(list_without_d.point_indices[a_key], IndexInFrame::new(root_key, 0));
    assert_eq!(list_without_d.point_indices[b_key], IndexInFrame::new(root_key, 1));
    assert_eq!(list_without_d.point_indices[c_key], IndexInFrame::new(root_key, 2));
    assert_eq!(list_without_d.element(a_key), Some(&'a'));
    assert_eq!(list_without_d.position(a_key), Some(4));
    assert_eq!(list_without_d.element(b_key), Some(&'b'));
    assert_eq!(list_without_d.position(b_key), Some(6));
    assert_eq!(list_without_d.element(c_key), Some(&'c'));
    assert_eq!(list_without_d.position(c_key), Some(9));
    assert_eq!(list_without_d.element(d_key), None);
    assert_eq!(list_without_d.position(d_key), None);
}*/

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

fn list_from_array<P: Position, E: Element, const N: usize>(array: [(E, P); N]) -> (PointList<P, E>, [PointKey; N]) {
    let mut list = PointList::new();
    let mut keys = [PointKey::default(); N];
    let mut last_position = zero();
    for (i, (e, p)) in array.into_iter().enumerate() {
        keys[i] = list.add_element(e, p - last_position);
        last_position = p;
    }
    (list, keys)
}

#[test]
fn test_add_element_remove_element() {
    let (mut list1, [a1, ..]) = list_from_array([('a', 4), ('b', 6), ('c', 9), ('d', 10), ('e', 12), ('f', 13)]);
    let (list2, [..]) = list_from_array([('b', 6), ('c', 9), ('d', 10), ('e', 12), ('f', 13)]);
    std::fs::write("out/list1_original.txt", format!("{:?}", list1)).unwrap();
    list1.remove_element(a1);
    std::fs::write("out/list1.txt", format!("{:?}", list1)).unwrap();
    std::fs::write("out/list2.txt", format!("{:?}", list2)).unwrap();
}
