use crate::components::{
    BlogCategories, BlogSearch, CallToAction, Card, Comment, Comments, Container, EntryHero, Hero,
    Section,
};
use crate::data::projects::{get_all_categories, get_all_projects, get_project_by_id};
use crate::data::utils::{get_base_path, markdown_to_html};
use crate::hooks::use_syntax_highlighting;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ProjectList() -> Element {
    let projects = get_all_projects();

    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                Hero {
                    title: "The Workshop",
                    subtitle: "A collection of my work, showcasing the versatility of Rust across the full stack.",
                    centered: Some(false),
                    children: rsx! {
                        div { class: "flex flex-col md:flex-row gap-4 w-full items-center mt-4",
                            BlogSearch { placeholder: "Search projects..." }
                            BlogCategories {
                                categories: get_all_categories(),
                                active: "All".to_string(),
                            }
                        }
                    },
                }
                Section { class: "px-4 mb-20",
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        for project in projects {
                            Card {
                                title: project.title.clone(),
                                description: project.description.clone(),
                                image_url: format!("{}/{}", get_base_path(), project.image_url),
                                tags: project.tags.clone(),
                                link_text: project.link_text.clone().unwrap_or_else(|| "View Details".to_string()),
                                external_link: if project.route.is_none() && project.link.is_some() { project.link.clone().unwrap_or_else(|| "#".to_string()) } else { "".to_string() },
                                link_to: Some(Route::ProjectPost {
                                    id: project.id.clone(),
                                }),
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}

#[component]
pub fn ProjectPost(id: String) -> Element {
    let project_resource = use_resource(move || {
        let id = id.clone();
        async move { get_project_by_id(&id).await }
    });

    use_syntax_highlighting();

    let resource = project_resource.read();
    match &*resource {
        Some(Some(project)) => {
            let html_content = markdown_to_html(&project.content, &project.meta.id, "projects");

            rsx! {
                document::Title { "{project.meta.title} - Rust's Horizon" }
                div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
                    article { class: "w-full max-w-3xl flex flex-col gap-10",
                        EntryHero {
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

                        if let Some(link) = &project.meta.link {
                            div { class: "mt-8",
                                a {
                                    href: "{link}",
                                    class: "inline-flex items-center gap-2 bg-primary-light text-text-dark px-6 py-3 rounded-lg font-bold hover:opacity-90 transition-all shadow-md active:scale-95",
                                    target: "_blank",
                                    span { class: "material-symbols-outlined", "launch" }
                                    "{project.meta.link_text.clone().unwrap_or_else(|| \"Visit Project\".to_string())}"
                                }
                            }
                        }

                        Comments {
                            comments: vec![
                                Comment {
                                    author: "Alice".to_string(),
                                    date: "2 days ago".to_string(),
                                    avatar_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuAKTYnKgoWMM15UTvOugoMXvYWs9d-Yo9RCvP6v_ilmnnp8-_OLVqoz1-1AXhD1nNrSq9Z6DfjYY84gVr6eNvJB9O-GYThPTVr5TKapPERZQYYJqdPdks41NivF_GEpX82s4WZ3YZR39bKzgBc7MnkRyKSpauNcQoLJE1pg6IgE5PeMQOMCD0-4TATNGCc_JqpTcEdqQl_9Xelzn2FMFigdAiJ3_Vlsl9CvsliwUySKm-99ilP7IdYUSYQ0v9A6FapxMTzVqSRGWpI7"
                                        .to_string(),
                                    text: "This is an incredible write-up. The performance gains you've achieved by using Rust and Wasm are seriously impressive. I've been considering a similar approach for a project, and this post just convinced me. Great work!"
                                        .to_string(),
                                },
                                Comment {
                                    author: "Bob".to_string(),
                                    date: "1 day ago".to_string(),
                                    avatar_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBBY54kL1b9FE1c05fMLGziuoEeOH4QsqnruT9ad7rlIs0rLWzKrS_Z9-RWZkSuiCXnRcqQS5Rt2JoqkiKOL6VQSYzlHEl35OgBa9EOvwBmD5ypYtvZyL9nfx7eKDMA5PDHBV1Rf2ROcDvuF7FABBmdLvY4qL1tG20C_4JssGJPKbDdANUcfv11LJL_8s-67IVXzTFr2uo3ApFtU-PxGgkRp3IhLgB8AlonuubXq7sMAgiS8-BZq3EyMP3qr2pbtOYx7fz15rmwrsHX"
                                        .to_string(),
                                    text: "Fascinating project. I'm curious about the bundle size. How did you manage to keep it small enough for a good web experience? Any tips on optimizing the Wasm binary?"
                                        .to_string(),
                                },
                            ],
                        }

                        CallToAction {}
                    }
                }
                Footer {}
            }
        }
        Some(None) => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                h1 { class: "text-4xl font-bold", "Project Not Found" }
                Link {
                    to: Route::ProjectList {},
                    class: "mt-4 text-primary-light hover:underline",
                    "Back to Projects"
                }
            }
            Footer {}
        },
        None => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                div { class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary-light" }
                p { class: "mt-4 text-text-dark/60 dark:text-text-light/60",
                    "Loading project info..."
                }
            }
            Footer {}
        },
    }
}
