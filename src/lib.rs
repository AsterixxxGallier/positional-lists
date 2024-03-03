// for Position
#![feature(trait_alias)]
// for debug_assert_matches
#![feature(assert_matches)]
// for "impl Debug for PointList"
#![feature(debug_closure_helpers)]

#![allow(dead_code)]

mod trait_aliases;
mod point_list;
mod frame;
mod index;

#[doc(inline)]
pub use {
    index::PersistentIndex,
    point_list::PointList,
    trait_aliases::Element,
    trait_aliases::Position,
};

#[allow(unused_imports)]
pub(crate) use {
    frame::Frame,
    frame::EitherFrame,
    frame::meta::MetaFrame,
    frame::element::ElementFrame,
    frame::distances::Distances,
    frame::Embedding,
    frame::FrameKey,

    index::EphemeralIndex,
};
