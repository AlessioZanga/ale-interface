#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! Rust binding for the [Arcade-Learning-Environment](https://github.com/mgbellemare/Arcade-Learning-Environment).

/// Include bindings.
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub use root::*;
}

/// C++ utils for Rust FFI.
pub mod utils {
    // Import C/C++ types.
    use super::ffi::{std::string as std_string, utils::std_string_from, utils::std_string_into};
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// Map `&str` to `std::string`.
    #[inline]
    pub fn into_std_string(str: &str) -> std_string {
        // C++ `const char*` is mapped as Rust `*const c_char`.
        unsafe { std_string_from(str.as_ptr() as *const c_char, str.len()) }
    }

    /// Map `std::string` to `&str`.
    #[inline]
    pub fn from_std_string<'a>(std_string: std_string) -> &'a str {
        unsafe {
            // Get raw pointer from `std::string`.
            let c_str = std_string_into(&std_string) as *const c_char;
            // Map raw pointer to associated struct.
            let c_str = CStr::from_ptr(c_str);
            // Check for valid UTF-8 encoding.
            c_str.to_str().expect("UTF-8 validation failed")
        }
    }
}

pub struct ALEInterface {
    ffi: ffi::ale::ALEInterface,
}

impl ALEInterface {
    /// Constructs an ALEInterface struct.
    pub fn new() -> Self {
        // Initialize the ALEInterface FFI.
        let ffi = unsafe { ffi::ale::ALEInterface::new() };

        Self { ffi }
    }

    /// Constructs an ALEInterface struct with or without a display screen. Alias of `new1`.
    pub fn with_display_screen(flag: bool) -> Self {
        // Initialize the ALEInterface FFI.
        let ffi = unsafe { ffi::ale::ALEInterface::new1(flag) };

        Self { ffi }
    }

    // TODO: act
    // TODO: cloneState
    // TODO: cloneSystemState
    // TODO: createOSystem
    // TODO: createScreenExporter
    // TODO: disableBufferedIO
    // TODO: game_over
    // TODO: getAvailableDifficulties
    // TODO: getAvailableModes

    /// Gets boolean attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_bool(&mut self, key: &str) -> bool {
        unsafe { self.ffi.getBool(&utils::into_std_string(key)) }
    }

    // TODO: getEpisodeFrameNumber

    /// Gets float attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_float(&mut self, key: &str) -> f32 {
        unsafe { self.ffi.getFloat(&utils::into_std_string(key)) }
    }

    // TODO: getFrameNumber

    /// Gets integer attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_int(&mut self, key: &str) -> i32 {
        unsafe { self.ffi.getInt(&utils::into_std_string(key)) }
    }

    // TODO: getLegalActionSet
    // TODO: getMinimalActionSet
    // TODO: getRAM
    // TODO: getScreen
    // TODO: getScreenGrayscale
    // TODO: getScreenRGB

    /// Gets string attribute given key.
    // FIXME: Make self reference immutable.
    pub fn get_str(&mut self, key: &str) -> &str {
        unsafe { utils::from_std_string(self.ffi.getString(&utils::into_std_string(key))) }
    }

    // TODO: getStringInplace
    // TODO: isSupportedROM
    // TODO: lives
    // TODO: loadROM
    // TODO: loadSettings
    // TODO: reset_game
    // TODO: restoreState
    // TODO: restoreSystemState
    // TODO: saveScreenPNG
    // TODO: setBool
    // TODO: setDifficulty
    // TODO: setFloat
    // TODO: setInt
    // TODO: setMode
    // TODO: setRAM
    // TODO: setString
    // TODO: welcomeMessage
}

impl Drop for ALEInterface {
    // Alias of `destruct`.
    fn drop(&mut self) {
        unsafe { self.ffi.destruct() }
    }
}

impl Default for ALEInterface {
    fn default() -> Self {
        Self::new()
    }
}
