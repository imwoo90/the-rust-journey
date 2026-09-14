//! # Shared Data Utilities and Markdown Processing
//!
//! ## Overview
//! Provides core parsing algorithms for frontmatter metadata, read-time calculation,
//! subpath base-URL resolution, and AST-level markdown transformation via pulldown-cmark.
//!
//! ## Search Tags
//! #utils, #markdown, #frontmatter, #base-path, #read-time

use pulldown_cmark::{html, Event, Options, Parser, Tag};
use serde::{Deserialize, Serialize};

/// Returns the runtime base URL prefix for asset and routing resolution.
pub fn get_base_path() -> String {
    if let Some(base_path) = dioxus_cli_config::base_path() {
        let base_path = base_path.trim_matches('/');
        if base_path.is_empty() {
            "".to_string()
        } else {
            format!("/{}", base_path)
        }
    } else {
        "".to_string()
    }
}

/// Generic common frontmatter structure used across posts and projects.
#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CommonMeta {
    /// Unique item slug identifier.
    #[serde(default)]
    pub id: String,
    /// Title string.
    pub title: String,
    /// Publication date string.
    pub date: String,
    /// Author attribution name.
    pub author: String,
    /// Summary description text.
    pub description: String,
    /// Thumbnail image link.
    pub image_url: String,
    /// Classification keyword tags.
    pub tags: Vec<String>,
}

/// Parses frontmatter YAML and body markdown from a raw string.
pub fn parse_frontmatter<T: for<'de> Deserialize<'de>>(content: &str) -> Result<(T, &str), String> {
    if !content.starts_with("---") {
        return Err("No frontmatter found".to_string());
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format".to_string());
    }

    let yaml = parts[1];
    let markdown = parts[2];

    let meta: T = serde_yaml::from_str(yaml).map_err(|e| e.to_string())?;

    Ok((meta, markdown.trim()))
}

/// Calculates estimated reading time assuming an average reading speed of 200 wpm.
pub fn get_read_time(content: &str) -> String {
    let words = content.split_whitespace().count();
    let minutes = (words as f32 / 200.0).ceil() as u32;
    if minutes <= 1 {
        "1 min read".to_string()
    } else {
        format!("{} min read", minutes)
    }
}

/// Resolves link URLs to preserve site base path and relative content locations.
fn resolve_link_dest(dest_url: &str, post_id: &str, content_type: &str) -> String {
    if dest_url.starts_with('/') {
        format!("{}{}", get_base_path(), dest_url)
    } else if !dest_url.starts_with("http") {
        let clean_path = dest_url.trim_start_matches("./");
        format!(
            "{}/content/{}/{}/{}",
            get_base_path(),
            content_type,
            post_id,
            clean_path
        )
    } else {
        dest_url.to_string()
    }
}

/// Resolves image URLs ensuring relative asset references resolve under public content paths.
fn resolve_image_dest(dest_url: &str, post_id: &str, content_type: &str) -> String {
    if dest_url.starts_with('/') {
        format!("{}{}", get_base_path(), dest_url)
    } else if !dest_url.starts_with("http") {
        let clean_path = dest_url.trim_start_matches("./");
        format!(
            "{}/content/{}/{}/{}",
            get_base_path(),
            content_type,
            post_id,
            clean_path
        )
    } else {
        dest_url.to_string()
    }
}

/// Transforms markdown text into rendered HTML with rewritten relative link and asset URLs.
pub fn markdown_to_html(markdown: &str, post_id: &str, content_type: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let new_dest = resolve_link_dest(&dest_url, post_id, content_type);
            Event::Start(Tag::Link {
                link_type,
                dest_url: new_dest.into(),
                title,
                id,
            })
        }
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let new_dest = resolve_image_dest(&dest_url, post_id, content_type);
            Event::Start(Tag::Image {
                link_type,
                dest_url: new_dest.into(),
                title,
                id,
            })
        }
        _ => event,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
