use leptos::prelude::*;
use std::fs;

pub fn parse_markdown(md: String) -> impl IntoView {
    let parser = pulldown_cmark::Parser::new(&md);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    view! { <div inner_html=html_output></div> }
}

pub fn read_markdown_from_file(md_path: String) -> String {
    fs::read_to_string(md_path).expect("Unable to read the file")
}
