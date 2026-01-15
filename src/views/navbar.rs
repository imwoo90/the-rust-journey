use crate::data::constants::{APP_TITLE, FAVICON};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut mobile_menu_open = use_signal(|| false);
    let mut is_dark = use_context::<Signal<bool>>();

    rsx! {
        header { class: "flex items-center justify-between whitespace-nowrap border-b border-solid border-text-dark/10 dark:border-white/10 px-4 sm:px-6 lg:px-8 py-4 sticky top-0 bg-background-light/80 dark:bg-background-dark/80 backdrop-blur-sm z-50 transition-colors duration-300",
            Link {
                to: Route::Home {},
                class: "flex items-center gap-4 text-text-dark dark:text-white group",
                Logo { class: "group-hover:scale-110 transition-transform duration-300" }
                h2 { class: "text-text-dark dark:text-white text-xl font-bold leading-tight tracking-[-0.015em]",
                    "{APP_TITLE}"
                }
            }
            div { class: "flex flex-1 justify-end items-center gap-4",
                nav { class: "hidden md:flex items-center gap-8",
                    NavLink { to: Route::Home {}, "Home" }
                    NavLink { to: Route::BlogList {}, "Blog" }
                    NavLink { to: Route::ProjectList {}, "Projects" }
                    NavLink { to: Route::About {}, "About" }
                    NavLink { to: Route::Contact {}, "Contact" }
                }

                // Theme Toggle
                button {
                    class: "p-2 rounded-full hover:bg-black/5 dark:hover:bg-white/5 text-text-dark dark:text-white transition-colors",
                    onclick: move |_| is_dark.set(!is_dark()),
                    span { class: "material-symbols-outlined",
                        if is_dark() {
                            "light_mode"
                        } else {
                            "dark_mode"
                        }
                    }
                }

                // Mobile Menu Toggle
                button {
                    class: "md:hidden text-text-dark dark:text-white p-2",
                    onclick: move |_| mobile_menu_open.set(!mobile_menu_open()),
                    span { class: "material-symbols-outlined",
                        if mobile_menu_open() {
                            "close"
                        } else {
                            "menu"
                        }
                    }
                }
            }

            // Mobile Navigation
            if mobile_menu_open() {
                nav { class: "absolute top-full left-0 w-full bg-background-light dark:bg-background-dark border-b border-text-dark/10 dark:border-white/10 p-4 md:hidden flex flex-col shadow-2xl transition-colors duration-300 animate-in fade-in slide-in-from-top-4",
                    MobileLink {
                        to: Route::Home {},
                        onclick: move |_| mobile_menu_open.set(false),
                        "Home"
                    }
                    MobileLink {
                        to: Route::BlogList {},
                        onclick: move |_| mobile_menu_open.set(false),
                        "Blog"
                    }
                    MobileLink {
                        to: Route::ProjectList {},
                        onclick: move |_| mobile_menu_open.set(false),
                        "Projects"
                    }
                    MobileLink {
                        to: Route::About {},
                        onclick: move |_| mobile_menu_open.set(false),
                        "About"
                    }
                    MobileLink {
                        to: Route::Contact {},
                        onclick: move |_| mobile_menu_open.set(false),
                        "Contact"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Logo(class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    rsx! {
        img { class: "size-6 {class}", src: FAVICON, alt: "Logo" }
    }
}

#[component]
fn NavLink(to: Route, children: Element) -> Element {
    let current_route: Route = use_route();
    // Simple check: Exact match or prefix match for some routes could be added if needed.
    // Dioxus router matching logic:
    let is_active = current_route == to
        || (to == Route::BlogList {} && matches!(current_route, Route::BlogPost { .. }))
        || (to == Route::ProjectList {} && matches!(current_route, Route::ProjectPost { .. }));

    let active_class = if is_active {
        "text-primary-light"
    } else {
        "text-text-dark/70 dark:text-[#D4D4D4] hover:text-primary-light"
    };

    rsx! {
        Link {
            to,
            class: "text-sm font-medium leading-normal transition-colors {active_class}",
            {children}
        }
    }
}

#[component]
fn MobileLink(to: Route, onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    rsx! {
        Link {
            to,
            class: "text-text-dark dark:text-[#D4D4D4] hover:text-primary-light text-lg font-medium py-3 border-b border-text-dark/5 dark:border-white/5 last:border-0 transition-colors",
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}
