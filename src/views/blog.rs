//! # Blog Views Module
//!
//! ## Overview
//! Provides the blog catalog and detailed individual article reading views.
//! Handles syntax highlighting hooks, mermaid rendering, SEO metadata, and series navigation.
//!
//! ## Search Tags
//! #blog, #article, #reading-view, #series, #markdown-view

use crate::components::{
    Comments, ContentGallery, DetailHero, GalleryItem, RouteFactory, ShareButtons,
};
use crate::data::blog::{derive_categories, fetch_all_posts, get_post_by_id, Post, PostMeta};
use crate::data::constants::APP_TITLE;
use crate::data::utils::markdown_to_html;
use crate::hooks::{use_mermaid, use_syntax_highlighting};
use crate::Route;
use dioxus::prelude::*;

fn resolve_post_image(post: &Post) -> String {
    if post.meta.image_url.is_empty() {
        String::new()
    } else if post.meta.image_url.starts_with("http") {
        post.meta.image_url.clone()
    } else if post.meta.image_url.starts_with('/') {
        format!("https://imwoo90.github.io/the-rust-journey{}", post.meta.image_url)
    } else {
        format!(
            "https://imwoo90.github.io/the-rust-journey/content/posts/{}/{}",
            post.meta.id, post.meta.image_url
        )
    }
}

/// Renders the searchable blog article gallery.
#[component]
pub fn BlogList() -> Element {
    let posts_res = use_server_future(fetch_all_posts)?;
    let posts_guard = posts_res.read();
    let Some(posts) = posts_guard.as_ref() else {
        return rsx! { div { class: "flex justify-center items-center min-h-[50vh]", "Loading articles..." } };
    };

    let blog_items = posts
        .iter()
        .map(|post| GalleryItem {
            id: post.id.clone(),
            title: post.title.clone(),
            description: post.description.clone(),
            image_url: post.image_url.clone(),
            tags: post.tags.clone(),
        })
        .collect();

    rsx! {
        ContentGallery {
            title: "The Journey's Log",
            subtitle: "Documenting every breakthrough and lesson learned while navigating the Rust ecosystem—from bare-metal firmware to cloud-native services.",
            search_placeholder: "Search articles...",
            items: blog_items,
            categories: derive_categories(posts),
            route_factory: RouteFactory(|id| Route::BlogPost { id }),
        }
    }
}

#[component]
fn BlogPostMeta(
    title: String,
    description: String,
    tags: Vec<String>,
    author: String,
    id: String,
    img_url: String,
) -> Element {
    rsx! {
        document::Title { "{title} - {APP_TITLE}" }
        document::Link {
            rel: "canonical",
            href: format!("https://imwoo90.github.io/the-rust-journey/blog/{}", id),
        }
        document::Meta { name: "description", content: description.clone() }
        document::Meta { name: "keywords", content: tags.join(", ") }
        document::Meta { name: "author", content: author }
        document::Meta { property: "og:title", content: title.clone() }
        document::Meta { property: "og:description", content: description.clone() }
        document::Meta { property: "og:type", content: "article" }
        document::Meta { property: "og:url", content: format!("https://imwoo90.github.io/the-rust-journey/blog/{}", id) }
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

#[component]
fn BlogPostContent(post: Post, html_content: String) -> Element {
    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
            article { class: "w-full max-w-3xl flex flex-col gap-10",
                DetailHero {
                    title: post.meta.title.clone(),
                    author: post.meta.author.clone(),
                    date: post.meta.date.clone(),
                    read_time: post.get_read_time(),
                    back_link: Route::BlogList {},
                    back_label: "Blog".to_string(),
                }
                div {
                    class: "prose max-w-none dark:prose-invert",
                    dangerous_inner_html: "{html_content}",
                }
                ShareButtons { title: post.meta.title.clone() }
                SeriesNavigation { current_post: post.clone() }
                Comments {}
            }
        }
    }
}

#[component]
fn PostNotFoundView() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[60vh]",
            h1 { class: "text-4xl font-bold", "Post Not Found" }
            Link {
                to: Route::BlogList {},
                class: "mt-4 text-primary-light hover:underline",
                "Back to Blog"
            }
        }
    }
}

