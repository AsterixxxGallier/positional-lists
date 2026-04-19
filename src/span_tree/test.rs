use super::*;
use itertools::Itertools;

#[test]
fn test_level_to_len() {
    assert_eq!(level_to_len(0), 1);
    assert_eq!(level_to_len(1), 2);
    assert_eq!(level_to_len(2), 4);
    assert_eq!(level_to_len(3), 8);
}

#[test]
fn test_len_to_level() {
    assert_eq!(len_to_level(1), 0);
    assert_eq!(len_to_level(2), 1);
    assert_eq!(len_to_level(4), 2);
    assert_eq!(len_to_level(8), 3);
}

#[test]
fn test_child_offset() {
    assert_eq!(child_offset(4, 3), 0);
    assert_eq!(child_offset(4, 2), 8);
    assert_eq!(child_offset(4, 1), 12);
    assert_eq!(child_offset(4, 0), 14);
}

#[test]
fn test_end_to_len() {
    assert_eq!(end_to_len(1), 1);
    assert_eq!(end_to_len(2), 2);
    assert_eq!(end_to_len(3), 1);
    assert_eq!(end_to_len(4), 4);
    assert_eq!(end_to_len(5), 1);
    assert_eq!(end_to_len(6), 2);
    assert_eq!(end_to_len(7), 1);
    assert_eq!(end_to_len(8), 8);
}

#[test]
fn test_parent_tree_index() {
    assert_eq!(parent_tree_index(0), 1);
    assert_eq!(parent_tree_index(1), 3);
    assert_eq!(parent_tree_index(2), 3);
    assert_eq!(parent_tree_index(3), 7);
    assert_eq!(parent_tree_index(4), 5);
    assert_eq!(parent_tree_index(5), 7);
    assert_eq!(parent_tree_index(6), 7);
    assert_eq!(parent_tree_index(7), 15);
    assert_eq!(parent_tree_index(8), 9);
    assert_eq!(parent_tree_index(9), 11);
    assert_eq!(parent_tree_index(10), 11);
    assert_eq!(parent_tree_index(11), 15);
    assert_eq!(parent_tree_index(12), 13);
    assert_eq!(parent_tree_index(13), 15);
    assert_eq!(parent_tree_index(14), 15);
}

#[test]
fn test_parent_tree_indices() {
    assert_eq!(parent_tree_indices(0).take(4).collect_vec(), vec![1, 3, 7, 15]);
    assert_eq!(parent_tree_indices(1).take(4).collect_vec(), vec![3, 7, 15, 31]);
    assert_eq!(parent_tree_indices(2).take(4).collect_vec(), vec![3, 7, 15, 31]);
    assert_eq!(parent_tree_indices(3).take(4).collect_vec(), vec![7, 15, 31, 63]);
    assert_eq!(parent_tree_indices(4).take(4).collect_vec(), vec![5, 7, 15, 31]);
    assert_eq!(parent_tree_indices(5).take(4).collect_vec(), vec![7, 15, 31, 63]);
    assert_eq!(parent_tree_indices(6).take(4).collect_vec(), vec![7, 15, 31, 63]);
    assert_eq!(parent_tree_indices(7).take(4).collect_vec(), vec![15, 31, 63, 127]);
    assert_eq!(parent_tree_indices(8).take(4).collect_vec(), vec![9, 11, 15, 31]);
    assert_eq!(parent_tree_indices(9).take(4).collect_vec(), vec![11, 15, 31, 63]);
    assert_eq!(parent_tree_indices(10).take(4).collect_vec(), vec![11, 15, 31, 63]);
    assert_eq!(parent_tree_indices(11).take(4).collect_vec(), vec![15, 31, 63, 127]);
    assert_eq!(parent_tree_indices(12).take(4).collect_vec(), vec![13, 15, 31, 63]);
    assert_eq!(parent_tree_indices(13).take(4).collect_vec(), vec![15, 31, 63, 127]);
    assert_eq!(parent_tree_indices(14).take(4).collect_vec(), vec![15, 31, 63, 127]);
}

