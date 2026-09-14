//! # Blog Post Data Model and Asynchronous Ingestion
//!
//! Exposes structures and loaders for blog articles, post metadata, and category derivation.
//! Supports runtime WASM HTTP fetching via gloo-net and non-WASM filesystem reads for SSG.

#[allow(unused_imports)]
use crate::data::utils::{get_base_path, parse_frontmatter};
use serde::{Deserialize, Serialize};

/// Metadata for a blog post.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PostMeta {
    /// Unique URL slug identifier.
    #[serde(default)]
    pub id: String,
    /// Article title.
    pub title: String,
    /// Publication date in ISO-8601 or YYYY-MM-DD format.
    pub date: String,
    /// Author display name.
    pub author: String,
    /// Short summary description for cards and SEO tags.
    pub description: String,
    /// Primary thumbnail image URL or relative path.
    pub image_url: String,
    /// Classification keyword tags.
    pub tags: Vec<String>,
    /// Optional parent series name.
    pub series: Option<String>,
    /// Optional index order within the parent series.
    pub series_order: Option<i32>,
}

/// A complete blog post including metadata and markdown content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    /// Frontmatter metadata.
    pub meta: PostMeta,
    /// Raw Markdown body content.
    pub content: String,
}

impl Post {
    /// Calculates the estimated read time for the post content.
    pub fn get_read_time(&self) -> String {
        crate::data::utils::get_read_time(&self.content)
    }
}

/// Fetches all blog posts metadata from the server, sorted by date descending.
pub async fn fetch_all_posts() -> Vec<PostMeta> {
    #[cfg(target_arch = "wasm32")]
    {
        let url = format!("{}/content/posts_index.json", get_base_path());
        let mut posts: Vec<PostMeta> = match gloo_net::http::Request::get(&url).send().await {
            Ok(resp) => resp.json().await.unwrap_or_default(),
            Err(_) => Vec::new(),
        };
        posts.sort_by(|a, b| b.date.cmp(&a.date));
        posts
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let index_path = manifest_dir.join("public/content/posts_index.json");
        if index_path.exists() {
            let content = std::fs::read_to_string(index_path).unwrap_or_default();
            let mut posts: Vec<PostMeta> = serde_json::from_str(&content).unwrap_or_default();
            posts.sort_by(|a, b| b.date.cmp(&a.date));
            posts
        } else {
            Vec::new()
        }
    }
}

/// Derives unique categories from a list of posts.
pub fn derive_categories(posts: &[PostMeta]) -> Vec<String> {
    let mut categories = std::collections::HashSet::new();
    for post in posts {
        for tag in &post.tags {
            categories.insert(tag.clone());
        }
    }
    let mut categories: Vec<String> = categories.into_iter().collect();
    categories.sort();
    categories
}

/// Fetches a specific blog post by its ID from the server.
pub async fn get_post_by_id(id: &str) -> Option<Post> {
    #[cfg(target_arch = "wasm32")]
    {
        let url = format!("{}/content/posts/{}/index.md", get_base_path(), id);
        let content = match gloo_net::http::Request::get(&url).send().await {
            Ok(resp) => resp.text().await.ok()?,
            Err(_) => return None,
        };

        parse_markdown(&content, id.to_string()).ok()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = manifest_dir.join(format!("public/content/posts/{}/index.md", id));
        if path.exists() {
            let content = std::fs::read_to_string(path).ok()?;
            parse_markdown(&content, id.to_string()).ok()
        } else {
            None
        }
    }
}

fn parse_markdown(content: &str, id: String) -> Result<Post, String> {
    let (mut meta, markdown): (PostMeta, &str) = parse_frontmatter(content)?;
    meta.id = id;

    Ok(Post {
        meta,
        content: markdown.to_string(),
    })
}
