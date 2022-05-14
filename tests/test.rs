mod ale_interface {
    use ale_interface::ALEInterface;

    #[test]
    fn new() {
        ALEInterface::new();
    }

    #[test]
    fn with_display_screen() {
        // Test with/without display screen.
        ALEInterface::with_display_screen(false);
        ALEInterface::with_display_screen(true);
    }

    #[test]
    fn default() {
        ALEInterface::default();
    }
}
