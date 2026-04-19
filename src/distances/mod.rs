use std::ops::{AddAssign, RangeBounds, SubAssign};
use num_traits::Zero;
use crate::span_tree::SpanTree;

#[derive(Default)]
pub(crate) struct Distances<T, const N: usize>(SpanTree<T, N>);

impl<T, const N: usize> Distances<T, N> where T: AddAssign + SubAssign + Zero + Clone {
    pub(crate) fn length(&self) -> T {
        self.0.get(self.0.root()).unwrap().clone()
    }

    pub(crate) fn increase_distance(&mut self, index: usize, change: T) {
        for span in self.0.spans_containing(index) {
            *self.0.get_mut(span).unwrap() += change.clone();
        }
    }

    pub(crate) fn decrease_distance(&mut self, index: usize, change: T) {
        for span in self.0.spans_containing(index) {
            *self.0.get_mut(span).unwrap() -= change.clone();
        }
    }

    pub(crate) fn distance_to_next(&self, index: usize) -> T {
        let span = self.0.smallest_span_containing(index);
        let mut distance = self.0.get(span).unwrap().clone();
        for child in self.0.children(span) {
            distance -= self.0.get(child).unwrap().clone();
        }
        distance
    }

    pub(crate) fn distance(&self, range: impl RangeBounds<usize>) -> T {
        let mut distance = T::zero();
        let (excluded_spans, included_spans) = self.0.spans_for_range(range);
        for span in included_spans {
            distance += self.0.get(span).unwrap().clone();
        }
        for span in excluded_spans {
            distance -= self.0.get(span).unwrap().clone();
        }
        distance
    }
}