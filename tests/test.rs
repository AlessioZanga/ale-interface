mod ale_interface {
    use ale_interface::ALEInterface;

    #[test]
    fn new() {
        ALEInterface::new();
    }

    #[test]
    fn with_display_screen() {
        // Test with/without display screen.
        ALEInterface::with_display_screen(true);
        ALEInterface::with_display_screen(false);
    }

    #[test]
    fn get_bool() {
        // Initialize the ALEInterface.
        let mut ale: ALEInterface = Default::default();

        // Test `display_screen` attribute.
        assert_eq!(ale.get_bool("display_screen"), false);
        // Test `sound` attribute.
        assert_eq!(ale.get_bool("sound"), false);
    }

    #[test]
    fn default() {
        ALEInterface::default();
    }
}
