mod action {
    use ale_interface::ALEAction;

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
            // TODO: Check for missing values.
            (ALEAction::SAVE_STATE, ""),
            (ALEAction::LOAD_STATE, ""),
            (ALEAction::SYSTEM_RESET, ""),
            (ALEAction::LAST_ACTION_INDEX, ""),
        ];

        for (a, s) in data {
            assert_eq!(a.to_string(), s.to_string());
        }
    }
}