/// Renders an individual blog post with syntax highlighting, mermaid diagrams, and discussion.
#[component]
pub fn BlogPost(id: String) -> Element {
    let mut current_id = use_signal(|| id.clone());
    if current_id() != id {
        current_id.set(id.clone());
    }

    let post_res = use_server_future(move || {
        let id = current_id();
        async move { get_post_by_id(&id).await }
    })?;

    use_syntax_highlighting();
    use_mermaid();

    let posts_guard = post_res.read();
    let Some(post_opt) = posts_guard.as_ref() else {
        return rsx! { div { class: "flex justify-center items-center min-h-[50vh]", "Loading post..." } };
    };

    match post_opt {
        Some(post) => {
            let html_content = markdown_to_html(&post.content, &post.meta.id, "posts");
            let img_url = resolve_post_image(post);

            rsx! {
                BlogPostMeta {
                    title: post.meta.title.clone(),
                    description: post.meta.description.clone(),
                    tags: post.meta.tags.clone(),
                    author: post.meta.author.clone(),
                    id: post.meta.id.clone(),
                    img_url,
                }
                BlogPostContent { post: post.clone(), html_content }
            }
        }
        None => rsx! { PostNotFoundView {} },
    }
}

#[component]
fn SeriesNavigation(current_post: Post) -> Element {
    let posts_res = use_server_future(fetch_all_posts)?;
    let posts_guard = posts_res.read();

    let (Some(series_name), Some(posts)) =
        (current_post.meta.series.as_ref(), posts_guard.as_ref())
    else {
        return rsx! { "" };
    };

    let mut series_posts: Vec<_> = posts
        .iter()
        .filter(|p| p.series.as_ref() == Some(series_name))
        .cloned()
        .collect();

    series_posts.sort_by_key(|p| p.series_order.unwrap_or(0));

    let Some(current_index) = series_posts
        .iter()
        .position(|p| p.id == current_post.meta.id)
    else {
        return rsx! { "" };
    };

    if series_posts.len() < 2 {
        return rsx! { "" };
    }

    let prev_post = if current_index > 0 {
        series_posts.get(current_index - 1).cloned()
    } else {
        None
    };

    let next_post = if current_index < series_posts.len() - 1 {
        series_posts.get(current_index + 1).cloned()
    } else {
        None
    };

    rsx! {
        div { class: "mt-12 pt-8 border-t border-text-dark/10 dark:border-text-light/10",
            div { class: "flex flex-col gap-4",
                h3 { class: "text-sm font-semibold uppercase tracking-wider text-text-dark/50 dark:text-text-light/50",
                    "More from: {series_name}"
                }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                    SeriesLinkCard { post: prev_post, is_next: false }
                    SeriesLinkCard { post: next_post, is_next: true }
                }
            }
        }
    }
}

#[component]
fn SeriesLinkCard(post: Option<PostMeta>, is_next: bool) -> Element {
    match post {
        Some(item) if is_next => rsx! {
            Link {
                to: Route::BlogPost { id: item.id.clone() },
                class: "group flex flex-col p-4 rounded-xl border border-text-dark/10 dark:border-text-light/10 hover:border-primary-light transition-all duration-300 items-end text-right",
                span { class: "text-xs text-text-dark/60 dark:text-text-light/60 mb-1 flex items-center gap-1",
                    "Next"
                    span { class: "material-symbols-outlined text-[14px]", "arrow_forward" }
                }
                span { class: "font-medium group-hover:text-primary-light transition-colors line-clamp-1",
                    "{item.title}"
                }
            }
        },
        Some(item) => rsx! {
            Link {
                to: Route::BlogPost { id: item.id.clone() },
                class: "group flex flex-col p-4 rounded-xl border border-text-dark/10 dark:border-text-light/10 hover:border-primary-light transition-all duration-300",
                span { class: "text-xs text-text-dark/60 dark:text-text-light/60 mb-1 flex items-center gap-1",
                    span { class: "material-symbols-outlined text-[14px]", "arrow_back" }
                    "Previous"
                }
                span { class: "font-medium group-hover:text-primary-light transition-colors line-clamp-1",
                    "{item.title}"
                }
            }
        },
        None => rsx! {
            div { class: "hidden sm:block" }
        },
    }
}
