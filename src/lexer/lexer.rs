pub mod lexer {
    use crate::lexer::builder::builder::MdNode;
    use crate::lexer::pattern::code_block::code_block::{is_code_block_start, parse_code_block};
    use crate::lexer::pattern::inline::inline::inline_parse;
    use crate::lexer::pattern::list::list::ListPattern::SimpleList;
    use crate::lexer::pattern::list::list::{
        is_number_list, is_simple_list, parse_list, ListPattern,
    };
    use crate::lexer::pattern::quote::quote::{enclose_quote, is_quote_block};
    use crate::lexer::pattern::table::table::{is_table_block_start, parse_table};

    #[derive(Debug, PartialEq)]
    pub enum Token {
        H1,
        H2,
        H3,
        H4,
        H5,
        H6,
        P,
        Ul,
        Ol,
        Li,
        Blockquote,
        Code,
        Table,
    }

    impl Token {
        pub fn value(&self) -> String {
            match *self {
                Token::H1 => "h1",
                Token::H2 => "h2",
                Token::H3 => "h3",
                Token::H4 => "h4",
                Token::H5 => "h5",
                Token::H6 => "h6",
                Token::P => "p",
                Token::Ul => "ul",
                Token::Ol => "ol",
                Token::Li => "li",
                Token::Blockquote => "Blockquote",
                Token::Code => "code",
                Token::Table => "table",
            }
            .to_string()
        }

        pub fn value_of(number: i8) -> Self {
            match number {
                1 => Token::H1,
                2 => Token::H2,
                3 => Token::H3,
                4 => Token::H4,
                5 => Token::H5,
                6 => Token::H6,
                _ => Token::P,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Table {
        pub head: Vec<TableHead>,
        pub body: Vec<Vec<String>>,
    }

    impl Table {
        pub fn new(head: Vec<TableHead>, body: Vec<Vec<String>>) -> Self {
            Table { head, body }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct TableHead {
        cell: String,
        align: Align,
    }

    impl TableHead {
        pub fn new(cell: String, align: Align) -> Self {
            TableHead { cell, align }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Align {
        Center,
        Left,
        Right,
    }

    impl Align {
        pub fn value(&self) -> String {
            match *self {
                Align::Center => "center",
                Align::Left => "left",
                Align::Right => "right",
            }
            .to_string()
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Content {
        PlainText { value: String },
        ElementNode { value: ElementNode },
        ElementNodes { value: Vec<ElementNode> },
        Table { value: Table },
    }

    #[derive(Debug, PartialEq)]
    pub enum ElementNode {
        Exist {
            tag: Token,
            content: Box<Content>,
            children: Box<ElementNode>,
        },
        Nil,
    }

    impl ElementNode {
        pub fn new(tag: Token, content: Content, children: Box<ElementNode>) -> Self {
            ElementNode::Exist {
                tag,
                content: Box::new(content),
                children,
            }
        }
    }

    #[macro_export]
    macro_rules! table {
        (head: $head:expr, body: $body:expr $(,)? ) => {
            Table {
                head: $head,
                body: $body,
            }
        };
    }

    #[macro_export]
    macro_rules! element_node {
        (tag: $tag:expr, content: $content:expr, children: $children:expr $(,)? ) => {
            ElementNode::Exist {
                tag: $tag,
                content: Box::new($content),
                children: Box::new($children),
            }
        };
        (tag: $tag:expr, content: $content:expr $(,)? ) => {
            ElementNode::Exist {
                tag: $tag,
                content: Box::new($content),
                children: Box::new(element_node!()),
            }
        };
        () => {
            ElementNode::Nil
        };
    }

    #[macro_export]
    macro_rules! content_element_nodes {
        ($($x : expr), + $(,) ? ) => {
            Content::ElementNodes { value: vec![$($x), +] }
        };
    }

    #[macro_export]
    macro_rules! content_plain_text {
        ($value:expr $(,)? ) => {
            Content::PlainText { value: $value }
        };
    }

    fn parse_line(input: &String) -> ElementNode {
        let mut sharp_count: i8 = 0;
        for char in input.as_str().chars() {
            if char == '#' {
                sharp_count += 1;
            } else if char == ' ' {
                break;
            }
        }
        let content = if sharp_count == 0 {
            input
        } else {
            &input[(sharp_count as usize) + 1..]
        }
        .to_string();
        element_node! {
            tag: Token::value_of(sharp_count),
            content: content_plain_text!(inline_parse(&content)),
        }
    }

    fn parse(input: &Vec<String>) -> Vec<ElementNode> {
        let mut element_nodes: Vec<ElementNode> = vec![];
        let mut i: usize = 0;
        while i < input.len() {
            let list_index = i;
            if is_simple_list(input.get(i).unwrap()) {
                while is_simple_list(input.get(i).unwrap()) {
                    i += 1;
                }
                if list_index != i {
                    let parse_result =
                        parse_list(input[list_index..i].to_vec(), ListPattern::SimpleList, 0);
                    element_nodes.push(parse_result);
                    continue;
                }
            } else if is_number_list(input.get(i).unwrap()) {
                while is_number_list(input.get(i).unwrap()) {
                    i += 1;
                }
                if list_index != i {
                    let parse_result =
                        parse_list(input[list_index..i].to_vec(), ListPattern::NumberList, 0);
                    element_nodes.push(parse_result);
                    continue;
                }
            } else if is_quote_block(input.get(i).unwrap()) {
                let quote_start = i;
                while i < input.len() && input.get(i).unwrap() != "" {
                    i += 1;
                }
                let parse_result = parse(&enclose_quote(input[quote_start..i].to_vec()));
                element_nodes.push(element_node! {
                    tag: Token::Blockquote,
                    content: Content::ElementNodes { value: parse_result },
                });
                i += 1;
                continue;
            } else if is_code_block_start(input.get(i).unwrap()) {
                i += 1;
                let code_block_start = i;
                while !is_code_block_start(input.get(i).unwrap()) {
                    i += 1;
                }
                element_nodes.push(element_node! {
                    tag: Token::Code,
                    content: Content::PlainText{
                        value: parse_code_block(input[code_block_start..i].to_vec()).join("<br />"),
                    },
                });
                i += 1;
                continue;
            } else if is_table_block_start(input.get(i).unwrap()) {
                let (table, skip) = parse_table(input[i..].to_vec());
                i += skip;
                element_nodes.push(element_node! {
                    tag: Token::Table,
                    content: Content::Table {
                        value: table
                    },
                });
            }
            element_nodes.push(parse_line(input.get(i).unwrap()));
            i += 1;
        }
        element_nodes
    }

    pub struct Lexer {
        text: Vec<String>,
    }

    impl Lexer {
        pub fn new(text: Vec<String>) -> Self {
            Lexer { text }
        }

        pub fn parse(&self) -> MdNode {
            let result_str = parse(&self.text);
            MdNode::new(result_str)
        }
    }

    #[cfg(test)]
    mod test_lexer {
        use super::*;
        use crate::vec_string;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_parse() {
            let input = vec_string![
                "# hello",
                "## world",
                "* hogehoge",
                "* hogehoge1",
                "  * this is [Google先生](https://example.com)",
                "  * hogehoge3",
                "    * hoge 4",
                "* hogehoge4",
                "1. hoge1",
                "2. hoge2",
                "  1. aaa",
                "  2. ccc",
                "    1. ddd",
                "this is [Google先生](https://example.com)",
                "画像 ![エビフライトライアングル](https://example.com)",
                "> aaa",
                "bbb",
                ">> ccc",
                "ddd",
                "",
                "## world",
                "```html",
                r#"<script src="hoge.js"></script>"#,
                r#"<script src="hoge.js"></script>"#,
                "```",
                "this is `hoge` and `fuga`",
                "this is *hoge*",
                "this is **hoge**",
                "this is *hoge **fuga***",
                "|  head1  | head2 | head3|",
                "|:----:|-----:|:----- |",
                "|  aaa1  | bbb1 | ccc1|",
                "|  aaa2 | bbb2 | ccc2|",
                "aaa"
            ];
            let expected = MdNode::new(vec![
                element_node! {
                    tag: Token::H1,
                    content: content_plain_text!("hello".to_string()),
                },
                element_node! {
                    tag: Token::H2,
                    content: content_plain_text!("world".to_string()),
                },
                element_node! {
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
                                        content: content_plain_text!(r#"this is <a class="flav-md-a" href="https://example.com" alt="Google先生">Google先生</a>"#.to_string()),
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
                element_node! {
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
                element_node! {
                    tag: Token::P,
                    content: content_plain_text!(r#"this is <a class="flav-md-a" href="https://example.com" alt="Google先生">Google先生</a>"#.to_string()),
                },
                element_node! {
                    tag: Token::P,
                    content: content_plain_text!(r#"画像 <img class="flav-md-img" src="https://example.com" alt="エビフライトライアングル">"#.to_string()),
                },
                element_node! {
                    tag: Token::Blockquote,
                    content: content_element_nodes![
                        element_node! {
                            tag: Token::P,
                            content: content_plain_text!("aaa".to_string()),
                        },
                        element_node! {
                            tag: Token::P,
                            content: content_plain_text!("bbb".to_string()),
                        },
                        element_node! {
                            tag: Token::Blockquote,
                            content: content_element_nodes![
                                element_node! {
                                    tag: Token::P,
                                    content: content_plain_text!("ccc".to_string()),
                                },
                                element_node! {
                                    tag: Token::P,
                                    content: content_plain_text!("ddd".to_string()),
                                },
                            ],
                        },
                    ],
                },
                element_node! {
                    tag: Token::H2,
                    content: content_plain_text!("world".to_string()),
                },
                element_node! {
                    tag: Token::Code,
                    content: content_plain_text!("&lt;script src=&quot;hoge.js&quot;&gt;&lt;/script&gt;<br />&lt;script src=&quot;hoge.js&quot;&gt;&lt;/script&gt;".to_string()),
                },
                element_node! {
                    tag: Token::P,
                    content: content_plain_text!(r#"this is <code class="flav-md-code-inline">hoge</code> and <code class="flav-md-code-inline">fuga</code>"#.to_string()),
                },
                element_node! {
                    tag: Token::P,
                    content: content_plain_text!(r#"this is <em class="flav-md-em">hoge</em>"#.to_string()),
                },
                element_node! {
                    tag: Token::P,
                    content: content_plain_text!(r#"this is <strong class="flav-md-strong">hoge</strong>"#.to_string()),
                },
                element_node! {
                    tag: Token::P,
                    content: content_plain_text!(r#"this is <em class="flav-md-em">hoge <strong class="flav-md-strong">fuga</strong></em>"#.to_string()),
                },
                element_node! {
                    tag: Token::Table,
                    content: Content::Table {
                        value: table! {
                            head: vec![
                                TableHead::new("head1".to_string(), Align::Center),
                                TableHead::new("head2".to_string(), Align::Right),
                                TableHead::new("head3".to_string(), Align::Left),
                            ],
                            body: vec![
                                vec_string!["aaa1", "bbb1", "ccc1"],
                                vec_string!["aaa2", "bbb2", "ccc2"],
                            ],
                        },
                    }
                },
                element_node! {
                    tag: Token::P,
                    content: content_plain_text!("aaa".to_string()),
                },
            ]);
            let lex = Lexer::new(input);
            assert_eq!(lex.parse(), expected);
        }
    }
}
