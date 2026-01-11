use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogSearch(placeholder: String) -> Element {
    rsx! {
        div { class: "relative w-full md:flex-1",
            span { class: "material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-text-dark/40 dark:text-gray-400",
                "search"
            }
            input {
                class: "w-full bg-white dark:bg-[#2a2a2a] border border-text-dark/10 dark:border-white/10 rounded-md h-12 pl-10 pr-4 text-text-dark dark:text-white placeholder:text-text-dark/40 dark:placeholder:text-gray-400 focus:ring-primary-light focus:border-primary-light transition-all",
                placeholder: "{placeholder}",
                r#type: "text",
            }
        }
    }
}

#[component]
pub fn BlogCategories(categories: Vec<String>, active: String) -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-2 justify-center",
            button {
                class: {
                    let base = "px-4 py-2 text-sm font-medium rounded-md transition-colors";
                    if active == "All" {
                        format!(
                            "{} bg-text-dark/10 dark:bg-white/10 text-text-dark dark:text-white",
                            base,
                        )
                    } else {
                        format!(
                            "{} bg-transparent text-text-dark/60 dark:text-gray-400 hover:bg-text-dark/10 dark:hover:bg-white/10 hover:text-text-dark dark:hover:text-white",
                            base,
                        )
                    }
                },
                "All"
            }
            for cat in categories {
                button {
                    class: {
                        let base = "px-4 py-2 text-sm font-medium rounded-md transition-colors";
                        if active == cat {
                            format!(
                                "{} bg-text-dark/10 dark:bg-white/10 text-text-dark dark:text-white",
                                base,
                            )
                        } else {
                            format!(
                                "{} bg-transparent text-text-dark/60 dark:text-gray-400 hover:bg-text-dark/10 dark:hover:bg-white/10 hover:text-text-dark dark:hover:text-white",
                                base,
                            )
                        }
                    },
                    "{cat}"
                }
            }
        }
    }
}

#[component]
pub fn EntryHero(
    title: String,
    author: String,
    date: String,
    read_time: String,
    back_link: Route,
    back_label: String,
) -> Element {
    rsx! {
        header { class: "flex flex-col gap-6 border-b border-text-dark/5 dark:border-white/5 pb-10",
            div { class: "flex items-center gap-2 text-sm text-text-dark/60 dark:text-gray-500 font-medium",
                Link {
                    class: "hover:text-primary-light transition-colors",
                    to: back_link,
                    "{back_label}"
                }
                span { "›" }
                span { class: "truncate text-text-dark/40 dark:text-gray-400", "{title}" }
            }
            h1 { class: "text-text-dark dark:text-white text-4xl sm:text-5xl font-extrabold leading-tight tracking-tight",
                "{title}"
            }
            div { class: "flex items-center gap-4 text-sm text-gray-400",
                span { class: "font-medium text-primary-light", "{author}" }
                span { class: "text-gray-600", "•" }
                time { "{date}" }
                span { class: "text-gray-600", "•" }
                span { "{read_time}" }
            }
        }
    }
}
