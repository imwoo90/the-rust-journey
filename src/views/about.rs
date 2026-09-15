//! # About Page View
//!
//! ## Overview
//! Exposes the author biography, full-stack embedded development philosophy,
//! core engineering skill badges, and professional career milestones timeline.
//!
//! ## Search Tags
//! #about, #biography, #skills, #timeline, #career

use crate::components::{Badge, Container, Section, SectionTitle, TimelineItem};
use crate::data::constants::{APP_TITLE, AUTHOR_NAME, AVATAR};
use dioxus::prelude::*;

const HEADSHOT_SRC: Asset = AVATAR;

#[component]
fn AboutProfileHeader() -> Element {
    rsx! {
        section { class: "flex flex-col md:flex-row items-center gap-8 md:gap-12 px-4",
            div { class: "w-48 h-48 md:w-60 md:h-60 flex-shrink-0",
                img {
                    class: "w-full h-full rounded-full object-cover border-4 border-primary-light/50 shadow-lg",
                    src: HEADSHOT_SRC,
                    alt: "Wim's developer mascot avatar",
                }
            }
            div { class: "flex flex-col gap-4 text-center md:text-left",
                h1 { class: "text-text-dark dark:text-white text-4xl md:text-5xl font-black leading-tight tracking-[-0.033em] transition-colors",
                    "Hi, I'm {AUTHOR_NAME}."
                }
                p { class: "text-lg md:text-xl font-normal leading-normal text-text-dark/80 dark:text-[#D4D4D4] transition-colors",
                    "I'm a full-stack embedded developer with a singular passion: leveraging the power of Rust to build robust, efficient, and secure software across every conceivable platform. From the tight constraints of bare-metal microcontrollers to the vast scale of cloud backends, I believe Rust is the key to a new era of reliable systems."
                }
            }
        }
    }
}

#[component]
fn AboutPhilosophy() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            SectionTitle { title: "My Philosophy" }
            p { class: "text-base font-normal leading-relaxed text-text-dark/80 dark:text-[#D4D4D4] transition-colors",
                "The \"Rust-for-everything\" philosophy isn't just a technical preference; it's a commitment to quality. It means applying the principles of memory safety, zero-cost abstractions, and fearless concurrency to every layer of the stack. This approach minimizes bugs, maximizes performance, and creates software that is a pleasure to maintain and extend, whether it's firmware for a tiny IoT device or a high-traffic web service."
            }
        }
    }
}

#[component]
fn AboutCoreSkills() -> Element {
    let skills = [
        "Embedded Rust",
        "Bare-Metal Firmware",
        "RTOS Integration",
        "WebAssembly (WASM)",
        "Async Rust (Tokio)",
        "Backend APIs (axum)",
        "Cross-Platform Mobile",
        "CI/CD & DevOps",
        "Linux Systems",
        "Agent-Native Architecture",
        "AST Analysis (syn)",
    ];

    rsx! {
        div { class: "flex flex-col gap-4",
            SectionTitle { title: "Core Skills" }
            div { class: "flex flex-wrap gap-3",
                for skill in skills {
                    Badge { text: skill.to_string() }
                }
            }
        }
    }
}

struct JourneyMilestone {
    date: &'static str,
    title: &'static str,
    description: &'static str,
}

const MILESTONES: &[JourneyMilestone] = &[
    JourneyMilestone {
        date: "2024–Present",
        title: "Agentic Tooling & System Interfaces",
        description: "Architecting autonomous Rust agent workflows with Tuner, zero-driver browser hardware monitoring with Web Serial (RusTerm), and establishing Agent-Native compile-time quality standards.",
    },
    JourneyMilestone {
        date: "2021–2024",
        title: "Embedded Systems & WASM Integration",
        description: "Bridging bare-metal MCUs with modern browser-based tooling. Pioneered high-performance WebAssembly interfaces and data pipelines connecting hardware serial protocols directly to client applications.",
    },
    JourneyMilestone {
        date: "2018–2021",
        title: "Firmware & RTOS Engineering",
        description: "Engineered mission-critical MCU firmware in C and modern C++, implementing real-time operating systems (RTOS), serial communication protocols (UART, SPI, I2C), and hardened device drivers for industrial applications.",
    },
    JourneyMilestone {
        date: "2014–2018",
        title: "Hardware Bring-Up & Embedded Programming",
        description: "Specialized in board bring-up, peripheral interfacing, and low-level debugging with oscilloscopes and logic analyzers, developing resilient control loops and embedded systems foundations.",
    },
];

#[component]
fn AboutJourneyTimeline() -> Element {
    let total = MILESTONES.len();
    rsx! {
        Section { class: "flex flex-col gap-8 px-4 mb-20",
            SectionTitle { title: "My Journey" }
            div { class: "relative pl-6 border-l-2 border-primary-light/30",
                for (idx, item) in MILESTONES.iter().enumerate() {
                    TimelineItem {
                        key: "{item.date}",
                        date: item.date.to_string(),
                        title: item.title.to_string(),
                        description: item.description.to_string(),
                        is_last: idx + 1 == total,
                    }
                }
            }
        }
    }
}

/// About view describing the author's background, expertise, and timeline.
#[component]
pub fn About() -> Element {
    rsx! {
        document::Title { "About - {APP_TITLE}" }
        Container {
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                AboutProfileHeader {}
                Section { class: "grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12 px-4",
                    AboutPhilosophy {}
                    AboutCoreSkills {}
                }
                AboutJourneyTimeline {}
            }
        }
    }
}
