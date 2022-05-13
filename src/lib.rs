#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Rust binding for the [Arcade-Learning-Environment](https://github.com/mgbellemare/Arcade-Learning-Environment).

// Import C/C++ types.
use std::os::raw::c_char;

// Include bindings.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Map `&str` to `std::string`.
#[inline]
fn into_std_string(str: &str) -> root::std::string {
    // C++ `const char*` is mapped as Rust `*const c_char`.
    unsafe { root::std_string_from(str.as_ptr() as *const c_char, str.len()) }
}

// Map `std::string` to `&str`.
#[inline]
fn from_std_string<'a>(std_string: root::std::string) -> &'a str {
    unsafe {
        // Get raw pointer from `std::string`.
        let c_str = root::std_string_into(&std_string) as *const c_char;
        // Map raw pointer to associated struct.
        let c_str = std::ffi::CStr::from_ptr(c_str);
        // Check for valid UTF-8 encoding.
        c_str.to_str().expect("UTF-8 validation failed")
    }
}

pub struct ALEInterface {
    ffi: root::ale::ALEInterface,
}

impl ALEInterface {
    /// Constructs an ALEInterface struct.
    pub fn new() -> Self {
        // Initialize the ALEInterface FFI.
        let ffi = unsafe { root::ale::ALEInterface::new() };

        Self { ffi }
    }

    /// Constructs an ALEInterface struct with or without a display screen.
    pub fn with_display_screen(flag: bool) -> Self {
        // Initialize the ALEInterface FFI.
        let ffi = unsafe { root::ale::ALEInterface::new1(flag) };

        Self { ffi }
    }

    /// Gets boolean attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_bool(&mut self, key: &str) -> bool {
        unsafe { self.ffi.getBool(&into_std_string(key)) }
    }

    /// Gets integer attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_int(&mut self, key: &str) -> i32 {
        unsafe { self.ffi.getInt(&into_std_string(key)) }
    }

    /// Gets float attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_float(&mut self, key: &str) -> f32 {
        unsafe { self.ffi.getFloat(&into_std_string(key)) }
    }

    /// Gets string attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_str(&mut self, key: &str) -> &str {
        unsafe { from_std_string(self.ffi.getString(&into_std_string(key))) }
    }
}

impl Drop for ALEInterface {
    fn drop(&mut self) {
        unsafe { self.ffi.destruct() }
    }
}

impl Default for ALEInterface {
    fn default() -> Self {
        Self::new()
    }
}
