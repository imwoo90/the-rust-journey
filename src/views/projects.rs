use crate::components::{ContentGallery, DetailHero, GalleryItem, RouteFactory};
use crate::data::constants::APP_TITLE;
use crate::data::projects::{derive_categories, fetch_all_projects, get_project_by_id};
use crate::data::utils::markdown_to_html;
use crate::hooks::use_syntax_highlighting;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ProjectList() -> Element {
    let projects_res = use_server_future(fetch_all_projects)?;
    let projects_guard = projects_res.read();
    let projects = projects_guard.as_ref().unwrap();

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

    let projects_guard = project_res.read();
    let project_opt = projects_guard.as_ref().unwrap();

    match project_opt {
        Some(project) => {
            let html_content = markdown_to_html(&project.content, &project.meta.id, "projects");

            rsx! {
                document::Title { "{project.meta.title} - {APP_TITLE}" }
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

                    }
                }
            }
        }
        None => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                h1 { class: "text-4xl font-bold", "Project Not Found" }
                Link {
                    to: Route::ProjectList {},
                    class: "mt-4 text-primary-light hover:underline",
                    "Back to Projects"
                }
            }
        },
    }
}
