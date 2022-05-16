use std::{collections::BTreeSet, mem::MaybeUninit, pin::Pin};

use autocxx::prelude::*;

use crate::{ffi, ffi::ale::reward_t as ALEReward, ALEAction};

/// Rust FFI binding for the ALEInterface class.
pub struct ALEInterface {
    ffi: ffi::ale::ALEInterface,
}

impl ALEInterface {
    /// Constructor of the ALEInterface struct [alias: new].
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

    /// Constructor of the ALEInterface struct with/without display screen [alias: new1].
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

    /// Applies an action to the game and returns the reward [alias: act].
    pub fn act(&mut self, action: ALEAction) -> ALEReward {
        unsafe { Pin::new_unchecked(&mut self.ffi).act(action) }
    }

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

    /// Returns the vector of legal actions [alias: getLegalActionSet].
    pub fn get_actions(&mut self) -> BTreeSet<ALEAction> {
        unsafe {
            // Get wrapper around the returned value.
            let actions = Pin::new_unchecked(&mut self.ffi).getLegalActionSet();
            // Map return value to associated type.
            actions
                .as_ref()
                .expect("Invalid pointer")
                .into_iter()
                .cloned()
                .collect()
        }
    }

    /// Returns the vector of the minimal set of actions needed to play the game [alias: getMinimalActionSet].
    pub fn get_minimal_actions(&mut self) -> BTreeSet<ALEAction> {
        unsafe {
            // Get wrapper around the returned value.
            let actions = Pin::new_unchecked(&mut self.ffi).getLegalActionSet();
            // Map return value to associated type.
            actions
                .as_ref()
                .expect("Invalid pointer")
                .into_iter()
                .cloned()
                .collect()
        }
    }

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
