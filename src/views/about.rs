use crate::components::{Badge, Container, Section, SectionTitle, TimelineItem};
use crate::views::Footer;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        Container {
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                section { class: "flex flex-col md:flex-row items-center gap-8 md:gap-12 px-4",
                    div { class: "w-48 h-48 md:w-60 md:h-60 flex-shrink-0",
                        img {
                            class: "w-full h-full rounded-full object-cover border-4 border-primary-light/50 shadow-lg",
                            src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAPY0CCVN_4GL3-hGi_oTrltehQY07eSFKotu0R7VEFER7S3HpAsYPIVhi9tPeKSEfsqsbJDH-pUOecDZ1_9LKTtOUxtVEzozK1Mg5PTH34O5gL4fQqpGTXDDdohpSDkoCMHma6B6i_LW633qtLmpDbSQRBMyRE_h90oqvCdv2jK4_ToQG3ZlHKwnorWLecafDGcNgiVkbjeGfDCyqoR6ADi6nytdDEqrwSVszYU0i1h-BRbHH4V5fdVO9QZZq5i0QXVNIKKabPaWwV",
                            alt: "Professional headshot of the developer",
                        }
                    }
                    div { class: "flex flex-col gap-4 text-center md:text-left",
                        h1 { class: "text-text-dark dark:text-white text-4xl md:text-5xl font-black leading-tight tracking-[-0.033em] transition-colors",
                            "Hi, I'm Alex."
                        }
                        p { class: "text-lg md:text-xl font-normal leading-normal text-text-dark/80 dark:text-[#D4D4D4] transition-colors",
                            "I'm a full-stack embedded developer with a singular passion: leveraging the power of Rust to build robust, efficient, and secure software across every conceivable platform. From the tight constraints of bare-metal microcontrollers to the vast scale of cloud backends, I believe Rust is the key to a new era of reliable systems."
                        }
                    }
                }
                Section { class: "grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12 px-4",
                    div { class: "flex flex-col gap-4",
                        SectionTitle { title: "My Philosophy" }
                        p { class: "text-base font-normal leading-relaxed text-text-dark/80 dark:text-[#D4D4D4] transition-colors",
                            "The \"Rust-for-everything\" philosophy isn't just a technical preference; it's a commitment to quality. It means applying the principles of memory safety, zero-cost abstractions, and fearless concurrency to every layer of the stack. This approach minimizes bugs, maximizes performance, and creates software that is a pleasure to maintain and extend, whether it's firmware for a tiny IoT device or a high-traffic web service."
                        }
                    }
                    div { class: "flex flex-col gap-4",
                        SectionTitle { title: "Core Skills" }
                        div { class: "flex flex-wrap gap-3",
                            for skill in [
                                "Embedded Rust",
                                "Bare-Metal Firmware",
                                "RTOS Integration",
                                "WebAssembly (WASM)",
                                "Async Rust (Tokio)",
                                "Backend APIs (axum)",
                                "Cross-Platform Mobile",
                                "CI/CD & DevOps",
                                "Linux Systems",
                            ]
                            {
                                Badge { text: skill.to_string() }
                            }
                        }
                    }
                }
                Section { class: "flex flex-col gap-8 px-4 mb-20",
                    SectionTitle { title: "My Journey" }
                    div { class: "relative pl-6 border-l-2 border-primary-light/30",
                        TimelineItem {
                            date: "2022 - Present",
                            title: "Lead Embedded Engineer, Innovatech Dynamics",
                            description: "Architected and developed a new generation of IoT devices, migrating the entire firmware from C to Rust. Achieved a 40% reduction in memory usage and eliminated a whole class of memory corruption bugs.",
                        }
                        TimelineItem {
                            date: "2020 - 2022",
                            title: "Full-Stack Developer, QuantumLeap Solutions",
                            description: "Built high-performance backend services in Rust and explored its potential for frontend development using WebAssembly, creating interactive data visualization tools that ran entirely in the browser.",
                        }
                        TimelineItem {
                            date: "2018",
                            title: "The Spark: Discovering Rust",
                            description: "While working on a complex C++ project, I discovered Rust. Its promise of safety without sacrificing performance was a revelation that set the course for my entire career.",
                        }
                        TimelineItem {
                            date: "2016 - 2018",
                            title: "Firmware Engineer, Core Systems Inc.",
                            description: "My professional journey began here, writing C and C++ for industrial control systems. It was here I learned the criticality of robust, reliable code in resource-constrained environments.",
                            is_last: true,
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
