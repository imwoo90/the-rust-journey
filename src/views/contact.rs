use crate::components::{Container, Hero, Input, PrimaryButton, Section, SectionTitle, TextArea};
use crate::views::Footer;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        Container {
            Hero {
                title: "Get In Touch",
                subtitle: "Have a project in mind, a question about an article, or just want to connect? I'm always open to discussing new opportunities and collaborating on exciting ideas.",
                children: rsx! {},
            }

            Section { class: "px-4 mb-20",
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 md:gap-12",
                    div { class: "md:col-span-2 bg-white dark:bg-[#2a2a2a] p-8 rounded-lg border border-text-dark/5 dark:border-white/10 transition-colors shadow-sm",
                        form {
                            class: "flex flex-col gap-6",
                            onsubmit: |e| e.prevent_default(),
                            SectionTitle { title: "Send a Message" }
                            div { class: "flex flex-col sm:flex-row gap-6",
                                Input {
                                    label: Some("Your Name".to_string()),
                                    id: "name",
                                    placeholder: "John Doe",
                                }
                                Input {
                                    label: Some("Your Email".to_string()),
                                    id: "email",
                                    placeholder: "john.doe@email.com",
                                    r#type: "email",
                                }
                            }
                            TextArea {
                                label: Some("Message".to_string()),
                                id: "message",
                                placeholder: "I'd like to discuss...",
                                rows: 6,
                            }
                            div { class: "flex justify-start",
                                PrimaryButton {
                                    text: "Submit Message",
                                    onclick: move |_| {},
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col gap-6",
                        SectionTitle { title: "Contact Information" }
                        div { class: "flex flex-col gap-4",
                            ContactInfoItem {
                                icon: "mail",
                                label: "Email",
                                value: "contact@rustshorizon.com",
                                href: "mailto:contact@rustshorizon.com",
                            }
                            ContactInfoItem {
                                icon: "link",
                                label: "GitHub",
                                value: "@rust-developer",
                                href: "https://github.com",
                            }
                            ContactInfoItem {
                                icon: "group",
                                label: "LinkedIn",
                                value: "Jane Doe",
                                href: "https://linkedin.com",
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
fn ContactInfoItem(icon: String, label: String, value: String, href: String) -> Element {
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
