use std::{collections::BTreeSet, mem::transmute, path::Path};

use crate::{ffi, utils};

/// ALEAction enumerator.
pub type ALEAction = ffi::ale::Action;

/// ALEScreenExporter.
pub type ALEScreenExporter = ffi::ale::ScreenExporter;

/// ALEState.
pub type ALEState = ffi::ale::ALEState;

/// Rust binding of the ALEInterface class.
pub struct ALEInterface {
    ffi: ffi::ale::ALEInterface,
}

impl ALEInterface {
    /// [alias: ALEInterface] Constructs an ALEInterface struct.
    pub fn new() -> Self {
        // Initialize the ALEInterface FFI.
        let ffi = unsafe { ffi::ale::ALEInterface::new() };
        // Safe because is a constructor.
        Self { ffi }
    }

    /// [alias: ALEInterface] Constructs an ALEInterface struct with or without a display screen.
    pub fn with_display_screen(flag: bool) -> Self {
        // Initialize the ALEInterface FFI by setting the `display_screen` flag.
        let ffi = unsafe { ffi::ale::ALEInterface::new1(flag) };
        // Safe because is a constructor.
        Self { ffi }
    }

    /// [alias: act] Perform the given action and return the obtained reward.
    // TODO: Check return type.
    pub fn act(&mut self, action: ALEAction) -> i32 {
        unsafe { self.ffi.act(action) }
    }

    /// [alias: cloneState] Clone the state of the ALEInterface.
    // TODO: Check for unnecessary mutability on the C++ side.
    pub fn clone_state(&mut self, with_rng: bool) -> ALEState {
        unsafe { self.ffi.cloneState(with_rng) }
    }

    /// [alias: cloneSystemState] Clone the state of the ALEInterface.
    // TODO: Check for unnecessary mutability on the C++ side.
    // TODO: Check for overloading of `clone_state`.
    pub fn close_system_state(&mut self) -> ALEState {
        unsafe { self.ffi.cloneSystemState() }
    }

    // TODO: createOSystem
    pub fn create_os_system(&mut self) {
        todo!()
    }

    // TODO: createScreenExporter
    pub fn create_screen_exporter(&mut self) {
        todo!()
    }

    // TODO: disableBufferedIO
    pub fn disable_buffered_io(&mut self, path: &Path) -> &mut ALEScreenExporter {
        todo!()
    }

    /// [alias: game_over] Checks whether the game is over or not.
    pub fn is_game_over(&self) -> bool {
        // Safe because is a getter.
        unsafe { self.ffi.game_over() }
    }

    /// [alias: getAvailableDifficulties] Gets the vector of difficulties available for the current game.
    // TODO: Check for unnecessary mutability on the C++ side.
    pub fn get_difficulties(&mut self) -> Vec<u32> {
        unsafe { utils::from_std_vector_u32(self.ffi.getAvailableDifficulties()) }
    }

    /// [alias: getAvailableModes] Gets the vector of modes available for the current game.
    // TODO: Check for unnecessary mutability on the C++ side.
    pub fn get_game_modes(&mut self) -> Vec<u32> {
        unsafe { utils::from_std_vector_u32(self.ffi.getAvailableModes()) }
    }

    /// [alias: getBool] Gets boolean attribute given key.
    // TODO: Check for unnecessary mutability on the C++ side.
    // TODO: If `key` is not set, then it should returns None, instead of the default value.
    pub fn get_bool(&mut self, key: &str) -> bool {
        unsafe { self.ffi.getBool(&utils::into_std_string(key)) }
    }

    /// [alias: getEpisodeFrameNumber] Gets the frame number since the start of the current episode.
    pub fn get_episode_frames_count(&self) -> usize {
        unsafe { self.ffi.getEpisodeFrameNumber() as usize }
    }

    /// [alias: getFloat] Gets float attribute given key.
    // TODO: Check for unnecessary mutability on the C++ side.
    // TODO: If `key` is not set, then it should returns None, instead of the default value.
    pub fn get_float(&mut self, key: &str) -> f32 {
        unsafe { self.ffi.getFloat(&utils::into_std_string(key)) }
    }

    // TODO: getFrameNumber

    /// [alias: getInt] Gets integer attribute given key.
    // TODO: Check for unnecessary mutability on the C++ side.
    // TODO: If `key` is not set, then it should returns None, instead of the default value.
    pub fn get_int(&mut self, key: &str) -> i32 {
        unsafe { self.ffi.getInt(&utils::into_std_string(key)) }
    }

    /// [alias: getLegalActionSet] Gets the set of legal actions.
    // TODO: Check for unnecessary mutability on the C++ side.
    pub fn get_legal_action_set(&mut self) -> BTreeSet<ALEAction> {
        unsafe {
            // Get the vector of legal actions.
            let actions = utils::from_std_vector_u32(self.ffi.getLegalActionSet());
            // Map action values to the associated enum.
            let actions = actions.into_iter().map(|x| transmute::<_, ALEAction>(x));
            // Construct the legal actions set.
            actions.collect()
        }
    }

    /// [alias: getLegalActionSet] Gets the set of minimal actions.
    // TODO: Check for unnecessary mutability on the C++ side.
    pub fn get_minimal_action_set(&mut self) -> BTreeSet<ALEAction> {
        unsafe {
            // Get the vector of legal actions.
            let actions = utils::from_std_vector_u32(self.ffi.getMinimalActionSet());
            // Map action values to the associated enum.
            let actions = actions.into_iter().map(|x| transmute::<_, ALEAction>(x));
            // Construct the legal actions set.
            actions.collect()
        }
    }

    // TODO: getRAM
    // TODO: getScreen
    // TODO: getScreenGrayscale
    // TODO: getScreenRGB

    /// [alias: getString] Gets string attribute given key.
    // TODO: Check for unnecessary mutability on the C++ side.
    // TODO: If `key` is not set, then it should returns None, instead of the default value.
    pub fn get_string(&mut self, key: &str) -> String {
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

impl Default for ALEInterface {
    fn default() -> Self {
        Self::new()
    }
}
