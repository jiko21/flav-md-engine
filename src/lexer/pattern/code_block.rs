pub mod code_block {
    use crate::util::string::string::escape_code_string;
    use once_cell::sync::Lazy;
    use regex::Regex;

    static CODE_BLOCK_PAREN_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^```[^`|.]*$").unwrap());

    pub fn is_code_block_start(input: &String) -> bool {
        CODE_BLOCK_PAREN_PATTERN.is_match(input)
    }

    pub fn parse_code_block(input: Vec<String>) -> Vec<String> {
        input.iter().map(|s| escape_code_string(s)).collect()
    }

    #[cfg(test)]
    mod test_code_block {
        use super::*;

        #[test]
        fn test_is_code_block_start() {
            #[derive(Debug)]
            struct TestCase {
                it: String,
                input: String,
                expected: bool,
            }
            let test_cases = [
                TestCase {
                    it: String::from("should return true when input is ```"),
                    input: String::from("```"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return true when input is ```<langname>"),
                    input: String::from("```html"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return true when input is ``"),
                    input: String::from("``"),
                    expected: false,
                },
                TestCase {
                    it: String::from("should return true when input is `"),
                    input: String::from("``"),
                    expected: false,
                },
            ];
            for test_case in test_cases.iter() {
                let output = is_code_block_start(&test_case.input);
                assert_eq!(output, test_case.expected, "Failed: {}\n", test_case.it);
            }
        }

        #[test]
        fn test_parse_code_block() {
            let input = [
                r#"<script src="/a/b.js">alert('aaa')</script>"#,
                r#"'aaa'"#,
                r#""aaa""#,
            ]
            .iter()
            .map(|&s| s.into())
            .collect();
            let expected: Vec<String> = [
                "&lt;script src=&quot;/a/b.js&quot;&gt;alert(&#39;aaa&#39;)&lt;/script&gt;",
                "&#39;aaa&#39;",
                "&quot;aaa&quot;",
            ]
            .iter()
            .map(|&s| s.into())
            .collect();
            let output = parse_code_block(input);
            assert_eq!(output, expected);
        }
    }
}
