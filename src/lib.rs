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

#[doc(inline)]
pub use {
    point_list::PointList,
    point_list::PointKey,
    trait_aliases::Element,
    trait_aliases::Position,
};

#[allow(unused_imports)]
pub(crate) use {
    frame::Frame,
    frame::EitherFrame,
    frame::meta::MetaFrame,
    frame::base::BaseFrame,
    frame::distances::Distances,
    frame::Embedding,
    frame::FrameKey,
    frame::IndexInFrame,

    frame::FRAME_CAPACITY,
    frame::distances::DISTANCES_DEPTH,
    frame::distances::DISTANCES_CAPACITY,
};
