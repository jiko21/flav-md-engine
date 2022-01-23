pub mod quote {
    pub fn is_quote_block(input: &String) -> bool {
        input.chars().nth(0).unwrap() == '>'
    }

    #[cfg(test)]
    mod test_quote {
        use super::*;

        #[test]
        fn test_is_quote_block() {
            #[derive(Debug)]
            struct TestCase {
                it: String,
                input: String,
                expected: bool,
            }

            let test_cases = [
                TestCase {
                    it: String::from("should return true when input is block"),
                    input: String::from("> hogehoge"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return false when input is not block"),
                    input: String::from("hogehoge"),
                    expected: false,
                },
            ];

            for test_case in test_cases.iter() {
                let output = is_quote_block(&test_case.input);
                assert_eq!(output, test_case.expected, "Failed: {}\n", test_case.it);
            }
        }
    }
}
