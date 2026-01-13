use crate::data::utils::{get_base_path, parse_frontmatter};
use serde::{Deserialize, Serialize};

/// Metadata for a project, parsed from Markdown frontmatter.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectMeta {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
    pub link: Option<String>,
    pub link_text: Option<String>,
    pub route: Option<String>,
}

/// A complete project entry including metadata and markdown content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub meta: ProjectMeta,
    pub content: String,
}

impl Project {
    /// Calculates the estimated read time for the project content.
    pub fn get_read_time(&self) -> String {
        crate::data::utils::get_read_time(&self.content)
    }
}

/// Fetches all projects metadata from the server, sorted by date descending.
pub async fn fetch_all_projects() -> Vec<ProjectMeta> {
    let url = format!("{}/content/projects_index.json", get_base_path());
    let mut projects: Vec<ProjectMeta> = match gloo_net::http::Request::get(&url).send().await {
        Ok(resp) => resp.json().await.unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    // Sort by date descending
    projects.sort_by(|a, b| b.date.cmp(&a.date));
    projects
}

/// Derives unique categories from a list of projects.
pub fn derive_categories(projects: &[ProjectMeta]) -> Vec<String> {
    let mut categories = std::collections::HashSet::new();
    for project in projects {
        for tag in &project.tags {
            categories.insert(tag.clone());
        }
    }
    let mut categories: Vec<String> = categories.into_iter().collect();
    categories.sort();
    categories
}

/// Fetches a specific project by its ID from the server.
pub async fn get_project_by_id(id: &str) -> Option<Project> {
    let url = format!("{}/content/projects/{}/index.md", get_base_path(), id);
    let content = match gloo_net::http::Request::get(&url).send().await {
        Ok(resp) => resp.text().await.ok()?,
        Err(_) => return None,
    };

    parse_project_full(&content, id.to_string()).ok()
}

fn parse_project_full(content: &str, id: String) -> Result<Project, String> {
    let (mut meta, markdown): (ProjectMeta, &str) = parse_frontmatter(content)?;
    meta.id = id;

    Ok(Project {
        meta,
        content: markdown.to_string(),
    })
}
