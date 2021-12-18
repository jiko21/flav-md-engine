pub mod string {
    pub fn split_string(content: String) -> Vec<String> {
        content.split('\n').into_iter().map(|s| s.into()).collect()
    }

    pub fn escape_code_string(content: &String) -> String {
        content
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;")
    }

    #[cfg(test)]
    mod test_split_string {
        use super::*;

        #[test]
        fn should_return_true_when_code_block_starts_with_lang_name() {
            let input = r#"this
is
a"#;
            let result = split_string(input.to_string());
            let expected: Vec<String> = ["this", "is", "a"].iter().map(|&s| s.into()).collect();
            assert_eq!(result, expected);
        }
    }

    #[cfg(test)]
    mod test_escape_code_string {
        use super::*;

        #[derive(Debug)]
        struct TestCase {
            it: String,
            input: String,
            expected: String,
        }

        #[test]
        fn test() {
            let test_cases = [
                TestCase {
                    it: String::from("should correctly escape &"),
                    input: String::from("true && false"),
                    expected: String::from("true &amp;&amp; false"),
                },
                TestCase {
                    it: String::from("should correctly escape <"),
                    input: String::from("<script"),
                    expected: String::from("&lt;script"),
                },
                TestCase {
                    it: String::from("should correctly escape >"),
                    input: String::from("script>"),
                    expected: String::from("script&gt;"),
                },
                TestCase {
                    it: String::from("should correctly escape '"),
                    input: String::from("'aaa'"),
                    expected: String::from("&#39;aaa&#39;"),
                },
                TestCase {
                    it: String::from("should correctly escape whole script tag"),
                    input: String::from(r#"<script src="/a/b.js">alert('aaa')</script>"#),
                    expected: String::from(
                        "&lt;script src=&quot;/a/b.js&quot;&gt;alert(&#39;aaa&#39;)&lt;/script&gt;",
                    ),
                },
            ];
            for test_case in test_cases.iter() {
                let output = escape_code_string(&test_case.input);
                assert_eq!(output, test_case.expected, "Failed: {}\n", test_case.it);
            }
        }
    }
}
