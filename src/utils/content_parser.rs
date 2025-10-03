use leptos::prelude::*;
use std::fs;

#[server]
pub async fn parse_markdown(
    md: String,
    number_of_chars: Option<usize>,
) -> Result<String, ServerFnError> {
    let parser = pulldown_cmark::Parser::new(&md);
    let mut html_output = String::new();

    pulldown_cmark::html::push_html(&mut html_output, parser);

    if let Some(length) = number_of_chars {
        Ok(html_output[0..length].to_string())
    } else {
        Ok(html_output)
    }
}

pub fn read_markdown_from_file(md_path: String) -> String {
    fs::read_to_string(md_path).expect("Unable to read the file")
}
