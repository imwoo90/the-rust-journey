//! # Home Landing Page View
//!
//! ## Overview
//! Provides the primary landing view showcasing the flagship RusTerm Web Serial project banner,
//! recent blog posts and portfolio projects, personal engineering philosophy, and contact call-to-actions.
//!
//! ## Search Tags
//! #home, #landing, #featured-banner, #rusterm, #web-serial, #embedded, #wasm

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
fn RusTermActions() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 w-full md:w-auto flex-shrink-0",
            a {
                class: "flex min-w-[84px] cursor-pointer items-center justify-center gap-2 rounded-lg h-12 px-6 bg-primary hover:bg-primary-hover text-white text-base font-semibold transition-all shadow-md active:scale-95 text-center",
                href: "https://imwoo90.github.io/RusTerm/",
                target: "_blank",
                rel: "noopener noreferrer",
                "Launch Web Monitor 🚀"
            }
            a {
                class: "flex min-w-[84px] cursor-pointer items-center justify-center gap-2 rounded-lg h-12 px-6 border border-text-dark/10 dark:border-white/10 hover:border-primary-light text-text-dark dark:text-white hover:bg-text-dark/5 dark:hover:bg-white/5 text-base font-semibold transition-all active:scale-95 text-center",
                href: "https://github.com/imwoo90/RusTerm",
                target: "_blank",
                rel: "noopener noreferrer",
                span { class: "material-symbols-outlined text-base", "code" }
                "GitHub Repository"
            }
        }
    }
}

#[component]
fn RusTermBadges() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-2",
            Badge { text: "Rust".to_string() }
            Badge { text: "WebAssembly".to_string() }
            Badge { text: "Web Serial API".to_string() }
            Badge { text: "Embedded Systems".to_string() }
            Badge { text: "OPFS".to_string() }
        }
    }
}

#[component]
fn RusTermContent() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 max-w-2xl flex-1",
            div { class: "flex flex-wrap items-center gap-2",
                span { class: "text-xs font-mono font-bold uppercase tracking-wider bg-primary text-white px-2.5 py-1 rounded-full",
                    "Flagship Project"
                }
                span { class: "text-xs font-mono bg-text-dark/5 dark:bg-white/10 text-text-dark/70 dark:text-gray-300 px-2 py-1 rounded",
                    "Zero-Driver Web Serial"
                }
            }
            div { class: "flex flex-col gap-2",
                h2 { class: "text-2xl md:text-3xl font-extrabold text-text-dark dark:text-white tracking-tight",
                    "RusTerm: Zero-Driver Web Serial Monitor"
                }
                p { class: "text-text-dark/80 dark:text-[#D4D4D4] text-base leading-relaxed",
                    "Bridging bare-metal hardware and WebAssembly. An installation-free serial monitor running live in the browser via Rust, WASM, and Web Workers—stream real-time MCU telemetry without installing native desktop drivers or toolchains."
                }
            }
            RusTermBadges {}
        }
    }
}

#[component]
fn RusTermBanner() -> Element {
    rsx! {
        section { class: "relative overflow-hidden rounded-2xl bg-white dark:bg-surface-dark border border-primary-light/30 p-8 md:p-10 shadow-lg transition-colors",
            div { class: "flex flex-col md:flex-row items-center justify-between gap-8 md:gap-12",
                RusTermContent {}
                RusTermActions {}
            }
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
            document::Link {
                rel: "canonical",
                href: "https://imwoo90.github.io/the-rust-journey/",
            }
            document::Meta { name: "description", content: "{APP_SUBTITLE}" }
            Container {
                main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                    HomeHero {}
                    RusTermBanner {}
                    LatestSection { posts: posts.clone(), projects: projects.clone() }
                    HomeCta {}
                }
            }
        }
    } else {
        rsx! { HomeLoading {} }
    }
}
