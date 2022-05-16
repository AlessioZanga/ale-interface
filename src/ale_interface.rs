use std::{mem::MaybeUninit, pin::Pin};

use autocxx::prelude::New;

use crate::ffi;

/// Rust FFI binding for the ALEInterface class.
pub struct ALEInterface {
    ffi: ffi::ale::ALEInterface,
}

impl ALEInterface {
    /// [alias: new] Constructor of the ALEInterface struct.
    pub fn new() -> Self {
        unsafe {
            use ffi::ale::ALEInterface;
            // Allocate memory for ALEInterface FFI.
            let mut ffi = MaybeUninit::<ALEInterface>::uninit();
            // Write a new ALEInterface FFI into the memory location.
            ALEInterface::new().new(Pin::new_unchecked(&mut ffi));
            // Get the ALEInterface FFI from the memory location.
            let ffi = ffi.assume_init();

            Self { ffi }
        }
    }

    /// [alias: new1] Constructor of the ALEInterface struct with/without display screen.
    pub fn with_display_screen(flag: bool) -> Self {
        unsafe {
            use ffi::ale::ALEInterface;
            // Allocate memory for ALEInterface FFI.
            let mut ffi = MaybeUninit::<ALEInterface>::uninit();
            // Write a new ALEInterface FFI into the memory location.
            ALEInterface::new1(flag).new(Pin::new_unchecked(&mut ffi));
            // Get the ALEInterface FFI from the memory location.
            let ffi = ffi.assume_init();

            Self { ffi }
        }
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
    // TODO: getBool
    // TODO: getDifficulty
    // TODO: getEpisodeFrameNumber
    // TODO: getFloat
    // TODO: getFrameNumber
    // TODO: getInt
    // TODO: getLegalActionSet
    // TODO: getMinimalActionSet
    // TODO: getMode
    // TODO: getRAM
    // TODO: getScreen
    // TODO: getScreenGrayscale
    // TODO: getScreenRGB
    // TODO: getString
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

impl Default for ALEInterface {
    fn default() -> Self {
        Self::new()
    }
}
