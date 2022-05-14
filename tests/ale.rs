mod ale_action {
    use std::mem::transmute;

    use ale_interface::ALEAction;

    #[test]
    fn match_transmute() {
        let data = vec![
            (ALEAction::PLAYER_A_NOOP, 0),
            (ALEAction::PLAYER_A_FIRE, 1),
            (ALEAction::PLAYER_A_UP, 2),
            (ALEAction::PLAYER_A_RIGHT, 3),
            (ALEAction::PLAYER_A_LEFT, 4),
            (ALEAction::PLAYER_A_DOWN, 5),
            (ALEAction::PLAYER_A_UPRIGHT, 6),
            (ALEAction::PLAYER_A_UPLEFT, 7),
            (ALEAction::PLAYER_A_DOWNRIGHT, 8),
            (ALEAction::PLAYER_A_DOWNLEFT, 9),
            (ALEAction::PLAYER_A_UPFIRE, 10),
            (ALEAction::PLAYER_A_RIGHTFIRE, 11),
            (ALEAction::PLAYER_A_LEFTFIRE, 12),
            (ALEAction::PLAYER_A_DOWNFIRE, 13),
            (ALEAction::PLAYER_A_UPRIGHTFIRE, 14),
            (ALEAction::PLAYER_A_UPLEFTFIRE, 15),
            (ALEAction::PLAYER_A_DOWNRIGHTFIRE, 16),
            (ALEAction::PLAYER_A_DOWNLEFTFIRE, 17),
            (ALEAction::PLAYER_B_NOOP, 18),
            (ALEAction::PLAYER_B_FIRE, 19),
            (ALEAction::PLAYER_B_UP, 20),
            (ALEAction::PLAYER_B_RIGHT, 21),
            (ALEAction::PLAYER_B_LEFT, 22),
            (ALEAction::PLAYER_B_DOWN, 23),
            (ALEAction::PLAYER_B_UPRIGHT, 24),
            (ALEAction::PLAYER_B_UPLEFT, 25),
            (ALEAction::PLAYER_B_DOWNRIGHT, 26),
            (ALEAction::PLAYER_B_DOWNLEFT, 27),
            (ALEAction::PLAYER_B_UPFIRE, 28),
            (ALEAction::PLAYER_B_RIGHTFIRE, 29),
            (ALEAction::PLAYER_B_LEFTFIRE, 30),
            (ALEAction::PLAYER_B_DOWNFIRE, 31),
            (ALEAction::PLAYER_B_UPRIGHTFIRE, 32),
            (ALEAction::PLAYER_B_UPLEFTFIRE, 33),
            (ALEAction::PLAYER_B_DOWNRIGHTFIRE, 34),
            (ALEAction::PLAYER_B_DOWNLEFTFIRE, 35),
            (ALEAction::RESET, 40),
            (ALEAction::UNDEFINED, 41),
            (ALEAction::RANDOM, 42),
            (ALEAction::SAVE_STATE, 43),
            (ALEAction::LOAD_STATE, 44),
            (ALEAction::SYSTEM_RESET, 45),
            (ALEAction::LAST_ACTION_INDEX, 50),
        ];

        for (a, i) in data {
            // Test Action -> Integer.
            assert_eq!(a as u32, i);
            // Test Integer -> Action.
            assert_eq!(unsafe { transmute::<u32, ALEAction>(i) }, a);
        }
    }

    #[test]
    fn to_string() {
        let data = vec![
            (ALEAction::PLAYER_A_NOOP, "PLAYER_A_NOOP"),
            (ALEAction::PLAYER_A_FIRE, "PLAYER_A_FIRE"),
            (ALEAction::PLAYER_A_UP, "PLAYER_A_UP"),
            (ALEAction::PLAYER_A_RIGHT, "PLAYER_A_RIGHT"),
            (ALEAction::PLAYER_A_LEFT, "PLAYER_A_LEFT"),
            (ALEAction::PLAYER_A_DOWN, "PLAYER_A_DOWN"),
            (ALEAction::PLAYER_A_UPRIGHT, "PLAYER_A_UPRIGHT"),
            (ALEAction::PLAYER_A_UPLEFT, "PLAYER_A_UPLEFT"),
            (ALEAction::PLAYER_A_DOWNRIGHT, "PLAYER_A_DOWNRIGHT"),
            (ALEAction::PLAYER_A_DOWNLEFT, "PLAYER_A_DOWNLEFT"),
            (ALEAction::PLAYER_A_UPFIRE, "PLAYER_A_UPFIRE"),
            (ALEAction::PLAYER_A_RIGHTFIRE, "PLAYER_A_RIGHTFIRE"),
            (ALEAction::PLAYER_A_LEFTFIRE, "PLAYER_A_LEFTFIRE"),
            (ALEAction::PLAYER_A_DOWNFIRE, "PLAYER_A_DOWNFIRE"),
            (ALEAction::PLAYER_A_UPRIGHTFIRE, "PLAYER_A_UPRIGHTFIRE"),
            (ALEAction::PLAYER_A_UPLEFTFIRE, "PLAYER_A_UPLEFTFIRE"),
            (ALEAction::PLAYER_A_DOWNRIGHTFIRE, "PLAYER_A_DOWNRIGHTFIRE"),
            (ALEAction::PLAYER_A_DOWNLEFTFIRE, "PLAYER_A_DOWNLEFTFIRE"),
            (ALEAction::PLAYER_B_NOOP, "PLAYER_B_NOOP"),
            (ALEAction::PLAYER_B_FIRE, "PLAYER_B_FIRE"),
            (ALEAction::PLAYER_B_UP, "PLAYER_B_UP"),
            (ALEAction::PLAYER_B_RIGHT, "PLAYER_B_RIGHT"),
            (ALEAction::PLAYER_B_LEFT, "PLAYER_B_LEFT"),
            (ALEAction::PLAYER_B_DOWN, "PLAYER_B_DOWN"),
            (ALEAction::PLAYER_B_UPRIGHT, "PLAYER_B_UPRIGHT"),
            (ALEAction::PLAYER_B_UPLEFT, "PLAYER_B_UPLEFT"),
            (ALEAction::PLAYER_B_DOWNRIGHT, "PLAYER_B_DOWNRIGHT"),
            (ALEAction::PLAYER_B_DOWNLEFT, "PLAYER_B_DOWNLEFT"),
            (ALEAction::PLAYER_B_UPFIRE, "PLAYER_B_UPFIRE"),
            (ALEAction::PLAYER_B_RIGHTFIRE, "PLAYER_B_RIGHTFIRE"),
            (ALEAction::PLAYER_B_LEFTFIRE, "PLAYER_B_LEFTFIRE"),
            (ALEAction::PLAYER_B_DOWNFIRE, "PLAYER_B_DOWNFIRE"),
            (ALEAction::PLAYER_B_UPRIGHTFIRE, "PLAYER_B_UPRIGHTFIRE"),
            (ALEAction::PLAYER_B_UPLEFTFIRE, "PLAYER_B_UPLEFTFIRE"),
            (ALEAction::PLAYER_B_DOWNRIGHTFIRE, "PLAYER_B_DOWNRIGHTFIRE"),
            (ALEAction::PLAYER_B_DOWNLEFTFIRE, "PLAYER_B_DOWNLEFTFIRE"),
            (ALEAction::RESET, "RESET"),
            (ALEAction::UNDEFINED, "UNDEFINED"),
            (ALEAction::RANDOM, "RANDOM"),
            (ALEAction::SAVE_STATE, "SAVE_STATE"),
            (ALEAction::LOAD_STATE, "LOAD_STATE"),
            (ALEAction::SYSTEM_RESET, "SYSTEM_RESET"),
            (ALEAction::LAST_ACTION_INDEX, "LAST_ACTION_INDEX"),
        ];

        for (a, s) in data {
            assert_eq!(a.to_string(), s.to_string());
        }
    }
}

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
