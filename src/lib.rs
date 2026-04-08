mod utils;

use wasm_bindgen::prelude::*;

use comrak::{markdown_to_html, Options};

#[wasm_bindgen]
pub fn render(s: &str) -> String {
    let mut options = Options::default();
    options.render.r#unsafe = true;
    let html = markdown_to_html(s, &options);
    ammonia::Builder::default()
        .add_tag_attributes("code", &["class"])
        .clean(&html)
        .to_string()
}