#[test]
fn test_start_end_valid() {
    assert_eq!(start_end_valid(0, 0), false);
    assert_eq!(start_end_valid(0, 1), true);
    assert_eq!(start_end_valid(0, 2), true);
    assert_eq!(start_end_valid(0, 3), false);
    assert_eq!(start_end_valid(0, 4), true);
    assert_eq!(start_end_valid(0, 5), false);
    assert_eq!(start_end_valid(0, 6), false);
    assert_eq!(start_end_valid(0, 7), false);
    assert_eq!(start_end_valid(0, 8), true);

    assert_eq!(start_end_valid(1, 0), false);
    assert_eq!(start_end_valid(1, 1), false);
    assert_eq!(start_end_valid(1, 2), false);
    assert_eq!(start_end_valid(1, 3), false);
    assert_eq!(start_end_valid(1, 4), false);
    assert_eq!(start_end_valid(1, 5), false);
    assert_eq!(start_end_valid(1, 6), false);
    assert_eq!(start_end_valid(1, 7), false);
    assert_eq!(start_end_valid(1, 8), false);

    assert_eq!(start_end_valid(2, 0), false);
    assert_eq!(start_end_valid(2, 1), false);
    assert_eq!(start_end_valid(2, 2), false);
    assert_eq!(start_end_valid(2, 3), true);
    assert_eq!(start_end_valid(2, 4), false);
    assert_eq!(start_end_valid(2, 5), false);
    assert_eq!(start_end_valid(2, 6), false);
    assert_eq!(start_end_valid(2, 7), false);
    assert_eq!(start_end_valid(2, 8), false);

    assert_eq!(start_end_valid(3, 0), false);
    assert_eq!(start_end_valid(3, 1), false);
    assert_eq!(start_end_valid(3, 2), false);
    assert_eq!(start_end_valid(3, 3), false);
    assert_eq!(start_end_valid(3, 4), false);
    assert_eq!(start_end_valid(3, 5), false);
    assert_eq!(start_end_valid(3, 6), false);
    assert_eq!(start_end_valid(3, 7), false);
    assert_eq!(start_end_valid(3, 8), false);

    assert_eq!(start_end_valid(4, 0), false);
    assert_eq!(start_end_valid(4, 1), false);
    assert_eq!(start_end_valid(4, 2), false);
    assert_eq!(start_end_valid(4, 3), false);
    assert_eq!(start_end_valid(4, 4), false);
    assert_eq!(start_end_valid(4, 5), true);
    assert_eq!(start_end_valid(4, 6), true);
    assert_eq!(start_end_valid(4, 7), false);
    assert_eq!(start_end_valid(4, 8), false);

    assert_eq!(start_end_valid(5, 0), false);
    assert_eq!(start_end_valid(5, 1), false);
    assert_eq!(start_end_valid(5, 2), false);
    assert_eq!(start_end_valid(5, 3), false);
    assert_eq!(start_end_valid(5, 4), false);
    assert_eq!(start_end_valid(5, 5), false);
    assert_eq!(start_end_valid(5, 6), false);
    assert_eq!(start_end_valid(5, 7), false);
    assert_eq!(start_end_valid(5, 8), false);

    assert_eq!(start_end_valid(6, 0), false);
    assert_eq!(start_end_valid(6, 1), false);
    assert_eq!(start_end_valid(6, 2), false);
    assert_eq!(start_end_valid(6, 3), false);
    assert_eq!(start_end_valid(6, 4), false);
    assert_eq!(start_end_valid(6, 5), false);
    assert_eq!(start_end_valid(6, 6), false);
    assert_eq!(start_end_valid(6, 7), true);
    assert_eq!(start_end_valid(6, 8), false);

    assert_eq!(start_end_valid(7, 0), false);
    assert_eq!(start_end_valid(7, 1), false);
    assert_eq!(start_end_valid(7, 2), false);
    assert_eq!(start_end_valid(7, 3), false);
    assert_eq!(start_end_valid(7, 4), false);
    assert_eq!(start_end_valid(7, 5), false);
    assert_eq!(start_end_valid(7, 6), false);
    assert_eq!(start_end_valid(7, 7), false);
    assert_eq!(start_end_valid(7, 8), false);

    assert_eq!(start_end_valid(8, 0), false);
    assert_eq!(start_end_valid(8, 1), false);
    assert_eq!(start_end_valid(8, 2), false);
    assert_eq!(start_end_valid(8, 3), false);
    assert_eq!(start_end_valid(8, 4), false);
    assert_eq!(start_end_valid(8, 5), false);
    assert_eq!(start_end_valid(8, 6), false);
    assert_eq!(start_end_valid(8, 7), false);
    assert_eq!(start_end_valid(8, 8), false);
}

