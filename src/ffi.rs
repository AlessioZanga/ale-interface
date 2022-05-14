use ::std::cmp::Ordering;

// Include bindings.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use root::*;

use crate::ffi::ale::Action;

// Fix missing `PartialOrd` derive on enum.
impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&(*self as u32), &(*other as u32))
    }
}

// Fix missing `Ord` derive on enum.
impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&(*self as u32), &(*other as u32))
    }
}
