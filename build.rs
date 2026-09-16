//! Build script for The Rust Journey.
//!
//! Generates metadata indices for blog posts and projects, and runs the Agent-Native linter.

#![allow(missing_docs, clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[path = "build_linter.rs"]
mod build_linter;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct PostMeta {
    #[serde(default)]
    id: String,
    title: String,
    date: String,
    author: String,
    description: String,
    image_url: String,
    tags: Vec<String>,
    series: Option<String>,
    series_order: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProjectMeta {
    #[serde(default)]
    id: String,
    title: String,
    date: String,
    author: String,
    description: String,
    image_url: String,
    tags: Vec<String>,
    link: Option<String>,
    link_text: Option<String>,
    route: Option<String>,
}

fn main() {
    build_linter::run_linter();

    println!("cargo:rerun-if-changed=public/content/posts");
    println!("cargo:rerun-if-changed=public/content/projects");

    let posts = generate_index(
        "public/content/posts",
        "public/content/posts_index.json",
        parse_post_meta,
    );
    let projects = generate_index(
        "public/content/projects",
        "public/content/projects_index.json",
        parse_project_meta,
    );

    generate_sitemap(&posts, &projects);
    generate_robots();
}

fn generate_index<F, T>(dir_path: &str, output_path: &str, parser: F) -> Vec<T>
where
    F: Fn(&str, &str) -> Option<T>,
    T: Serialize + Clone,
{
    let mut items = Vec::new();
    let dir = Path::new(dir_path);

    if dir.exists() && dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let id = path.file_name().unwrap().to_str().unwrap().to_string();
                let md_path = path.join("index.md");
                if md_path.exists() {
                    let content = fs::read_to_string(md_path).unwrap();
                    if let Some(item) = parser(&content, &id) {
                        items.push(item);
                    }
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&items).unwrap();

    let should_write = match fs::read_to_string(output_path) {
        Ok(existing_content) => existing_content != json,
        Err(_) => true,
    };

    if should_write {
        fs::write(output_path, json).unwrap();
    }

    items
}

fn generate_sitemap(posts: &[PostMeta], projects: &[ProjectMeta]) {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    let base = "https://imwoo90.github.io/the-rust-journey";
    let static_routes = [
        ("/", "1.0", "daily"),
        ("/blog", "0.9", "daily"),
        ("/projects", "0.9", "weekly"),
        ("/about", "0.7", "monthly"),
        ("/contact", "0.6", "monthly"),
    ];

    for (route, priority, freq) in static_routes {
        xml.push_str(&format!(
            "  <url>\n    <loc>{base}{route}</loc>\n    <changefreq>{freq}</changefreq>\n    <priority>{priority}</priority>\n  </url>\n"
        ));
    }

    for post in posts {
        xml.push_str(&format!(
            "  <url>\n    <loc>{base}/blog/{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            post.id, post.date
        ));
    }

    for project in projects {
        xml.push_str(&format!(
            "  <url>\n    <loc>{base}/projects/{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            project.id, project.date
        ));
    }

    xml.push_str("</urlset>\n");

    let output_path = "public/sitemap.xml";
    let should_write = match fs::read_to_string(output_path) {
        Ok(existing) => existing != xml,
        Err(_) => true,
    };
    if should_write {
        fs::write(output_path, xml).unwrap();
    }
}

fn generate_robots() {
    let robots = "User-agent: *\nAllow: /\n\nSitemap: https://imwoo90.github.io/the-rust-journey/sitemap.xml\n";
    let output_path = "public/robots.txt";
    let should_write = match fs::read_to_string(output_path) {
        Ok(existing) => existing != robots,
        Err(_) => true,
    };
    if should_write {
        fs::write(output_path, robots).unwrap();
    }
}

trait Metadata {
    fn set_id(&mut self, id: String);
    fn get_image_url(&self) -> &str;
    fn set_image_url(&mut self, url: String);
}

impl Metadata for PostMeta {
    fn set_id(&mut self, id: String) {
        self.id = id;
    }
    fn get_image_url(&self) -> &str {
        &self.image_url
    }
    fn set_image_url(&mut self, url: String) {
        self.image_url = url;
    }
}

impl Metadata for ProjectMeta {
    fn set_id(&mut self, id: String) {
        self.id = id;
    }
    fn get_image_url(&self) -> &str {
        &self.image_url
    }
    fn set_image_url(&mut self, url: String) {
        self.image_url = url;
    }
}

fn parse_meta<T: Metadata + for<'de> Deserialize<'de>>(
    content: &str,
    id: &str,
    folder: &str,
) -> Option<T> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }
    let mut meta: T = serde_yaml::from_str(parts[1]).ok()?;

    meta.set_id(id.to_string());

    let img = meta.get_image_url().to_string();
    if !img.starts_with("http") && !img.starts_with("/") {
        let clean_img = img.trim_start_matches("./");
        meta.set_image_url(format!("content/{}/{}/{}", folder, id, clean_img));
    }

    Some(meta)
}

fn parse_post_meta(content: &str, id: &str) -> Option<PostMeta> {
    parse_meta(content, id, "posts")
}

fn parse_project_meta(content: &str, id: &str) -> Option<ProjectMeta> {
    parse_meta(content, id, "projects")
}