#[test]
fn test_max_level_for_start() {
    assert_eq!(max_level_for_start(0), Some(usize::BITS - 1));
    assert_eq!(max_level_for_start(1), None);
    assert_eq!(max_level_for_start(2), Some(0));
    assert_eq!(max_level_for_start(3), None);
    assert_eq!(max_level_for_start(4), Some(1));
    assert_eq!(max_level_for_start(5), None);
    assert_eq!(max_level_for_start(6), Some(0));
    assert_eq!(max_level_for_start(7), None);
    assert_eq!(max_level_for_start(8), Some(2));
    assert_eq!(max_level_for_start(9), None);
    assert_eq!(max_level_for_start(10), Some(0));
    assert_eq!(max_level_for_start(11), None);
    assert_eq!(max_level_for_start(12), Some(1));
    assert_eq!(max_level_for_start(13), None);
    assert_eq!(max_level_for_start(14), Some(0));
    assert_eq!(max_level_for_start(15), None);
}

#[test]
fn test_max_level_for_len() {
    assert_eq!(max_level_for_len(1), 0);
    assert_eq!(max_level_for_len(2), 1);
    assert_eq!(max_level_for_len(3), 1);
    assert_eq!(max_level_for_len(4), 2);
    assert_eq!(max_level_for_len(5), 2);
    assert_eq!(max_level_for_len(6), 2);
    assert_eq!(max_level_for_len(7), 2);
    assert_eq!(max_level_for_len(8), 3);
    assert_eq!(max_level_for_len(9), 3);
    assert_eq!(max_level_for_len(10), 3);
    assert_eq!(max_level_for_len(11), 3);
    assert_eq!(max_level_for_len(12), 3);
    assert_eq!(max_level_for_len(13), 3);
    assert_eq!(max_level_for_len(14), 3);
    assert_eq!(max_level_for_len(15), 3);
    assert_eq!(max_level_for_len(16), 4);
}

