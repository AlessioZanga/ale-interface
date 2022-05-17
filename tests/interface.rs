mod interface {
    use std::path::PathBuf;

    use ale_interface::ALEInterface;

    #[test]
    fn new() {
        // Test constructor.
        ALEInterface::new();
    }

    #[test]
    fn with_display_screen() {
        // Test without display screen.
        ALEInterface::with_display_screen(false);
        // Test with display screen.
        ALEInterface::with_display_screen(true);
    }

    #[test]
    fn default() {
        // Test default constructor.
        ALEInterface::default();
    }

    #[test]
    fn load_rom() {
        // Set reference ROM path.
        let path = PathBuf::from("ale/tests/resources/tetris.bin");

        assert!(&path.exists());

        // Initialize default environment.
        let mut env: ALEInterface = Default::default();
        // Load ROM given path.
        env.load_rom(&path);
    }
}
