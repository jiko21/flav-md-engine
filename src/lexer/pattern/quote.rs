pub mod quote {
    use once_cell::sync::Lazy;
    use regex::Regex;

    static CODE_BLOCK_PAREN_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^>\s*([\s>]*.+)").unwrap());

    pub fn is_quote_block(input: &String) -> bool {
        match input.chars().nth(0) {
            Some(c) => c == '>',
            None => false,
        }
    }

    pub fn enclose_quote(input: Vec<String>) -> Vec<String> {
        input
            .iter()
            .map(|s| {
                let rslt = CODE_BLOCK_PAREN_PATTERN.captures(s);
                match rslt {
                    Some(c) => c.get(1).map_or(s.to_string(), |m| m.as_str().to_string()),
                    None => s.to_string(),
                }
            })
            .collect::<Vec<String>>()
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

        #[test]
        fn test_enclose_quote() {
            let input = vec!["> hogehoge", "> this is test", "aaa"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            let expected: Vec<String> = vec!["hogehoge", "this is test", "aaa"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            let output = enclose_quote(input);
            assert_eq!(output, expected);
        }
    }
}
