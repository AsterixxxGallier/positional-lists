use crate::span_tree::{level_to_len, max_level_for_len, max_level_for_start, Span};

pub(super) struct Iter {
    pub(super) start: usize,
    pub(super) end: usize,
}

impl Iterator for Iter {
    type Item = Span;

    fn next(&mut self) -> Option<Span> {
        let len = self.end - self.start;
        if len == 0 {
            return None;
        }

        let max_level_for_start = max_level_for_start(self.start).expect(
            "Iter::start should always be a valid start index (i.e. even) until iterator is empty",
        );
        let max_level_for_len = max_level_for_len(len);

        let level = max_level_for_start.min(max_level_for_len);

        let span = Span::from_start_level(self.start, level);

        self.start += level_to_len(level);

        Some(span)
    }
}
