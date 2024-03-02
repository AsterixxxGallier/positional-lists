// for Position
#![feature(trait_alias)]
// for debug_assert_matches
#![feature(assert_matches)]

#![allow(dead_code)]

mod position;
mod element;
mod frame;

#[doc(inline)]
pub use {
    position::Position,
    element::Element,
};

pub(crate) use {
    frame::Frame,
    frame::Slot,
};
