#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Rust binding for the [Arcade-Learning-Environment](https://github.com/mgbellemare/Arcade-Learning-Environment).

use autocxx::prelude::*;

include_cpp! {
    #include "ale_interface.hpp"
    generate!("ale::Action")
    generate!("ale::ALEInterface")
    generate!("ale::ALERAM")
    generate!("ale::ALEScreen")
    generate!("ale::ALEState")
    generate!("ale::action_to_string")
}

mod action;
pub use crate::action::ALEAction;

mod interface;
pub use crate::interface::ALEInterface;
