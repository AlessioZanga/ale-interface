mod utils {
    use ale_interface::utils::{from_std_string, from_std_vector_u32, into_std_string, into_std_vector_u32};

    #[test]
    fn std_string() {
        let data = vec!["", "0", "01", "0123456789"];

        for s in data {
            // Map &str to std_string.
            let std_string = into_std_string(s);
            // Map std_string to String.
            let string = from_std_string(std_string);

            assert_eq!(s, string.as_str());
        }
    }

    #[test]
    fn std_vector() {
        let data: Vec<Vec<u32>> = vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]];

        for v in data {
            // Map Vec<u32> to std_vector.
            let std_vector = into_std_vector_u32(v.clone());
            // Map std_string to String.
            // let vector = from_std_vector_u32(std_vector);

            // assert_eq!(v, vector);
        }
    }
}
