//! # Data Display Components
//!
//! Provides SectionTitle headers, vertical chronological TimelineItem views,
//! and scrollable horizontal CategoryFilter chip bars with active state styling.

use dioxus::prelude::*;

/// Prominent section header with lower border styling.
#[component]
pub fn SectionTitle(title: String) -> Element {
    rsx! {
        h2 { class: "text-text-dark dark:text-white text-3xl font-bold leading-tight tracking-[-0.015em] border-b border-text-dark/10 dark:border-white/10 pb-3 transition-colors",
            "{title}"
        }
    }
}

/// Chronological resume or milestones timeline item with left-side marker bullet.
#[component]
pub fn TimelineItem(
    date: String,
    title: String,
    description: String,
    is_last: Option<bool>,
) -> Element {
    let spacing_class = if is_last.unwrap_or(false) {
        ""
    } else {
        "mb-10"
    };
    rsx! {
        div { class: "{spacing_class} ml-4 relative",
            div { class: "absolute w-4 h-4 bg-primary rounded-full mt-1.5 -left-6.5 border border-background-light dark:border-background-dark" }
            time { class: "mb-1 text-sm font-normal leading-none text-text-dark/40 dark:text-gray-400 transition-colors",
                "{date}"
            }
            h3 { class: "text-lg font-semibold text-text-dark dark:text-white transition-colors",
                "{title}"
            }
            p { class: "text-base font-normal text-text-dark/70 dark:text-[#D4D4D4] transition-colors mt-1",
                "{description}"
            }
        }
    }
}

#[component]
fn CategoryChip(label: String, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let base = "px-4 py-2 text-sm font-medium rounded-md transition-colors whitespace-nowrap";
    let active_class = if is_active {
        format!("{} bg-text-dark/10 dark:bg-white/10 text-text-dark dark:text-white", base)
    } else {
        format!(
            "{} bg-transparent text-text-dark/60 dark:text-gray-400 hover:bg-text-dark/10 dark:hover:bg-white/10 hover:text-text-dark dark:hover:text-white",
            base
        )
    };

    rsx! {
        button {
            class: "{active_class}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

/// Scrollable tag chip bar for filtering gallery items by category.
#[component]
pub fn CategoryFilter(
    categories: Vec<String>,
    active: String,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        style {
            r#"
            .no-scrollbar::-webkit-scrollbar {{ display: none; }}
            .no-scrollbar {{ -ms-overflow-style: none; scrollbar-width: none; }}
            "#
        }
        div { class: "w-full md:w-auto overflow-x-auto no-scrollbar py-2",
            div { class: "flex flex-nowrap md:flex-wrap gap-2 justify-start md:justify-center min-w-max md:min-w-0",
                CategoryChip {
                    label: "All".to_string(),
                    is_active: active == "All",
                    onclick: move |_| onchange.call("All".to_string()),
                }
                for cat in categories {
                    CategoryChip {
                        label: cat.clone(),
                        is_active: active == cat,
                        onclick: {
                            let cat_val = cat.clone();
                            move |_| onchange.call(cat_val.clone())
                        },
                    }
                }
            }
        }
    }
}
