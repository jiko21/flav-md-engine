pub mod lexer {
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
}