#[test]
fn test_tree_index_least_common_ancestor() {
    assert_eq!(tree_index_least_common_ancestor(0, 0), 0);
    assert_eq!(tree_index_least_common_ancestor(0, 1), 1);
    assert_eq!(tree_index_least_common_ancestor(0, 2), 3);
    assert_eq!(tree_index_least_common_ancestor(0, 3), 3);
    assert_eq!(tree_index_least_common_ancestor(0, 4), 7);
    assert_eq!(tree_index_least_common_ancestor(0, 5), 7);
    assert_eq!(tree_index_least_common_ancestor(0, 6), 7);
    assert_eq!(tree_index_least_common_ancestor(0, 7), 7);

    assert_eq!(tree_index_least_common_ancestor(1, 0), 1);
    assert_eq!(tree_index_least_common_ancestor(1, 1), 1);
    assert_eq!(tree_index_least_common_ancestor(1, 2), 3);
    assert_eq!(tree_index_least_common_ancestor(1, 3), 3);
    assert_eq!(tree_index_least_common_ancestor(1, 4), 7);
    assert_eq!(tree_index_least_common_ancestor(1, 5), 7);
    assert_eq!(tree_index_least_common_ancestor(1, 6), 7);
    assert_eq!(tree_index_least_common_ancestor(1, 7), 7);

    assert_eq!(tree_index_least_common_ancestor(2, 0), 3);
    assert_eq!(tree_index_least_common_ancestor(2, 1), 3);
    assert_eq!(tree_index_least_common_ancestor(2, 2), 2);
    assert_eq!(tree_index_least_common_ancestor(2, 3), 3);
    assert_eq!(tree_index_least_common_ancestor(2, 4), 7);
    assert_eq!(tree_index_least_common_ancestor(2, 5), 7);
    assert_eq!(tree_index_least_common_ancestor(2, 6), 7);
    assert_eq!(tree_index_least_common_ancestor(2, 7), 7);

    assert_eq!(tree_index_least_common_ancestor(3, 0), 3);
    assert_eq!(tree_index_least_common_ancestor(3, 1), 3);
    assert_eq!(tree_index_least_common_ancestor(3, 2), 3);
    assert_eq!(tree_index_least_common_ancestor(3, 3), 3);
    assert_eq!(tree_index_least_common_ancestor(3, 4), 7);
    assert_eq!(tree_index_least_common_ancestor(3, 5), 7);
    assert_eq!(tree_index_least_common_ancestor(3, 6), 7);
    assert_eq!(tree_index_least_common_ancestor(3, 7), 7);

    assert_eq!(tree_index_least_common_ancestor(4, 0), 7);
    assert_eq!(tree_index_least_common_ancestor(4, 1), 7);
    assert_eq!(tree_index_least_common_ancestor(4, 2), 7);
    assert_eq!(tree_index_least_common_ancestor(4, 3), 7);
    assert_eq!(tree_index_least_common_ancestor(4, 4), 4);
    assert_eq!(tree_index_least_common_ancestor(4, 5), 5);
    assert_eq!(tree_index_least_common_ancestor(4, 6), 7);
    assert_eq!(tree_index_least_common_ancestor(4, 7), 7);

    assert_eq!(tree_index_least_common_ancestor(5, 0), 7);
    assert_eq!(tree_index_least_common_ancestor(5, 1), 7);
    assert_eq!(tree_index_least_common_ancestor(5, 2), 7);
    assert_eq!(tree_index_least_common_ancestor(5, 3), 7);
    assert_eq!(tree_index_least_common_ancestor(5, 4), 5);
    assert_eq!(tree_index_least_common_ancestor(5, 5), 5);
    assert_eq!(tree_index_least_common_ancestor(5, 6), 7);
    assert_eq!(tree_index_least_common_ancestor(5, 7), 7);

    assert_eq!(tree_index_least_common_ancestor(6, 0), 7);
    assert_eq!(tree_index_least_common_ancestor(6, 1), 7);
    assert_eq!(tree_index_least_common_ancestor(6, 2), 7);
    assert_eq!(tree_index_least_common_ancestor(6, 3), 7);
    assert_eq!(tree_index_least_common_ancestor(6, 4), 7);
    assert_eq!(tree_index_least_common_ancestor(6, 5), 7);
    assert_eq!(tree_index_least_common_ancestor(6, 6), 6);
    assert_eq!(tree_index_least_common_ancestor(6, 7), 7);

    assert_eq!(tree_index_least_common_ancestor(7, 0), 7);
    assert_eq!(tree_index_least_common_ancestor(7, 1), 7);
    assert_eq!(tree_index_least_common_ancestor(7, 2), 7);
    assert_eq!(tree_index_least_common_ancestor(7, 3), 7);
    assert_eq!(tree_index_least_common_ancestor(7, 4), 7);
    assert_eq!(tree_index_least_common_ancestor(7, 5), 7);
    assert_eq!(tree_index_least_common_ancestor(7, 6), 7);
    assert_eq!(tree_index_least_common_ancestor(7, 7), 7);
}

