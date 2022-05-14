use ::std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
};

// Include bindings.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use root::*;

use crate::{
    ffi::ale::{action_to_string, byte_t, Action, ALERAM},
    utils::from_std_string,
};

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

impl ToString for Action {
    fn to_string(&self) -> String {
        unsafe { from_std_string(action_to_string(*self)) }
    }
}

/// Implement immutable index access to ALERAM.
impl Index<usize> for ALERAM {
    type Output = byte_t;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m_ram[index]
    }
}

/// Implement mutable index access to ALERAM.
impl IndexMut<usize> for ALERAM {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m_ram[index]
    }
}
