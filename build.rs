use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct PostMeta {
    #[serde(default)]
    id: String,
    title: String,
    date: String,
    author: String,
    description: String,
    image_url: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    println!("cargo:rerun-if-changed=public/content/posts");
    println!("cargo:rerun-if-changed=public/content/projects");

    generate_index(
        "public/content/posts",
        "public/content/posts_index.json",
        parse_post_meta,
    );
    generate_index(
        "public/content/projects",
        "public/content/projects_index.json",
        parse_project_meta,
    );
}

fn generate_index<F, T>(dir_path: &str, output_path: &str, parser: F)
where
    F: Fn(&str, &str) -> Option<T>,
    T: Serialize,
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

    // Only write if content is different to prevent infinite build loops
    let should_write = match fs::read_to_string(output_path) {
        Ok(existing_content) => existing_content != json,
        Err(_) => true,
    };

    if should_write {
        fs::write(output_path, json).unwrap();
    }
}

fn parse_post_meta(content: &str, id: &str) -> Option<PostMeta> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }
    let yaml = parts[1];
    let mut meta: PostMeta = serde_yaml::from_str(yaml).ok()?;
    meta.id = id.to_string();

    // Resolve relative image_url
    if !meta.image_url.starts_with("http") && !meta.image_url.starts_with("/") {
        let clean_img = meta.image_url.trim_start_matches("./");
        meta.image_url = format!("content/posts/{}/{}", id, clean_img);
    }

    Some(meta)
}

fn parse_project_meta(content: &str, id: &str) -> Option<ProjectMeta> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }
    let yaml = parts[1];
    let mut meta: ProjectMeta = serde_yaml::from_str(yaml).ok()?;
    meta.id = id.to_string();

    // Resolve relative image_url
    if !meta.image_url.starts_with("http") && !meta.image_url.starts_with("/") {
        let clean_img = meta.image_url.trim_start_matches("./");
        meta.image_url = format!("content/projects/{}/{}", id, clean_img);
    }

    Some(meta)
}
