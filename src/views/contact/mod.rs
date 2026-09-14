//! # Contact View Subsystem (index.md)
//!
//! ## Overview
//! Provides the primary contact page view composing the interactive validation form,
//! message transmission status transitions, and external communication coordinates.
//!
//! ## Submodules
//! - [`form`]: Contact input state management, client-side validation, and submit dispatchers.
//! - [`mod@info`]: Sidebar contact coordinates, social handles, and availability metadata.
//!
//! ## Search Tags
//! #contact, #form-validation, #contact-info, #user-feedback

pub mod form;
pub mod info;

use crate::components::{Container, Hero, Section};
use crate::data::constants::APP_TITLE;
use dioxus::prelude::*;
use form::{trigger_submission, validate_contact_inputs, ContactFormView};
use info::ContactSidebarInfo;

#[component]
fn ContactHero() -> Element {
    rsx! {
        Hero {
            title: "Get In Touch",
            subtitle: "Have a project in mind, a question about an article, or just want to connect? I'm always open to discussing new opportunities and collaborating on exciting ideas.",
        }
    }
}

/// Renders the confirmation state when a message is successfully delivered.
#[component]
fn ContactSuccessView(on_reset: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center py-12 text-center animate-fade-in",
            div { class: "flex items-center justify-center size-16 bg-green-500/10 text-green-500 rounded-full mb-6 scale-110 transition-transform animate-bounce",
                span { class: "material-symbols-outlined text-4xl", "check_circle" }
            }
            h3 { class: "text-2xl font-bold text-text-dark dark:text-white mb-2", "Message Sent Successfully!" }
            p { class: "text-base text-text-dark/70 dark:text-[#D4D4D4] max-w-md mx-auto mb-8",
                "Thank you for reaching out! I appreciate you taking the time to write, and I will get back to you as soon as possible."
            }
            button {
                class: "flex cursor-pointer items-center justify-center rounded-lg h-12 px-6 bg-primary-light text-text-dark text-base font-bold hover:opacity-90 active:scale-95 transition-all shadow-md",
                onclick: move |_| on_reset.call(()),
                "Send Another Message"
            }
        }
    }
}

#[component]
fn ContactFormCard(
    name: Signal<String>,
    email: Signal<String>,
    message: Signal<String>,
    status: Signal<String>,
    error_msg: Signal<String>,
    on_submit: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "md:col-span-2 bg-white dark:bg-[#2a2a2a] p-8 rounded-lg border border-text-dark/5 dark:border-white/10 transition-colors shadow-sm",
            if status() == "success" {
                ContactSuccessView {
                    on_reset: move |_| {
                        let mut s = status;
                        s.set("idle".to_string());
                    },
                }
            } else {
                ContactFormView {
                    name,
                    email,
                    message,
                    status,
                    error_msg,
                    on_submit: move |e| on_submit.call(e),
                }
            }
        }
    }
}

/// Renders the Contact page with interactive form validation and author information.
#[component]
pub fn Contact() -> Element {
    let name = use_signal(String::new);
    let email = use_signal(String::new);
    let message = use_signal(String::new);
    let mut status = use_signal(|| "idle".to_string());
    let mut error_msg = use_signal(String::new);

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();
        match validate_contact_inputs(name().trim(), email().trim(), message().trim()) {
            Ok(()) => {
                error_msg.set(String::new());
                trigger_submission(status, name, email, message);
            }
            Err(err) => {
                error_msg.set(err.to_string());
                status.set("error".to_string());
            }
        }
    };

    rsx! {
        document::Title { "Contact - {APP_TITLE}" }
        Container {
            ContactHero {}
            Section { class: "px-4 mb-20",
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 md:gap-12",
                    ContactFormCard {
                        name,
                        email,
                        message,
                        status,
                        error_msg,
                        on_submit: handle_submit,
                    }
                    ContactSidebarInfo {}
                }
            }
        }
    }
}
