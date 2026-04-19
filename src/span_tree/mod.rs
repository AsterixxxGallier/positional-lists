use std::collections::Bound;
use std::ops::RangeBounds;

mod spans_for_range;
#[cfg(test)]
mod test;

type Level = u32;

// N must be a power of two
pub(crate) struct SpanTree<T, const N: usize>([T; N]);

impl<T: Default, const N: usize> Default for SpanTree<T, N> {
    fn default() -> Self {
        assert!(N.is_power_of_two());
        Self([(); N].map(|()| T::default()))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct Span {
    start: usize,
    // exclusive
    end: usize,
}

fn level_to_len(level: Level) -> usize {
    1 << level
}

fn len_to_level(len: usize) -> Level {
    len.trailing_zeros()
}

fn child_offset(parent_level: Level, child_level: Level) -> usize {
    level_to_len(parent_level) - level_to_len(child_level + 1)
}

fn tree_index_to_end(tree_index: usize) -> usize {
    tree_index + 1
}

fn end_to_tree_index(end: usize) -> usize {
    end - 1
}

fn end_to_len(end: usize) -> usize {
    end.isolate_lowest_one()
}

fn tree_index_to_len(tree_index: usize) -> usize {
    end_to_len(tree_index_to_end(tree_index))
}

fn end_to_start(end: usize) -> usize {
    end - end_to_len(end)
}

fn parent_tree_index(tree_index: usize) -> usize {
    tree_index | (1 << tree_index.trailing_ones())
}

fn parent_tree_indices(tree_index: usize) -> impl Iterator<Item = usize> {
    (0..usize::BITS).filter_map(move |level| {
        if tree_index & (1 << level) == 0 {
            let mask = (1 << (level + 1)) - 1;
            Some(tree_index | mask)
        } else {
            None
        }
    })
}

fn start_end_valid(start: usize, end: usize) -> bool {
    start < end && start == end_to_start(end)
}

fn max_level_for_start(start: usize) -> Option<Level> {
    start.trailing_zeros().checked_sub(1)
}

fn max_level_for_len(len: usize) -> Level {
    len.isolate_highest_one().trailing_zeros()
}

/*
_______________________________
_______________
_______         _______
___     ___     ___     ___
_   _   _   _   _   _   _   _
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5

 */

/// "Ancestor" is defined reflexively for the purposes of this function.
///
/// So, if `a` and `b` are equal, the same value will be returned.
/// If `a` is an ancestor of `b`, then `a` will be returned, and vice versa.
fn tree_index_least_common_ancestor(a: usize, b: usize) -> usize {
    // For tree indices, the parent is given by flipping the least significant zero.
    // As such, an iterator over all ancestors of a span can be created by subsequently filling up
    // the tree index with ones, starting from the right.
    // It may be that one of the tree index's bits was already a one; this is not a problem.
    // Such an already-set bit indicates that the span has no ancestor at the level given by the
    // index of this bit plus one.
    //
    // The least common ancestor of two spans A and B is the common ancestor of A and B with the
    // lowest level; equivalently, it is the ancestor with the numerically smallest tree index.
    // Going with the "common ancestor of lowest level" approach, one can imagine filling up both
    // A and B with ones in lockstep, again starting from the right.
    // For both A and B, this iterates over their ancestors.
    // Once A and B, gradually filling up with ones from the right, are equal, this common value
    // will be the tree index of the least common ancestor. Filled-up A and B being equal means
    // that the value is an ancestor both of A and of B. The process of filling up step by step
    // guarantees that this common ancestor is the one with the lowest level.
    //
    // Another way to look at it is through the lens of "erasing differences". We want to end up
    // with filled-up A and B being equal. We can take the XOR of A and B to get a mask showing us
    // exactly where they are not yet equal. Then, we can simply fill up both A and B with ones
    // right until the highest set bit in their XOR. Thus, we will have properly erased their
    // differences, while adhering to the principal algorithm described in the previous paragraph.
    let differences = a ^ b;
    if differences == 0 {
        a
    } else {
        let mask = (differences.isolate_highest_one() << 1).wrapping_sub(1);
        // same as b | mask
        a | mask
    }
}

fn tree_index_least_greater_sibling(a: usize, b: usize) -> Option<usize> {
    let differences = a ^ b;
    if differences == 0 {
        a.checked_sub(tree_index_to_len(a))
    } else {
        let mask = !(differences.isolate_highest_one() << 1).wrapping_sub(1);
        // a & mask == b & mask
        (a & mask).checked_sub(1)
    }
}

fn spans_for_range(start: usize, end: usize) -> (spans_for_range::Iter, spans_for_range::Iter) {
    if end <= start {
        (
            spans_for_range::Iter { start: 0, end: 0 },
            spans_for_range::Iter { start: 0, end: 0 },
        )
    } else {
        let common_start = Span::from_tree_index(start)
            .least_greater_sibling(Span::from_tree_index(end))
            .map_or(0, |span| span.end);

        (
            spans_for_range::Iter {
                start: common_start,
                end: start,
            },
            spans_for_range::Iter {
                start: common_start,
                end,
            },
        )
    }
}

impl Span {
    fn from_start_end(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn from_start_len(start: usize, len: usize) -> Self {
        Self::from_start_end(start, start + len)
    }

    fn from_start_level(start: usize, level: Level) -> Self {
        Self::from_start_len(start, level_to_len(level))
    }

    fn from_end(end: usize) -> Self {
        Self::from_start_end(end_to_start(end), end)
    }

    fn from_tree_index(tree_index: usize) -> Self {
        Self::from_end(tree_index + 1)
    }

    pub(crate) fn start(self) -> usize {
        self.start
    }

    pub(crate) fn end(self) -> usize {
        self.end
    }

    // always a power of two
    pub(crate) fn len(self) -> usize {
        self.end - self.start
    }

    pub(crate) fn contains_index(self, index: usize) -> bool {
        self.start <= index && index < self.end
    }

    pub(crate) fn contains_span(self, span: Span) -> bool {
        self.start <= span.start && span.end < self.end
    }

    fn level(self) -> Level {
        len_to_level(self.len())
    }

    fn parent(self) -> Self {
        Self::from_tree_index(parent_tree_index(self.tree_index()))
    }

    fn parents(self) -> impl Iterator<Item = Self> {
        parent_tree_indices(self.tree_index()).map(Self::from_tree_index)
    }

    fn parents_with_self(self) -> impl Iterator<Item = Self> {
        [self]
            .into_iter()
            .chain(parent_tree_indices(self.tree_index()).map(Self::from_tree_index))
    }

    fn children(self) -> impl Iterator<Item = Self> {
        (0..self.level()).rev().map(move |child_level| {
            Self::from_start_level(
                self.start() + child_offset(self.level(), child_level),
                child_level,
            )
        })
    }

    fn least_common_ancestor(self, other: Self) -> Self {
        Self::from_tree_index(tree_index_least_common_ancestor(
            self.tree_index(),
            other.tree_index(),
        ))
    }

    fn least_greater_sibling(self, other: Self) -> Option<Self> {
        tree_index_least_greater_sibling(self.tree_index(), other.tree_index())
            .map(Self::from_tree_index)
    }

    fn tree_index(self) -> usize {
        end_to_tree_index(self.end)
    }
}

impl<T, const N: usize> SpanTree<T, N> {
    fn root_level(&self) -> Level {
        len_to_level(N)
    }

    pub(crate) fn root(&self) -> Span {
        Span::from_start_len(0, N)
    }

    pub(crate) fn span_from_start_end(&self, start: usize, end: usize) -> Option<Span> {
        if start_end_valid(start, end) && end <= N {
            Some(Span::from_start_end(start, end))
        } else {
            None
        }
    }

    pub(crate) fn span_from_start_len(&self, start: usize, len: usize) -> Option<Span> {
        self.span_from_start_end(start, start + len)
    }

    pub(crate) fn smallest_span_containing(&self, index: usize) -> Span {
        self.check_index_bounds(index);
        Span::from_tree_index(index)
    }

    pub(crate) fn spans_containing(&self, index: usize) -> impl Iterator<Item = Span> {
        self.smallest_span_containing(index)
            .parents_with_self()
            .take_while(|span| span.end <= N)
    }

    pub(crate) fn parent(&self, span: Span) -> Option<Span> {
        if span.level() == self.root_level() {
            None
        } else {
            Some(span.parent())
        }
    }

    pub(crate) fn parents(&self, span: Span) -> impl Iterator<Item = Span> {
        span.parents().take_while(|parent| parent.end <= N)
    }

    pub(crate) fn children(&self, span: Span) -> impl Iterator<Item = Span> {
        span.children()
    }

    fn range_to_bounds(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let start = match range.start_bound() {
            Bound::Included(&index) => index,
            Bound::Excluded(&index) => index + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&index) => index + 1,
            Bound::Excluded(&index) => index,
            Bound::Unbounded => N,
        };
        (start, end)
    }

    fn check_range_bounds(&self, start: usize, end: usize) {
        if start >= N {
            panic!("start of range out of bounds (is {start}, should be less than {N})");
        }

        if end > N {
            panic!("end of range out of bounds (is {end}, should be less than or equal to {N})");
        }
    }

    fn check_index_bounds(&self, index: usize) {
        if index > N {
            panic!("index out of bounds (is {index}, should be less than {N})");
        }
    }

    pub(crate) fn spans_for_range(
        &self,
        range: impl RangeBounds<usize>,
    ) -> (impl Iterator<Item = Span>, impl Iterator<Item = Span>) {
        let (start, end) = self.range_to_bounds(range);

        self.check_range_bounds(start, end);

        spans_for_range(start, end)
    }

    pub(crate) fn get(&self, span: Span) -> Option<&T> {
        self.0.get(span.tree_index())
    }

    pub(crate) fn get_mut(&mut self, span: Span) -> Option<&mut T> {
        self.0.get_mut(span.tree_index())
    }
}
