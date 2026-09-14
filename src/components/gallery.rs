//! # Content Gallery Component
//!
//! Provides searchable, category-filtered portfolio and article gallery layouts.
//! Integrates hero headers, real-time keyword search, category filter chips, and card grids.

use crate::components::{Card, CategoryFilter, Container, Hero, SearchBar, Section};
use crate::data::utils::get_base_path;
use crate::Route;
use dioxus::prelude::*;

/// Representation of an individual item displayed within a content gallery grid.
#[derive(Props, Clone, PartialEq)]
pub struct GalleryItem {
    /// Unique identifier slug.
    pub id: String,
    /// Item title.
    pub title: String,
    /// Short summary description.
    pub description: String,
    /// Thumbnail image URL or relative path.
    pub image_url: String,
    /// Keyword tags.
    pub tags: Vec<String>,
}

/// Dynamic factory mapping item IDs to typed Dioxus router destinations.
#[derive(Clone)]
pub struct RouteFactory(pub fn(String) -> Route);

impl PartialEq for RouteFactory {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

fn filter_gallery_items(
    items: Vec<GalleryItem>,
    query: &str,
    category: &str,
) -> Vec<GalleryItem> {
    let q = query.to_lowercase();
    items
        .into_iter()
        .filter(|item| {
            let matches_search = q.is_empty()
                || item.title.to_lowercase().contains(&q)
                || item.description.to_lowercase().contains(&q)
                || item.tags.iter().any(|tag| tag.to_lowercase().contains(&q));
            let matches_category = category == "All" || item.tags.iter().any(|t| t == category);
            matches_search && matches_category
        })
        .collect()
}

#[component]
fn GalleryCardsGrid(
    items: Vec<GalleryItem>,
    route_factory: RouteFactory,
) -> Element {
    rsx! {
        Section { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 mb-20 px-4",
            for item in items {
                Card {
                    title: item.title.clone(),
                    description: item.description.clone(),
                    image_url: format!("{}/{}", get_base_path(), item.image_url),
                    tags: item.tags.clone(),
                    link_to: (route_factory.0)(item.id.clone()),
                }
            }
        }
    }
}

#[component]
fn GalleryHeroHeader(
    title: String,
    subtitle: String,
    centered_hero: Option<bool>,
    search_placeholder: String,
    search_query: Signal<String>,
    categories: Vec<String>,
    selected_category: Signal<String>,
) -> Element {
    rsx! {
        Hero {
            title: "{title}",
            subtitle: "{subtitle}",
            centered: centered_hero.unwrap_or(true),
            children: rsx! {
                div { class: "flex flex-col gap-6 w-full mt-4",
                    div { class: "w-full mx-auto max-w-2xl",
                        SearchBar {
                            placeholder: "{search_placeholder}",
                            value: search_query(),
                            oninput: move |e: FormEvent| {
                                let mut sq = search_query;
                                sq.set(e.value());
                            },
                        }
                    }
                    CategoryFilter {
                        categories,
                        active: selected_category(),
                        onchange: move |cat| {
                            let mut sc = selected_category;
                            sc.set(cat);
                        },
                    }
                }
            },
        }
    }
}

/// Interactive gallery with search, category filtering, and responsive card rendering.
#[component]
pub fn ContentGallery(
    title: String,
    subtitle: String,
    search_placeholder: String,
    items: Vec<GalleryItem>,
    categories: Vec<String>,
    route_factory: RouteFactory,
    centered_hero: Option<bool>,
) -> Element {
    let search_query = use_signal(String::new);
    let selected_category = use_signal(|| "All".to_string());

    let filtered_items = filter_gallery_items(items, &search_query(), &selected_category());

    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                GalleryHeroHeader {
                    title,
                    subtitle,
                    centered_hero,
                    search_placeholder,
                    search_query,
                    categories,
                    selected_category,
                }
                GalleryCardsGrid {
                    items: filtered_items,
                    route_factory,
                }
            }
        }
    }
}
