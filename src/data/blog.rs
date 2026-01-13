use crate::data::utils::{get_base_path, parse_frontmatter};
use serde::{Deserialize, Serialize};

/// Metadata for a blog post.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PostMeta {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
}

/// A complete blog post including metadata and markdown content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub meta: PostMeta,
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
    let url = format!("{}/content/posts_index.json", get_base_path());
    let mut posts: Vec<PostMeta> = match gloo_net::http::Request::get(&url).send().await {
        Ok(resp) => resp.json().await.unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    // Sort by date descending
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
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
    let url = format!("{}/content/posts/{}/index.md", get_base_path(), id);
    let content = match gloo_net::http::Request::get(&url).send().await {
        Ok(resp) => resp.text().await.ok()?,
        Err(_) => return None,
    };

    parse_markdown(&content, id.to_string()).ok()
}

fn parse_markdown(content: &str, id: String) -> Result<Post, String> {
    let (mut meta, markdown): (PostMeta, &str) = parse_frontmatter(content)?;
    meta.id = id;

    Ok(Post {
        meta,
        content: markdown.to_string(),
    })
}