#[test]
fn test_spans_for_range() {
    assert_eq!(spans_for_range(0, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(0, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(0, 1).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 1).1.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );

    assert_eq!(spans_for_range(0, 2).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 2).1.collect_vec(),
        vec![Span::from_start_end(0, 2)],
    );

    assert_eq!(spans_for_range(0, 3).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 3).1.collect_vec(),
        vec![Span::from_start_end(0, 2), Span::from_start_end(2, 3)],
    );

    assert_eq!(spans_for_range(0, 4).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 4).1.collect_vec(),
        vec![Span::from_start_end(0, 4)],
    );

    assert_eq!(spans_for_range(0, 5).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 5).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 5)],
    );

    assert_eq!(spans_for_range(0, 6).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 6).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 6)],
    );

    assert_eq!(spans_for_range(0, 7).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 7).1.collect_vec(),
        vec![
            Span::from_start_end(0, 4),
            Span::from_start_end(4, 6),
            Span::from_start_end(6, 7),
        ],
    );

    assert_eq!(spans_for_range(0, 8).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(0, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );

    // ---

    assert_eq!(spans_for_range(1, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(1, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(1, 1).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(1, 1).1.collect_vec(), vec![]);

    assert_eq!(
        spans_for_range(1, 2).0.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );
    assert_eq!(
        spans_for_range(1, 2).1.collect_vec(),
        vec![Span::from_start_end(0, 2)],
    );

    assert_eq!(
        spans_for_range(1, 3).0.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );
    assert_eq!(
        spans_for_range(1, 3).1.collect_vec(),
        vec![Span::from_start_end(0, 2), Span::from_start_end(2, 3)],
    );

    assert_eq!(
        spans_for_range(1, 4).0.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );
    assert_eq!(
        spans_for_range(1, 4).1.collect_vec(),
        vec![Span::from_start_end(0, 4)],
    );

    assert_eq!(
        spans_for_range(1, 5).0.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );
    assert_eq!(
        spans_for_range(1, 5).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 5)],
    );

    assert_eq!(
        spans_for_range(1, 6).0.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );
    assert_eq!(
        spans_for_range(1, 6).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 6)],
    );

    assert_eq!(
        spans_for_range(1, 7).0.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );
    assert_eq!(
        spans_for_range(1, 7).1.collect_vec(),
        vec![
            Span::from_start_end(0, 4),
            Span::from_start_end(4, 6),
            Span::from_start_end(6, 7),
        ],
    );

    assert_eq!(
        spans_for_range(1, 8).0.collect_vec(),
        vec![Span::from_start_end(0, 1)],
    );
    assert_eq!(
        spans_for_range(1, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );

    // ---

    assert_eq!(spans_for_range(2, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(2, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(2, 1).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(2, 1).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(2, 2).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(2, 2).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(2, 3).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(2, 3).1.collect_vec(),
        vec![Span::from_start_end(2, 3)],
    );

    assert_eq!(
        spans_for_range(2, 4).0.collect_vec(),
        vec![Span::from_start_end(0, 2)],
    );
    assert_eq!(
        spans_for_range(2, 4).1.collect_vec(),
        vec![Span::from_start_end(0, 4)],
    );

    assert_eq!(
        spans_for_range(2, 5).0.collect_vec(),
        vec![Span::from_start_end(0, 2)],
    );
    assert_eq!(
        spans_for_range(2, 5).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 5)],
    );

    assert_eq!(
        spans_for_range(2, 6).0.collect_vec(),
        vec![Span::from_start_end(0, 2)],
    );
    assert_eq!(
        spans_for_range(2, 6).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 6)],
    );

    assert_eq!(
        spans_for_range(2, 7).0.collect_vec(),
        vec![Span::from_start_end(0, 2)],
    );
    assert_eq!(
        spans_for_range(2, 7).1.collect_vec(),
        vec![
            Span::from_start_end(0, 4),
            Span::from_start_end(4, 6),
            Span::from_start_end(6, 7),
        ],
    );

    assert_eq!(
        spans_for_range(2, 8).0.collect_vec(),
        vec![Span::from_start_end(0, 2)],
    );
    assert_eq!(
        spans_for_range(2, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );

    // ---

    assert_eq!(spans_for_range(3, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(3, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(3, 1).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(3, 1).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(3, 2).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(3, 2).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(3, 3).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(3, 3).1.collect_vec(), vec![]);

    assert_eq!(
        spans_for_range(3, 4).0.collect_vec(),
        vec![Span::from_start_end(0, 2), Span::from_start_end(2, 3)],
    );
    assert_eq!(
        spans_for_range(3, 4).1.collect_vec(),
        vec![Span::from_start_end(0, 4)],
    );

    assert_eq!(
        spans_for_range(3, 5).0.collect_vec(),
        vec![Span::from_start_end(0, 2), Span::from_start_end(2, 3)],
    );
    assert_eq!(
        spans_for_range(3, 5).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 5)],
    );

    assert_eq!(
        spans_for_range(3, 6).0.collect_vec(),
        vec![Span::from_start_end(0, 2), Span::from_start_end(2, 3)],
    );
    assert_eq!(
        spans_for_range(3, 6).1.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 6)],
    );

    assert_eq!(
        spans_for_range(3, 7).0.collect_vec(),
        vec![Span::from_start_end(0, 2), Span::from_start_end(2, 3)],
    );
    assert_eq!(
        spans_for_range(3, 7).1.collect_vec(),
        vec![
            Span::from_start_end(0, 4),
            Span::from_start_end(4, 6),
            Span::from_start_end(6, 7),
        ],
    );

    assert_eq!(
        spans_for_range(3, 8).0.collect_vec(),
        vec![Span::from_start_end(0, 2), Span::from_start_end(2, 3)],
    );
    assert_eq!(
        spans_for_range(3, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );

    // ---

    assert_eq!(spans_for_range(4, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(4, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(4, 1).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(4, 1).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(4, 2).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(4, 2).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(4, 3).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(4, 3).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(4, 4).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(4, 4).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(4, 5).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(4, 5).1.collect_vec(),
        vec![Span::from_start_end(4, 5)],
    );

    assert_eq!(spans_for_range(4, 6).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(4, 6).1.collect_vec(),
        vec![Span::from_start_end(4, 6)],
    );

    assert_eq!(spans_for_range(4, 7).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(4, 7).1.collect_vec(),
        vec![Span::from_start_end(4, 6), Span::from_start_end(6, 7)],
    );

    assert_eq!(
        spans_for_range(4, 8).0.collect_vec(),
        vec![Span::from_start_end(0, 4)],
    );
    assert_eq!(
        spans_for_range(4, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );

    // ---

    assert_eq!(spans_for_range(5, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(5, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(5, 1).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(5, 1).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(5, 2).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(5, 2).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(5, 3).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(5, 3).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(5, 4).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(5, 4).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(5, 5).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(5, 5).1.collect_vec(), vec![]);

    assert_eq!(
        spans_for_range(5, 6).0.collect_vec(),
        vec![Span::from_start_end(4, 5)],
    );
    assert_eq!(
        spans_for_range(5, 6).1.collect_vec(),
        vec![Span::from_start_end(4, 6)],
    );

    assert_eq!(
        spans_for_range(5, 7).0.collect_vec(),
        vec![Span::from_start_end(4, 5)],
    );
    assert_eq!(
        spans_for_range(5, 7).1.collect_vec(),
        vec![Span::from_start_end(4, 6), Span::from_start_end(6, 7)],
    );

    assert_eq!(
        spans_for_range(5, 8).0.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 5)],
    );
    assert_eq!(
        spans_for_range(5, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );

    // ---

    assert_eq!(spans_for_range(6, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(6, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(6, 1).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(6, 1).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(6, 2).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(6, 2).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(6, 3).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(6, 3).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(6, 4).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(6, 4).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(6, 5).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(6, 5).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(6, 6).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(6, 6).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(6, 7).0.collect_vec(), vec![]);
    assert_eq!(
        spans_for_range(6, 7).1.collect_vec(),
        vec![Span::from_start_end(6, 7)],
    );

    assert_eq!(
        spans_for_range(6, 8).0.collect_vec(),
        vec![Span::from_start_end(0, 4), Span::from_start_end(4, 6)],
    );
    assert_eq!(
        spans_for_range(6, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );

    // ---

    assert_eq!(spans_for_range(7, 0).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 0).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(7, 1).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 1).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(7, 2).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 2).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(7, 3).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 3).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(7, 4).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 4).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(7, 5).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 5).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(7, 6).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 6).1.collect_vec(), vec![]);

    assert_eq!(spans_for_range(7, 7).0.collect_vec(), vec![]);
    assert_eq!(spans_for_range(7, 7).1.collect_vec(), vec![]);

    assert_eq!(
        spans_for_range(7, 8).0.collect_vec(),
        vec![
            Span::from_start_end(0, 4),
            Span::from_start_end(4, 6),
            Span::from_start_end(6, 7),
        ],
    );
    assert_eq!(
        spans_for_range(7, 8).1.collect_vec(),
        vec![Span::from_start_end(0, 8)],
    );
}

