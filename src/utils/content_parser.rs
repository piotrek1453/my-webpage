use crate::components::common::content::Image;
use leptos::prelude::*;
use std::fs;

pub fn parse_markdown_with_images(md: &str) -> impl IntoView {
    md.lines()
        .map(|line| {
            if let Some(path) = line
                .strip_prefix("[image:")
                .and_then(|s| s.strip_suffix("]"))
            {
                view! { <Image image_path=path /> }
            } else {
                view! { <p>{line}</p> }
            }
        })
        .collect_view()
}

pub fn read_markdown_from_file(md_path: &str) -> String {
    fs::read_to_string(md_path).expect("Unable to read the file")
}
