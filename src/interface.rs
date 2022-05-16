use std::{collections::BTreeSet, mem::MaybeUninit, path::Path, pin::Pin};

use autocxx::prelude::*;
use cxx::let_cxx_string;

use crate::{
    ffi,
    ffi::ale::{ALEScreen, ALEState, ALERAM},
    ALEAction,
};

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
            // Write the new ALEInterface FFI into the memory location.
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
            // Write the new ALEInterface FFI into the memory location.
            ALEInterface::new1(flag).new(Pin::new_unchecked(&mut ffi));
            // Get the ALEInterface FFI from the memory location.
            let ffi = ffi.assume_init();

            Self { ffi }
        }
    }

    /// Applies an action to the game and returns the reward [alias: act].
    pub fn act(&mut self, action: ALEAction) -> i32 {
        unsafe { Pin::new_unchecked(&mut self.ffi).act(action).into() }
    }

    /// Clones the environment state. [alias: cloneState].
    pub fn clone_state(&mut self, include_rng: bool) -> ALEState {
        unsafe {
            // Allocate memory for ALEState FFI.
            let mut state = MaybeUninit::<ALEState>::uninit();
            // Write the new ALEState FFI into the memory location.
            Pin::new_unchecked(&mut self.ffi)
                .cloneState(include_rng)
                .new(Pin::new_unchecked(&mut state));
            // Get the ALEState FFI from the memory location.
            state.assume_init()
        }
    }

    /// Clones the system and environment state, suitable for serialization [alias: cloneSystemState].
    pub fn clone_system_state(&mut self) -> ALEState {
        unsafe {
            // Allocate memory for ALEState FFI.
            let mut state = MaybeUninit::<ALEState>::uninit();
            // Write the new ALEState FFI into the memory location.
            Pin::new_unchecked(&mut self.ffi)
                .cloneSystemState()
                .new(Pin::new_unchecked(&mut state));
            // Get the ALEState FFI from the memory location.
            state.assume_init()
        }
    }

    // TODO: createOSystem
    // TODO: createScreenExporter
    // FIXME: disableBufferedIO

    /// Checks if the game has ended [alias: game_over].
    pub fn is_game_over(&self) -> bool {
        unsafe { self.ffi.game_over() }
    }

    // FIXME: getAvailableDifficulties
    // FIXME: getAvailableModes

    /// Gets boolean attribute given key [alias: getBool].
    pub fn get_bool(&mut self, key: &str) -> bool {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(k = key);

            Pin::new_unchecked(&mut self.ffi).getBool(&k)
        }
    }

    /// Returns the current difficulty switch setting [alias: getDifficulty].
    pub fn get_difficulty(&mut self) -> u32 {
        unsafe { Pin::new_unchecked(&mut self.ffi).getDifficulty().into() }
    }

    /// Returns the frame number since the start of the current episode [alias: getEpisodeFrameNumber].
    pub fn get_episode_frame_count(&mut self) -> usize {
        unsafe { i32::from(Pin::new_unchecked(&mut self.ffi).getEpisodeFrameNumber()) as usize }
    }

    /// Gets float attribute given key [alias: getFloat].
    pub fn get_float(&mut self, key: &str) -> f32 {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(k = key);

            Pin::new_unchecked(&mut self.ffi).getFloat(&k)
        }
    }

    /// Returns the frame number since the loading of the ROM [alias: getFrameNumber].
    pub fn get_total_frame_count(&mut self) -> usize {
        unsafe { i32::from(Pin::new_unchecked(&mut self.ffi).getFrameNumber()) as usize }
    }

    /// Gets integer attribute given key [alias: getInt].
    pub fn get_int(&mut self, key: &str) -> i32 {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(k = key);

            Pin::new_unchecked(&mut self.ffi).getInt(&k).into()
        }
    }

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

    /// Returns the game mode value last specified to the environment [alias: getMode].
    pub fn get_game_mode(&mut self) -> u32 {
        unsafe { Pin::new_unchecked(&mut self.ffi).getMode().into() }
    }

    /// Returns the current RAM content [alias: getRAM].
    pub fn get_ram(&mut self) -> &ALERAM {
        unsafe { Pin::new_unchecked(&mut self.ffi).getRAM() }
    }

    /// Returns the current game screen [alias: getScreen].
    pub fn get_screen(&mut self) -> &ALEScreen {
        unsafe { Pin::new_unchecked(&mut self.ffi).getScreen() }
    }

    // TODO: getScreenGrayscale
    // TODO: getScreenRGB

    /// Gets string attribute given key [alias: getString].
    pub fn get_string(&mut self, key: &str) -> String {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(k = key);

            Pin::new_unchecked(&mut self.ffi)
                .getString(&k)
                .as_ref()
                .expect("Invalid pointer")
                .to_str()
                .expect("Invalid UTF-8 encoding")
                .to_string()
        }
    }

    // FIXME: getStringInplaceReturns the frame number since the loading of the ROM
    // FIXME: isSupportedROM

    /// Gets the remaining number of lives [alias: lives].
    pub fn get_lives(&mut self) -> i32 {
        unsafe { Pin::new_unchecked(&mut self.ffi).lives().into() }
    }

    // FIXME: loadROM
    pub fn load_rom(&mut self, path: &Path) {
        unsafe {
            // Map Path into String.
            let path = path.to_string_lossy().into_owned();
            // Map String into std::string.
            let_cxx_string!(p = path);
            // Get the std::filesystem::path constructor.
            let path = ffi::utils::string_to_path(&p);
            // Pass the constructor to the FFI call.
            let path = as_new(path);
            // Load ROM with the given path.
            Pin::new_unchecked(&mut self.ffi).loadROM(path)
        }
    }

    // FIXME: loadSettings

    /// Resets the game, but not the full system [alias: reset_game].
    pub fn reset_game(&mut self) {
        unsafe { Pin::new_unchecked(&mut self.ffi).reset_game() }
    }

    /// Restores the state, if it was cloned including the RNG then the RNG will be restored [alias: restoreState].
    pub fn restore_state(&mut self, state: &ALEState) {
        unsafe { Pin::new_unchecked(&mut self.ffi).restoreState(state) }
    }

    /// This is maintained for backwards compatability and is equivalent to call restore_state [alias: restoreSystemState].
    pub fn restore_system_state(&mut self, state: &ALEState) {
        unsafe { Pin::new_unchecked(&mut self.ffi).restoreSystemState(state) }
    }

    /// Save the current screen as a PNG file [alias: saveScreenPNG].
    pub fn save_screen_png(&mut self, filename: &str) {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(f = filename);

            Pin::new_unchecked(&mut self.ffi).saveScreenPNG(&f)
        }
    }

    /// Sets a boolean attribute given key [alias: setBool].
    pub fn set_bool(&mut self, key: &str, value: bool) {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(k = key);

            Pin::new_unchecked(&mut self.ffi).setBool(&k, value)
        }
    }

    /// Sets the difficulty of the game [alias: setDifficulty].
    pub fn set_difficulty(&mut self, mode: u32) {
        unsafe { Pin::new_unchecked(&mut self.ffi).setDifficulty(mode.into()) }
    }

    /// Sets a float attribute given key [alias: setFloat].
    pub fn set_float(&mut self, key: &str, value: f32) {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(k = key);

            Pin::new_unchecked(&mut self.ffi).setFloat(&k, value)
        }
    }

    /// Sets a integer attribute given key [alias: setInt].
    pub fn set_int(&mut self, key: &str, value: i32) {
        unsafe {
            // Map the key to the associated type.
            let_cxx_string!(k = key);

            Pin::new_unchecked(&mut self.ffi).setInt(&k, value.into())
        }
    }

    /// Sets the mode of the game [alias: setMode].
    pub fn set_game_mode(&mut self, mode: u32) {
        unsafe { Pin::new_unchecked(&mut self.ffi).setMode(mode.into()) }
    }

    /// Sets byte at given memory address [alias: setRAM].
    pub fn set_ram(&mut self, idx: usize, value: u8) {
        unsafe { Pin::new_unchecked(&mut self.ffi).setRAM(idx, value) }
    }

    /// Sets a string attribute given key [alias: setString].
    pub fn set_string(&mut self, key: &str, value: &str) {
        unsafe {
            // Map the key and the value to the associated types.
            let_cxx_string!(k = key);
            let_cxx_string!(v = value);

            Pin::new_unchecked(&mut self.ffi).setString(&k, &v)
        }
    }

    /// Display ALE welcome message [alias: welcomeMessage].
    pub fn get_welcome_message() -> String {
        unsafe {
            ffi::ale::ALEInterface::welcomeMessage()
                .as_ref()
                .expect("Invalid pointer")
                .to_str()
                .expect("Invalid UTF-8 encoding")
                .to_string()
        }
    }
}

impl Default for ALEInterface {
    fn default() -> Self {
        Self::new()
    }
}
