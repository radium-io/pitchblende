mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use ammonia;
use comrak::{markdown_to_html, ComrakOptions};

#[wasm_bindgen]
pub fn render(s: &str) -> String {
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    let html = markdown_to_html(s, &options);
    ammonia::Builder::default()
        .add_tag_attributes("code", &["class"])
        .clean(&html)
        .to_string()
}
