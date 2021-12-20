pub mod inline {
    use once_cell::sync::Lazy;
    use regex::Regex;

    pub struct Pattern {
        pattern: Regex,
        template: String,
    }

    impl Pattern {
        pub fn new(pattern: Regex, template: String) -> Self {
            Pattern { pattern, template }
        }

        pub fn parse(&self, input: &String) -> String {
            self.pattern.replace_all(input, &self.template).to_string()
        }
    }

    static IMAGE_PARSE: Lazy<Pattern> = Lazy::new(|| {
        Pattern::new(
            Regex::new(r"!\[(.*?)]\((.*?)\)").unwrap(),
            r#"<img class="flav-md-img" src="$2" alt="$1">"#.to_string(),
        )
    });

    static LINK_PARSE: Lazy<Pattern> = Lazy::new(|| {
        Pattern::new(
            Regex::new(r"\[(.*?)]\((.*?)\)").unwrap(),
            r#"<a class="flav-md-a" href="$2" alt="$1">$1</a>"#.to_string(),
        )
    });

    static CODE_PARSE: Lazy<Pattern> = Lazy::new(|| {
        Pattern::new(
            Regex::new(r"`(.*?)`").unwrap(),
            r#"<code class="flav-md-code-inline">$1</code>"#.to_string(),
        )
    });

    static STRONG_PARSE: Lazy<Pattern> = Lazy::new(|| {
        Pattern::new(
            Regex::new(r"\*{2}(.*?)\*{2}").unwrap(),
            r#"<strong class="flav-md-strong">$1</strong>"#.to_string(),
        )
    });

    static EMPHASIS_PARSE: Lazy<Pattern> = Lazy::new(|| {
        Pattern::new(
            Regex::new(r"\*(.*?)\*").unwrap(),
            r#"<em class="flav-md-em">$1</em>"#.to_string(),
        )
    });

    trait Parsable {
        fn parse(&self, parser: &Pattern) -> Self;
    }

    impl Parsable for String {
        fn parse(&self, parser: &Pattern) -> Self {
            parser.parse(self)
        }
    }

    pub fn inline_parse(input: &String) -> String {
        input
            .parse(&IMAGE_PARSE)
            .parse(&LINK_PARSE)
            .parse(&CODE_PARSE)
            .parse(&STRONG_PARSE)
            .parse(&EMPHASIS_PARSE)
    }

    #[cfg(test)]
    mod test_inline {
        use super::*;

        #[test]
        fn test_image_pattern() {
            let output = inline_parse(&String::from("![hoge1](hoge2)"));
            assert_eq!(
                output,
                String::from(r#"<img class="flav-md-img" src="hoge2" alt="hoge1">"#)
            );
        }

        #[test]
        fn test_link_pattern() {
            let output = inline_parse(&String::from("[hoge1](hoge2)"));
            assert_eq!(
                output,
                String::from(r#"<a class="flav-md-a" href="hoge2" alt="hoge1">hoge1</a>"#)
            );
        }

        #[test]
        fn test_code_pattern() {
            let output = inline_parse(&String::from("`hoge`"));
            assert_eq!(
                output,
                String::from(r#"<code class="flav-md-code-inline">hoge</code>"#)
            );
        }

        #[test]
        fn test_strong_pattern() {
            let output = inline_parse(&String::from("**hoge**"));
            assert_eq!(
                output,
                String::from(r#"<strong class="flav-md-strong">hoge</strong>"#)
            );
        }

        #[test]
        fn test_emphasis_pattern() {
            let output = inline_parse(&String::from("*hoge*"));
            assert_eq!(output, String::from(r#"<em class="flav-md-em">hoge</em>"#));
        }

        #[test]
        fn test_inline_parse() {
            let output = inline_parse(&String::from("*hoge*"));
            assert_eq!(output, String::from(r#"<em class="flav-md-em">hoge</em>"#));
        }
    }
}
