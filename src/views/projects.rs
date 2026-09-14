//! # Projects Views Module
//!
//! Provides the workshop project portfolio catalog and detailed project showcase views.
//! Handles external project links, markdown content parsing, and social sharing metadata.

use crate::components::{
    Comments, ContentGallery, DetailHero, GalleryItem, RouteFactory, ShareButtons,
};
use crate::data::constants::APP_TITLE;
use crate::data::projects::{derive_categories, fetch_all_projects, get_project_by_id, Project};
use crate::data::utils::markdown_to_html;
use crate::hooks::{use_mermaid, use_syntax_highlighting};
use crate::Route;
use dioxus::prelude::*;

/// Resolves the canonical URL for project display images.
fn resolve_project_image(project: &Project) -> String {
    if project.meta.image_url.is_empty() {
        String::new()
    } else if project.meta.image_url.starts_with("http") {
        project.meta.image_url.clone()
    } else if project.meta.image_url.starts_with('/') {
        format!("https://imwoo90.github.io/the-rust-journey{}", project.meta.image_url)
    } else {
        format!(
            "https://imwoo90.github.io/the-rust-journey/content/projects/{}/{}",
            project.meta.id, project.meta.image_url
        )
    }
}

/// Renders the searchable portfolio catalog of built software and tools.
#[component]
pub fn ProjectList() -> Element {
    let projects_res = use_server_future(fetch_all_projects)?;
    let projects_guard = projects_res.read();
    let Some(projects) = projects_guard.as_ref() else {
        return rsx! { div { class: "flex justify-center items-center min-h-[50vh]", "Loading projects..." } };
    };

    let project_items = projects
        .iter()
        .map(|project| GalleryItem {
            id: project.id.clone(),
            title: project.title.clone(),
            description: project.description.clone(),
            image_url: project.image_url.clone(),
            tags: project.tags.clone(),
        })
        .collect();

    rsx! {
        ContentGallery {
            title: "The Workshop",
            subtitle: "Tangible milestones of my journey—a curated collection of tools, libraries, and applications forged along the road.",
            search_placeholder: "Search projects...",
            items: project_items,
            categories: derive_categories(projects),
            route_factory: RouteFactory(|id| Route::ProjectPost { id }),
        }
    }
}

/// Renders SEO, OpenGraph, and Twitter social cards metadata tags for projects.
#[component]
fn ProjectPostMeta(
    title: String,
    description: String,
    tags: Vec<String>,
    author: String,
    id: String,
    img_url: String,
) -> Element {
    rsx! {
        document::Title { "{title} - {APP_TITLE}" }
        document::Meta { name: "description", content: description.clone() }
        document::Meta { name: "keywords", content: tags.join(", ") }
        document::Meta { name: "author", content: author }
        document::Meta { property: "og:title", content: title.clone() }
        document::Meta { property: "og:description", content: description.clone() }
        document::Meta { property: "og:type", content: "article" }
        document::Meta { property: "og:url", content: format!("https://imwoo90.github.io/the-rust-journey/projects/{}", id) }
        if !img_url.is_empty() {
            document::Meta { property: "og:image", content: img_url.clone() }
        }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: title }
        document::Meta { name: "twitter:description", content: description }
        if !img_url.is_empty() {
            document::Meta { name: "twitter:image", content: img_url }
        }
    }
}

/// Renders the main project article layout, markdown presentation, launch link, and comments.
#[component]
fn ProjectPostContent(project: Project, html_content: String) -> Element {
    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
            article { class: "w-full max-w-3xl flex flex-col gap-10",
                DetailHero {
                    title: project.meta.title.clone(),
                    author: project.meta.author.clone(),
                    date: project.meta.date.clone(),
                    read_time: project.get_read_time(),
                    back_link: Route::ProjectList {},
                    back_label: "Projects".to_string(),
                }
                div {
                    class: "prose max-w-none dark:prose-invert",
                    dangerous_inner_html: "{html_content}",
                }
                ShareButtons { title: project.meta.title.clone() }
                ProjectExternalAction {
                    link: project.meta.link.clone(),
                    label: project.meta.link_text.clone(),
                }
                Comments {}
            }
        }
    }
}

/// Renders the 404 message when a project is not found.
#[component]
fn ProjectNotFoundView() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[60vh]",
            h1 { class: "text-4xl font-bold", "Project Not Found" }
            Link {
                to: Route::ProjectList {},
                class: "mt-4 text-primary-light hover:underline",
                "Back to Projects"
            }
        }
    }
}

/// Renders an individual project detail page with external links and documentation.
#[component]
pub fn ProjectPost(id: String) -> Element {
    let mut current_id = use_signal(|| id.clone());
    if current_id() != id {
        current_id.set(id.clone());
    }

    let project_res = use_server_future(move || {
        let id = current_id();
        async move { get_project_by_id(&id).await }
    })?;

    use_syntax_highlighting();
    use_mermaid();

    let projects_guard = project_res.read();
    let Some(project_opt) = projects_guard.as_ref() else {
        return rsx! { div { class: "flex justify-center items-center min-h-[50vh]", "Loading project..." } };
    };

    match project_opt {
        Some(project) => {
            let html_content = markdown_to_html(&project.content, &project.meta.id, "projects");
            let img_url = resolve_project_image(project);

            rsx! {
                ProjectPostMeta {
                    title: project.meta.title.clone(),
                    description: project.meta.description.clone(),
                    tags: project.meta.tags.clone(),
                    author: project.meta.author.clone(),
                    id: project.meta.id.clone(),
                    img_url,
                }
                ProjectPostContent { project: project.clone(), html_content }
            }
        }
        None => rsx! { ProjectNotFoundView {} },
    }
}

/// Renders the external project launch action button.
#[component]
fn ProjectExternalAction(link: Option<String>, label: Option<String>) -> Element {
    let Some(target_url) = link else {
        return rsx! { "" };
    };

    let button_text = label.unwrap_or_else(|| "Visit Project".to_string());

    rsx! {
        div { class: "mt-8",
            a {
                href: "{target_url}",
                class: "inline-flex items-center gap-2 bg-primary-light text-text-dark px-6 py-3 rounded-lg font-bold hover:opacity-90 transition-all shadow-md active:scale-95",
                target: "_blank",
                span { class: "material-symbols-outlined", "launch" }
                "{button_text}"
            }
        }
    }
}
