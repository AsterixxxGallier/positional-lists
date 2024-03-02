// for Position
#![feature(trait_alias)]
// for debug_assert_matches
#![feature(assert_matches)]

#![allow(dead_code)]

mod trait_aliases;
mod point_list;
mod frame;
mod index;

#[doc(inline)]
pub use {
    trait_aliases::Element,
    trait_aliases::Position,
    point_list::PointList,
};

#[allow(unused_imports)]
pub(crate) use {
    frame::Frame,
    frame::FrameKey,
    frame::Slot,
    index::EphemeralIndex,
    index::PersistentIndex,
};
