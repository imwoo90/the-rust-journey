use crate::components::{Card, CategoryFilter, Container, Hero, SearchBar, Section};
use crate::data::utils::get_base_path;
use crate::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GalleryItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
}

#[component]
pub fn ContentGallery(
    title: String,
    subtitle: String,
    search_placeholder: String,
    items: Vec<GalleryItem>,
    categories: Vec<String>,
    route_factory: fn(String) -> Route,
    centered_hero: Option<bool>,
) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut selected_category = use_signal(|| "All".to_string());

    let filtered_items = items.into_iter().filter(|item| {
        let matches_search = item
            .title
            .to_lowercase()
            .contains(&search_query().to_lowercase())
            || item
                .description
                .to_lowercase()
                .contains(&search_query().to_lowercase());
        let matches_category =
            selected_category() == "All" || item.tags.contains(&selected_category());
        matches_search && matches_category
    });

    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
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
                                    oninput: move |e: FormEvent| search_query.set(e.value()),
                                }
                            }
                            CategoryFilter {
                                categories,
                                active: selected_category(),
                                onchange: move |cat| selected_category.set(cat),
                            }
                        }
                    },
                }

                Section { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 mb-20 px-4",
                    for item in filtered_items {
                        Card {
                            title: item.title.clone(),
                            description: item.description.clone(),
                            image_url: format!("{}/{}", get_base_path(), item.image_url),
                            tags: item.tags.clone(),
                            link_to: route_factory(item.id.clone()),
                        }
                    }
                }
            }
        }
    }
}
