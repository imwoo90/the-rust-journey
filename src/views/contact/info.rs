//! # Contact Info Submodule
//!
//! ## Overview
//! Exposes external communication endpoints, author profile links,
//! and metadata cards for reaching out across developer networks.
//!
//! ## Search Tags
//! #contact-info, #social-links, #author-profile, #sidebar

use crate::components::SectionTitle;
use crate::data::constants::{AUTHOR_EMAIL, GITHUB_URL, GITHUB_USERNAME};
use dioxus::prelude::*;

/// Renders author contact channels and social media profiles.
#[component]
pub fn ContactSidebarInfo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            SectionTitle { title: "Contact Information" }
            div { class: "flex flex-col gap-4",
                ContactInfoItem {
                    icon: "mail".to_string(),
                    label: "Email".to_string(),
                    value: AUTHOR_EMAIL.to_string(),
                    href: format!("mailto:{}", AUTHOR_EMAIL),
                }
                ContactInfoItem {
                    icon: "link".to_string(),
                    label: "GitHub".to_string(),
                    value: format!("@{}", GITHUB_USERNAME),
                    href: GITHUB_URL.to_string(),
                }
            }
        }
    }
}

/// Individual link item within the contact information sidebar.
#[component]
pub fn ContactInfoItem(icon: String, label: String, value: String, href: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            div { class: "flex items-center justify-center size-10 bg-text-dark/5 dark:bg-white/10 rounded-full text-primary-light transition-colors",
                span { class: "material-symbols-outlined", "{icon}" }
            }
            div { class: "flex flex-col",
                p { class: "text-sm text-text-dark/40 dark:text-gray-400 transition-colors",
                    "{label}"
                }
                a {
                    class: "text-base font-medium text-text-dark dark:text-white hover:text-primary-light transition-colors",
                    href: "{href}",
                    "{value}"
                }
            }
        }
    }
}
