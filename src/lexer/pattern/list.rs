pub mod list {
    use crate::lexer::lexer::lexer::{Content, ElementNode, Token};
    use crate::lexer::pattern::inline::inline::inline_parse;
    use once_cell::sync::Lazy;
    use regex::{Captures, Regex};

    static SIMPLE_LIST_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^([\s\s]*)[\*-]\s(.+)").unwrap());

    static NUMBER_LIST_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^([\s\s]*)\d+\.\s(.+)").unwrap());

    #[derive(Clone, Copy, Debug)]
    pub enum ListPattern {
        SimpleList,
        NumberList,
    }

    impl ListPattern {
        // pub fn value(&self) -> Regex {
        //     match *self {
        //         ListPattern::SimpleList => *SIMPLE_LIST_PATTERN,
        //         ListPattern::NumberList => *NUMBER_LIST_PATTERN,
        //     }
        // }

        pub fn parse<'a>(&self, text: &'a String) -> Option<Captures<'a>> {
            match self {
                ListPattern::SimpleList => &SIMPLE_LIST_PATTERN,
                ListPattern::NumberList => &NUMBER_LIST_PATTERN,
            }
            .captures(text)
        }
    }

    pub fn is_number_list(input: &String) -> bool {
        NUMBER_LIST_PATTERN.is_match(input)
    }

    pub fn is_simple_list(input: &String) -> bool {
        SIMPLE_LIST_PATTERN.is_match(input)
    }

    pub fn parse_list(input: Vec<String>, pattern: ListPattern, now_indent: usize) -> ElementNode {
        let mut results_node = ElementNode::new(
            match pattern {
                ListPattern::SimpleList => Token::Ul,
                ListPattern::NumberList => Token::Ol,
            },
            Content::ElementNodes { value: vec![] },
            Box::new(ElementNode::Nil),
        );
        let mut at = 0;
        let input_len = input.len();
        while at < input_len {
            let text = input.get(at).unwrap();
            let caps = pattern.parse(text).unwrap();
            let indent_length = caps.get(1).map_or(0, |m| m.as_str().len());
            let content = caps.get(2).map_or("", |m| m.as_str());

            if indent_length > now_indent {
                let mut start_index = at;
                loop {
                    at += 1;
                    if at == input_len {
                        break;
                    }
                    let _indent_length = pattern
                        .parse(input.get(at).unwrap())
                        .unwrap()
                        .get(1)
                        .map_or(0, |m| m.as_str().len());
                    if _indent_length < indent_length {
                        break;
                    }
                }

                let parse_result =
                    parse_list(input[start_index..at].to_owned(), pattern, indent_length);
                match &mut results_node {
                    ElementNode::Exist {
                        ref mut content, ..
                    } => match &mut **content {
                        Content::ElementNodes { ref mut value } => {
                            match value.last_mut() {
                                Some(value) => match value {
                                    ElementNode::Exist {
                                        ref mut children, ..
                                    } => *children = Box::new(parse_result),
                                    _ => panic!(""),
                                },
                                _ => panic!(""),
                            };
                        }
                        _ => panic!("Type is not correct"),
                    },
                    _ => panic!("Type is not correct"),
                }
                at -= 1;
            } else {
                match &mut results_node {
                    ElementNode::Exist {
                        content: ref mut node_content,
                        ..
                    } => match &mut **node_content {
                        Content::ElementNodes { ref mut value } => value.push(ElementNode::new(
                            Token::Li,
                            Content::PlainText {
                                value: inline_parse(&content.to_string()),
                            },
                            Box::new(ElementNode::Nil),
                        )),
                        _ => panic!("Type is not correct"),
                    },
                    _ => panic!("Type is not correct"),
                }
            }
            at += 1;
        }
        results_node
    }

    #[cfg(test)]
    mod test_list {
        use super::*;
        use crate::lexer::lexer::lexer::Content::{ElementNodes, PlainText};
        use crate::lexer::pattern::list::list::ListPattern::NumberList;
        use crate::{content_element_nodes, content_plain_text, element_node};

        #[test]
        fn test_is_number_list() {
            #[derive(Debug)]
            struct TestCase {
                it: String,
                input: String,
                expected: bool,
            }
            let test_cases = [
                TestCase {
                    it: String::from("should return true when format is list `1.` (nest 0)"),
                    input: String::from("1. aaa"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return true when format is list `  2.` (nest 1)"),
                    input: String::from("  2. aaa"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return true when format is list `10.` (nest 0)"),
                    input: String::from("10. aaa"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return false when format is not list"),
                    input: String::from("aaa"),
                    expected: false,
                },
            ];
            for test_case in test_cases.iter() {
                let output = is_number_list(&test_case.input);
                assert_eq!(output, test_case.expected, "Failed: {}\n", test_case.it);
            }
        }

        #[test]
        fn test_is_simple_list() {
            #[derive(Debug)]
            struct TestCase {
                it: String,
                input: String,
                expected: bool,
            }
            let test_cases = [
                TestCase {
                    it: String::from("should return true when format is list `*` (nest 0)"),
                    input: String::from("* aaa"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return true when format is list `*` (nest 1)"),
                    input: String::from("  * aaa"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return true when format is list `-` (nest 1)"),
                    input: String::from("  - aaa"),
                    expected: true,
                },
                TestCase {
                    it: String::from("should return true when format is not list"),
                    input: String::from("aaa"),
                    expected: false,
                },
            ];
            for test_case in test_cases.iter() {
                let output = is_simple_list(&test_case.input);
                assert_eq!(output, test_case.expected, "Failed: {}\n", test_case.it);
            }
        }

        #[test]
        fn test_parse_list() {
            #[derive(Debug)]
            struct TestCase {
                it: String,
                input: Vec<String>,
                pattern: ListPattern,
                expected: ElementNode,
            }
            let test_cases = [
                TestCase {
                    it: String::from("should correctly parse simple list"),
                    input: [
                        "* hogehoge",
                        "* hogehoge1",
                        "  * this is test",
                        "  * hogehoge3",
                        "    * hoge 4",
                        "* hogehoge4",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    pattern: ListPattern::SimpleList,
                    expected: element_node! {
                        tag: Token::Ul,
                        content: content_element_nodes![
                            element_node! {
                                tag: Token::Li,
                                content: content_plain_text!("hogehoge".to_string()),
                            },
                            element_node! {
                                tag: Token::Li,
                                content: content_plain_text!("hogehoge1".to_string()),
                                children: element_node! {
                                    tag: Token::Ul,
                                    content: content_element_nodes![
                                        element_node! {
                                            tag: Token::Li,
                                            content: content_plain_text!("this is test".to_string()),
                                        },
                                        element_node! {
                                            tag: Token::Li,
                                            content: content_plain_text!("hogehoge3".to_string()),
                                            children: element_node! {
                                                tag: Token::Ul,
                                                content: content_element_nodes![
                                                    element_node! {
                                                        tag: Token::Li,
                                                        content: content_plain_text!("hoge 4".to_string()),
                                                    }
                                                ]
                                            }
                                        },
                                    ],
                                }
                            },
                            element_node! {
                                tag: Token::Li,
                                content: content_plain_text!("hogehoge4".to_string()),
                            },
                        ]
                    },
                },
                TestCase {
                    it: String::from("should correctly parse number list"),
                    input: ["1. hoge1", "2. hoge2", "  1. aaa", "  2. ccc", "    1. ddd"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    pattern: ListPattern::NumberList,
                    expected: element_node! {
                        tag: Token::Ol,
                        content: content_element_nodes![
                            element_node! {
                                tag: Token::Li,
                                content: content_plain_text!("hoge1".to_string()),
                            },
                            element_node! {
                                tag: Token::Li,
                                content: content_plain_text!("hoge2".to_string()),
                                children: element_node! {
                                    tag: Token::Ol,
                                    content: content_element_nodes![
                                        element_node! {
                                            tag: Token::Li,
                                            content: content_plain_text!("aaa".to_string()),
                                        },
                                        element_node! {
                                            tag: Token::Li,
                                            content: content_plain_text!("ccc".to_string()),
                                            children: element_node! {
                                                tag: Token::Ol,
                                                content: content_element_nodes![
                                                    element_node! {
                                                        tag: Token::Li,
                                                        content: content_plain_text!("ddd".to_string()),
                                                    }
                                                ]
                                            }
                                        },
                                    ],
                                }
                            },
                        ]
                    },
                },
            ];

            for test_case in test_cases.iter() {
                let output = parse_list(
                    test_case.input.iter().map(|s| s.into()).collect(),
                    test_case.pattern,
                    0,
                );
                assert_eq!(output, test_case.expected, "Failed: {}\n", test_case.it);
            }
        }
    }
}