#[test]
fn test_span_from_tree_index() {
    assert_eq!(Span::from_tree_index(0), Span::from_start_len(0, 1));
    assert_eq!(Span::from_tree_index(1), Span::from_start_len(0, 2));
    assert_eq!(Span::from_tree_index(2), Span::from_start_len(2, 1));
    assert_eq!(Span::from_tree_index(3), Span::from_start_len(0, 4));
    assert_eq!(Span::from_tree_index(4), Span::from_start_len(4, 1));
    assert_eq!(Span::from_tree_index(5), Span::from_start_len(4, 2));
    assert_eq!(Span::from_tree_index(6), Span::from_start_len(6, 1));
    assert_eq!(Span::from_tree_index(7), Span::from_start_len(0, 8));
}

#[test]
fn test_span_tree_index() {
    assert_eq!(Span::from_start_len(0, 1).tree_index(), 0);
    assert_eq!(Span::from_start_len(0, 2).tree_index(), 1);
    assert_eq!(Span::from_start_len(2, 1).tree_index(), 2);
    assert_eq!(Span::from_start_len(0, 4).tree_index(), 3);
    assert_eq!(Span::from_start_len(4, 1).tree_index(), 4);
    assert_eq!(Span::from_start_len(4, 2).tree_index(), 5);
    assert_eq!(Span::from_start_len(6, 1).tree_index(), 6);
    assert_eq!(Span::from_start_len(0, 8).tree_index(), 7);
}

