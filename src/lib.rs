//! This is rust implementation of [flav-md](https://github.com/jiko21/flav-md)
//!
//! You can parse markdown docs to styled html.
//!
use crate::lexer::lexer::lexer::Lexer;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod lexer;
mod util;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Struct for flav md engine
#[wasm_bindgen]
pub struct FlavMd {
    /// Rendered html by flav md engine
    html_text: String,
    /// Style for flav md docs
    ///
    /// This param is  passed by [`FlavMd::build`]
    css_text: String,
}

/// impl for flav md engine
#[wasm_bindgen]
impl FlavMd {
    /// Generate new flav md engine instance.
    ///
    /// In rust (not WebAssembly), using this function is recommended.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        FlavMd {
            html_text: "".to_string(),
            css_text: "".to_string(),
        }
    }

    /// Build html text with given markdown and css.
    pub fn build(&mut self, md_text: String, css_text: String) -> String {
        let lexer = Lexer::new(md_text.split("\n").map(|s| s.to_string()).collect());
        self.html_text = lexer.parse().to_html_string();
        self.css_text = css_text;
        format!("<style>{}</style>\n{}", self.css_text, self.html_text)
    }
}

/// Generate flav md instance
///
/// This is mainly for WebAssembly, so if you use in rust project,
/// consider using [`FlavMd::new()`]
///
/// # example
///
/// ```
/// use flav_md_engine::create_flav_md;
///
/// create_flav_md()
///     .build(md_text, css_text);
/// ```
#[wasm_bindgen]
pub fn create_flav_md() -> FlavMd {
    FlavMd::new()
}

#[cfg(test)]
mod test {
    use crate::create_flav_md;

    #[test]
    fn correctly_build_file() {
        let html_text = "<h1 class=\"flav-md-text flav-md-h1 flav-md-h\">sample</h1>".to_string();
        let css_text = r#".flav-md-h1 {
  color: red;
}"#
        .to_string();
        let expected = format!("<style>{}</style>\n{}", css_text, html_text);
        let actual = create_flav_md().build("# sample".to_string(), css_text);
        assert_eq!(actual, expected);
    }
}
