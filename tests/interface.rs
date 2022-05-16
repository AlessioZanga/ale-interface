mod interface {
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
}
