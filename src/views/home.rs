//! # Home Landing Page View
//!
//! ## Overview
//! Provides the primary landing view showcasing recent blog posts and featured projects,
//! personal engineering philosophy, and direct call-to-action triggers toward the contact view.
//!
//! ## Search Tags
//! #home, #landing, #featured-posts, #featured-projects, #hero

use crate::components::*;
use crate::data::blog::{fetch_all_posts, PostMeta};
use crate::data::constants::{APP_SUBTITLE, APP_TITLE};
use crate::data::projects::{fetch_all_projects, ProjectMeta};
use crate::data::utils::get_base_path;
use crate::Route;
use dioxus::prelude::*;

#[component]
fn HomeHero() -> Element {
    rsx! {
        Hero { title: "{APP_TITLE}", subtitle: "{APP_SUBTITLE}",
            PrimaryButton { to: Route::BlogList {}, text: "Explore the Blog" }
        }
    }
}

#[component]
fn HomeCta() -> Element {
    rsx! {
        section { class: "flex flex-col items-center text-center gap-6 bg-white dark:bg-white/5 p-8 sm:p-12 rounded-lg transition-colors border border-text-dark/5 dark:border-white/5 shadow-sm dark:shadow-none",
            SectionTitle { title: "Let's Build Together" }
            p { class: "text-text-dark/80 dark:text-[#D4D4D4] text-base font-normal leading-normal max-w-2xl",
                "I'm passionate about tackling challenging projects with Rust. If you're looking for a developer with deep experience in embedded systems, performance optimization, and cross-platform development, let's talk."
            }
            PrimaryButton { to: Route::Contact {}, text: "Get in Touch" }
        }
    }
}

#[component]
fn HomeLoading() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[80vh]",
            div { class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary-light" }
        }
    }
}

#[component]
fn LatestSection(posts: Vec<PostMeta>, projects: Vec<ProjectMeta>) -> Element {
    let mut recent_items: Vec<_> = posts
        .into_iter()
        .map(|post| {
            (
                post.date.clone(),
                rsx! {
                    Card {
                        title: post.title.clone(),
                        description: post.description.clone(),
                        image_url: format!("{}/{}", get_base_path(), post.image_url),
                        tags: post.tags.clone(),
                        link_to: Route::BlogPost { id: post.id.clone() },
                    }
                },
            )
        })
        .chain(projects.into_iter().map(|project| {
            (
                project.date.clone(),
                rsx! {
                    Card {
                        title: project.title.clone(),
                        description: project.description.clone(),
                        image_url: format!("{}/{}", get_base_path(), project.image_url),
                        tags: project.tags.clone(),
                        link_to: Route::ProjectPost { id: project.id.clone() },
                    }
                },
            )
        }))
        .collect();

    recent_items.sort_by(|a, b| b.0.cmp(&a.0));
    let latest_elements = recent_items.into_iter().take(3).map(|(_, el)| el);

    rsx! {
        Section {
            SectionTitle { title: "Latest Articles" }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                for el in latest_elements {
                    {el}
                }
            }
        }
    }
}

/// Primary home view presenting the latest updates and site overview.
#[component]
pub fn Home() -> Element {
    let posts_res = use_server_future(fetch_all_posts)?;
    let projects_res = use_server_future(fetch_all_projects)?;

    let posts_guard = posts_res.read();
    let projects_guard = projects_res.read();

    if let (Some(posts), Some(projects)) = (posts_guard.as_ref(), projects_guard.as_ref()) {
        rsx! {
            document::Title { "{APP_TITLE}" }
            Container {
                main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                    HomeHero {}
                    LatestSection { posts: posts.clone(), projects: projects.clone() }
                    HomeCta {}
                }
            }
        }
    } else {
        rsx! { HomeLoading {} }
    }
}
