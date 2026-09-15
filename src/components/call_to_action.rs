//! # Call To Action Banner Component
//!
//! ## Overview
//! Renders a prominent invitation banner encouraging collaboration and inquiries.
//! Features an arrow-accented action link directing visitors toward the contact view.
//!
//! ## Search Tags
//! #cta, #banner, #call-to-action, #contact-prompt

use crate::Route;
use dioxus::prelude::*;

/// Prominent footer banner inviting readers to reach out with project ideas or inquiries.
#[component]
pub fn CallToAction() -> Element {
    rsx! {
        section { class: "border-t border-text-dark/10 dark:border-white/10 pt-12",
            div { class: "flex flex-col items-center text-center",
                h2 { class: "text-text-dark dark:text-white text-2xl font-bold",
                    "Have a similar challenge?"
                }
                p { class: "text-text-dark/60 dark:text-gray-400 mt-2 max-w-xl",
                    "If you're looking to leverage Rust for high-performance web applications, embedded systems, or anything in between, let's talk."
                }
                Link {
                    to: Route::Contact {},
                    class: "inline-flex items-center justify-center gap-2 mt-6 bg-primary text-white font-semibold text-sm px-6 py-3 rounded-md hover:bg-primary-hover transition-all shadow-md active:scale-95",
                    "Get in Touch"
                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                }
            }
        }
    }
}