#[test]
fn test_span_parent() {
    assert_eq!(
        Span::from_start_len(0, 1).parent(),
        Span::from_start_len(0, 2),
    );
    assert_eq!(
        Span::from_start_len(0, 2).parent(),
        Span::from_start_len(0, 4),
    );
    assert_eq!(
        Span::from_start_len(2, 1).parent(),
        Span::from_start_len(0, 4),
    );
    assert_eq!(
        Span::from_start_len(0, 4).parent(),
        Span::from_start_len(0, 8),
    );
    assert_eq!(
        Span::from_start_len(4, 1).parent(),
        Span::from_start_len(4, 2),
    );
    assert_eq!(
        Span::from_start_len(4, 2).parent(),
        Span::from_start_len(0, 8),
    );
    assert_eq!(
        Span::from_start_len(6, 1).parent(),
        Span::from_start_len(0, 8),
    );
}

#[test]
fn test_span_children() {
    assert_eq!(Span::from_start_len(0, 1).children().collect_vec(), vec![],);
    assert_eq!(
        Span::from_start_len(0, 2).children().collect_vec(),
        vec![Span::from_start_len(0, 1)],
    );
    assert_eq!(Span::from_start_len(2, 1).children().collect_vec(), vec![],);
    assert_eq!(
        Span::from_start_len(0, 4).children().collect_vec(),
        vec![Span::from_start_len(0, 2), Span::from_start_len(2, 1)],
    );
    assert_eq!(Span::from_start_len(4, 1).children().collect_vec(), vec![],);
    assert_eq!(
        Span::from_start_len(4, 2).children().collect_vec(),
        vec![Span::from_start_len(4, 1)],
    );
    assert_eq!(Span::from_start_len(5, 1).children().collect_vec(), vec![],);
    assert_eq!(
        Span::from_start_len(0, 8).children().collect_vec(),
        vec![
            Span::from_start_len(0, 4),
            Span::from_start_len(4, 2),
            Span::from_start_len(6, 1),
        ],
    );
}
