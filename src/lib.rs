#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Rust binding for the [Arcade-Learning-Environment](https://github.com/mgbellemare/Arcade-Learning-Environment).

/// Re-export of the ALEInterface.
mod ale;
pub use ale::*;

/// Include bindings.
pub mod ffi;
/// C++ utils for Rust FFI.
pub mod utils;
