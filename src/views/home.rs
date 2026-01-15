use crate::components::*;
use crate::data::blog::fetch_all_posts;
use crate::data::constants::{APP_SUBTITLE, APP_TITLE};
use crate::data::projects::fetch_all_projects;
use crate::data::utils::get_base_path;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let posts_resource = use_resource(fetch_all_posts);
    let projects_resource = use_resource(fetch_all_projects);

    let posts = posts_resource.read();
    let projects = projects_resource.read();

    if let (Some(posts), Some(projects)) = (&*posts, &*projects) {
        // Create a unified list of recent items using iterators
        let mut recent_items: Vec<_> = posts
            .iter()
            .map(|post| {
                (
                    post.date.clone(),
                    rsx! {
                        Card {
                            title: post.title.clone(),
                            description: post.description.clone(),
                            image_url: format!("{}/{}", get_base_path(), post.image_url),
                            tags: post.tags.clone(),
                            link_to: Route::BlogPost {
                                id: post.id.clone(),
                            },
                        }
                    },
                )
            })
            .chain(projects.iter().map(|project| {
                (
                    project.date.clone(),
                    rsx! {
                        Card {
                            title: project.title.clone(),
                            description: project.description.clone(),
                            image_url: format!("{}/{}", get_base_path(), project.image_url),
                            tags: project.tags.clone(),
                            link_to: Route::ProjectPost {
                                id: project.id.clone(),
                            },
                        }
                    },
                )
            }))
            .collect();

        // Sort by date descending
        recent_items.sort_by(|a, b| b.0.cmp(&a.0));

        // Take the 3 most recent
        let latest_elements = recent_items.into_iter().take(3).map(|(_, el)| el);

        rsx! {
            document::Title { "{APP_TITLE}" }
            Container {
                main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                    Hero { title: "{APP_TITLE}", subtitle: "{APP_SUBTITLE}",
                        PrimaryButton { to: Route::BlogList {}, text: "Explore the Blog" }
                    }

                    Section {
                        SectionTitle { title: "Latest Articles" }

                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                            for el in latest_elements {
                                {el}
                            }
                        }
                    }

                    section { class: "flex flex-col items-center text-center gap-6 bg-white dark:bg-white/5 p-8 sm:p-12 rounded-lg transition-colors border border-text-dark/5 dark:border-white/5 shadow-sm dark:shadow-none",
                        SectionTitle { title: "Let's Build Together" }
                        p { class: "text-text-dark/80 dark:text-[#D4D4D4] text-base font-normal leading-normal max-w-2xl",
                            "I'm passionate about tackling challenging projects with Rust. If you're looking for a developer with deep experience in embedded systems, performance optimization, and cross-platform development, let's talk."
                        }
                        PrimaryButton { to: Route::Contact {}, text: "Get in Touch" }
                    }
                }
            }
            Footer {}
        }
    } else {
        rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[80vh]",
                div { class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary-light" }
            }
        }
    }
}
