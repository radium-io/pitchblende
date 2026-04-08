mod utils;

use wasm_bindgen::prelude::*;

use comrak::{markdown_to_html, Options};

#[wasm_bindgen]
pub fn render(s: &str) -> String {
    let mut options = Options::default();
    options.render.r#unsafe = true;
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_id_prefix = Some(String::new());
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = Some("---".to_string());
    options.extension.multiline_block_quotes = true;
    options.extension.math_dollars = true;
    options.extension.math_code = true;
    options.extension.shortcodes = true;
    options.extension.wikilinks_title_after_pipe = true;
    options.extension.wikilinks_title_before_pipe = true;
    options.extension.underline = true;
    options.extension.subscript = true;
    options.extension.spoiler = true;
    options.extension.greentext = true;
    let html = markdown_to_html(s, &options);
    ammonia::Builder::default()
        .add_tags(&["table", "thead", "tbody", "tr", "th", "td"])
        .add_tag_attributes("code", &["class"])
        .clean(&html)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |\n";
        let out = render(md);
        println!("table output:\n{}", out);
        assert!(out.contains("<table>"), "no table tag found in: {}", out);
    }

    #[test]
    fn test_table_with_code() {
        let md = "| Scale | Font Size |\n|-------|----------|\n| `prose-sm` | 0.875rem |\n| `prose-base` | 1rem |\n";
        let out = render(md);
        println!("table+code output:\n{}", out);
        assert!(out.contains("<table>"), "no table tag found in: {}", out);
    }
}
