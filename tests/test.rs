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
    }

    #[test]
    fn get_int() {
        // Initialize the ALEInterface.
        let mut ale: ALEInterface = Default::default();
    }

    #[test]
    fn get_float() {
        // Initialize the ALEInterface.
        let mut ale: ALEInterface = Default::default();
    }

    #[test]
    fn get_string() {
        // Initialize the ALEInterface.
        let mut ale: ALEInterface = Default::default();
    }

    #[test]
    fn default() {
        ALEInterface::default();
    }
}
