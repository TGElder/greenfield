use std::cmp::Ordering;

extern crate maplit;

pub mod almost;
pub mod color;
pub mod float_grid;
pub mod geometry;
pub mod grid;
pub mod image;
pub mod noise;
pub mod quadratic;
pub mod scale;

pub fn unsafe_float_ordering<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    a.partial_cmp(b).unwrap()
}
