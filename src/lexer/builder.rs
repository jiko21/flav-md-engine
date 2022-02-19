pub mod builder {
    use crate::lexer::lexer::lexer::Table;
    use crate::lexer::lexer::lexer::{Content, ElementNode, TableHead, Token};
    use std::borrow::Borrow;

    fn generate_class_for_the_tag(tag: &Token) -> Vec<String> {
        let mut tags = vec!["flav-md-text".to_string()];
        match tag {
            Token::H1 | Token::H2 | Token::H3 | Token::H4 | Token::H5 | Token::H6 => {
                tags.push(format!("flav-md-{}", tag.value()));
                tags.push("flav-md-h".to_string());
            }
            Token::P => {
                tags.push("flav-md-p".to_string());
            }
            Token::Blockquote => {
                tags.push("flav-md-blockquote".to_string());
            }
            _ => {}
        }
        tags
    }

    fn create_thead(head: &Vec<TableHead>) -> String {
        let heads = head
            .iter()
            .map(|item| {
                format!(
                    "      <th style=\"text-align: {}\">{}</th>",
                    item.get_align(),
                    item.cell
                )
                .to_string()
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            r#"  <thead>
    <tr>
{}
    </tr>
  </thead>"#,
            heads
        )
        .to_string()
    }

    fn create_tbody(head: &Vec<TableHead>, body: &Vec<Vec<String>>) -> String {
        let trs = body
            .iter()
            .map(|rows| {
                let tds = rows
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        format!(
                            "      <td style=\"text-align: {}\">{}</td>",
                            head.get(i).unwrap().get_align(),
                            item
                        )
                        .to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                format!(
                    r#"    <tr>
{}
    </tr>"#,
                    tds
                )
                .to_string()
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            r#"  <tbody>
{}
  </tbody>"#,
            trs
        )
        .to_string()
    }

    fn generate_table(content: &Table) -> String {
        let head = &content.head;
        let body = &content.body;
        let thead = create_thead(&head);
        let tbody = create_tbody(&head, body);
        format!(
            r#"<table>
{}
{}
</table>"#,
            thead, tbody
        )
    }

    fn parse_nested_tag(items: &Vec<ElementNode>, indent: usize) -> String {
        let mut results = "".to_string();
        for item in items.into_iter() {
            results += &format!("{}\n", create_tag(&item, indent));
        }
        results
    }

    fn create_tag(item: &ElementNode, indent: usize) -> String {
        match item {
            ElementNode::Exist {
                tag,
                content,
                children,
            } => {
                let classes = generate_class_for_the_tag(&tag);
                let whiltespace = " ".repeat(indent);
                match tag {
                    Token::Ul => match &**content {
                        Content::ElementNodes { value } => {
                            let content = parse_nested_tag(value.borrow(), indent + 2);
                            format!(
                                r#"<ul class="flav-md-ul">
{}{}</ul>"#,
                                content,
                                " ".repeat(indent)
                            )
                        }
                        _ => "".to_string(),
                    },
                    Token::Ol => match &**content {
                        Content::ElementNodes { value } => {
                            let content = parse_nested_tag(&value, indent + 2);
                            format!(
                                r#"<ol class="flav-md-ol">
{}{}</ol>"#,
                                content,
                                " ".repeat(indent)
                            )
                        }
                        _ => "".to_string(),
                    },
                    Token::Li => {
                        let content = match &**content {
                            Content::PlainText { value } => value,
                            _ => "",
                        };
                        match **children {
                            ElementNode::Exist { .. } => {
                                let c = format!(
                                    "{}{}",
                                    " ".repeat(indent + 2),
                                    create_tag(&*children, indent + 2)
                                );
                                format!(
                                    r#"{}<li class="flav-md-text flav-md-li">{}
{}
{}</li>"#,
                                    whiltespace, content, c, whiltespace
                                )
                            }
                            _ => {
                                format!(
                                    "{}<li class=\"flav-md-text flav-md-li\">{}</li>",
                                    " ".repeat(indent),
                                    content,
                                )
                            }
                        }
                    }
                    Token::Blockquote => match &**content {
                        Content::ElementNodes { value } => {
                            format!(
                                r#"{}<blockquote class="{}">
{}{}</blockquote>"#,
                                whiltespace,
                                classes.join(" "),
                                parse_nested_tag(&value, indent + 2),
                                whiltespace
                            )
                        }
                        _ => "".to_string(),
                    },
                    Token::Code => match &**content {
                        Content::PlainText { value } => {
                            format!(
                                r#"<code class="flav-md-code">
  {}
</code>"#,
                                value
                            )
                        }
                        _ => "".to_string(),
                    },
                    Token::Table => match &**content {
                        Content::Table { value } => generate_table(&value),
                        _ => "".to_string(),
                    },
                    _ => {
                        let content = match &**content {
                            Content::PlainText { value } => value,
                            _ => "",
                        };
                        let tag = tag.value();
                        format!(
                            r#"{}<{} class="{}">{}</{}>"#,
                            whiltespace,
                            tag,
                            classes.join(" "),
                            content,
                            tag
                        )
                    }
                }
            }
            _ => "".to_string(),
        }
        .to_string()
    }

    #[derive(Debug, PartialEq)]
    pub struct MdNode {
        element_nodes: Vec<ElementNode>,
    }

    impl MdNode {
        pub fn new(element_nodes: Vec<ElementNode>) -> Self {
            MdNode { element_nodes }
        }

        pub fn to_html_string(&self) -> String {
            self.element_nodes
                .iter()
                .map(|i| create_tag(&i, 0))
                .collect::<Vec<String>>()
                .join("\n")
        }
    }

    #[cfg(test)]
    mod test_builder {
        use super::*;
        use crate::lexer::lexer::lexer::{Align, Content, Table, TableHead, Token};
        use crate::vec_string;
        use crate::{content_element_nodes, content_plain_text, element_node, table};
        use pretty_assertions::assert_eq;

        #[test]
        fn test_is_code_block_start() {
            let expected = r#"<h1 class="flav-md-text flav-md-h1 flav-md-h">hello</h1>
<h2 class="flav-md-text flav-md-h2 flav-md-h">world</h2>
<ul class="flav-md-ul">
  <li class="flav-md-text flav-md-li">hogehoge</li>
  <li class="flav-md-text flav-md-li">hogehoge1
    <ul class="flav-md-ul">
      <li class="flav-md-text flav-md-li">this is <a class="flav-md-a" href="https://example.com" alt="Google先生">Google先生</a></li>
      <li class="flav-md-text flav-md-li">hogehoge3
        <ul class="flav-md-ul">
          <li class="flav-md-text flav-md-li">hoge 4</li>
        </ul>
      </li>
    </ul>
  </li>
  <li class="flav-md-text flav-md-li">hogehoge4</li>
</ul>
<ol class="flav-md-ol">
  <li class="flav-md-text flav-md-li">hoge1</li>
  <li class="flav-md-text flav-md-li">hoge2
    <ol class="flav-md-ol">
      <li class="flav-md-text flav-md-li">aaa</li>
      <li class="flav-md-text flav-md-li">ccc
        <ol class="flav-md-ol">
          <li class="flav-md-text flav-md-li">ddd</li>
        </ol>
      </li>
    </ol>
  </li>
</ol>
<p class="flav-md-text flav-md-p">this is <a class="flav-md-a" href="https://example.com" alt="Google先生">Google先生</a></p>
<p class="flav-md-text flav-md-p">画像 <img class="flav-md-img" src="https://example.com" alt="エビフライトライアングル"></p>
<blockquote class="flav-md-text flav-md-blockquote">
  <p class="flav-md-text flav-md-p">aaa</p>
  <p class="flav-md-text flav-md-p">bbb</p>
  <blockquote class="flav-md-text flav-md-blockquote">
    <p class="flav-md-text flav-md-p">ccc</p>
    <p class="flav-md-text flav-md-p">ddd</p>
  </blockquote>
</blockquote>
<h2 class="flav-md-text flav-md-h2 flav-md-h">world</h2>
<code class="flav-md-code">
  &lt;script src=&quot;hoge.js&quot;&gt;&lt;/script&gt;<br />&lt;script src=&quot;hoge.js&quot;&gt;&lt;/script&gt;
</code>
<p class="flav-md-text flav-md-p">this is <code class="flav-md-code-inline">hoge</code> and <code class="flav-md-code-inline">fuga</code></p>
<p class="flav-md-text flav-md-p">this is <em class="flav-md-em">hoge</em></p>
<p class="flav-md-text flav-md-p">this is <strong class="flav-md-strong">hoge</strong></p>
<p class="flav-md-text flav-md-p">this is <em class="flav-md-em">hoge <strong class="flav-md-strong">fuga</strong></em></p>
<table>
  <thead>
    <tr>
      <th style="text-align: center">head1</th>
      <th style="text-align: right">head2</th>
      <th style="text-align: left">head3</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td style="text-align: center">aaa1</td>
      <td style="text-align: right">bbb1</td>
      <td style="text-align: left">ccc1</td>
    </tr>
    <tr>
      <td style="text-align: center">aaa2</td>
      <td style="text-align: right">bbb2</td>
      <td style="text-align: left">ccc2</td>
    </tr>
  </tbody>
</table>
<p class="flav-md-text flav-md-p">aaa</p>"#;
            let md_node = MdNode::new(vec![
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
            let actual = md_node.to_html_string();
            assert_eq!(actual, expected);
        }
    }
}
