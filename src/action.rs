use std::cmp::Ordering;

pub use crate::ffi::ale::Action as ALEAction;

impl PartialOrd for ALEAction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&(*self as u32), &(*other as u32))
    }
}

impl Ord for ALEAction {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&(*self as u32), &(*other as u32))
    }
}

impl ToString for ALEAction {
    fn to_string(&self) -> String {
        unsafe {
            crate::ffi::ale::action_to_string(*self)
                .as_ref()
                .expect("Invalid pointer")
                .to_str()
                .expect("Invalid UTF-8 encoding")
                .to_string()
        }
    }
}
