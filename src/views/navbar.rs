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
                    "Rust's Horizon"
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
        div { class: "size-6 text-primary-light {class}",
            svg {
                fill: "none",
                view_box: "0 0 48 48",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M13.8261 30.5736C16.7203 29.8826 20.2244 29.4783 24 29.4783C27.7756 29.4783 31.2797 29.8826 34.1739 30.5736C36.9144 31.2278 39.9967 32.7669 41.3563 33.8352L24.8486 7.36089C24.4571 6.73303 23.5429 6.73303 23.1514 7.36089L6.64374 33.8352C8.00331 32.7669 11.0856 31.2278 13.8261 30.5736Z",
                    fill: "currentColor",
                }
                path {
                    clip_rule: "evenodd",
                    d: "M39.998 35.764C39.9944 35.7463 39.9875 35.7155 39.9748 35.6706C39.9436 35.5601 39.8949 35.4259 39.8346 35.2825C39.8168 35.2403 39.7989 35.1993 39.7813 35.1602C38.5103 34.2887 35.9788 33.0607 33.7095 32.5189C30.9875 31.8691 27.6413 31.4783 24 31.4783C20.3587 31.4783 17.0125 31.8691 14.2905 32.5189C12.0012 33.0654 9.44505 34.3104 8.18538 35.1832C8.17384 35.2075 8.16216 35.233 8.15052 35.2592C8.09919 35.3751 8.05721 35.4886 8.02977 35.589C8.00356 35.6848 8.00039 35.7333 8.00004 35.7388C8.00004 35.739 8 35.7393 8.00004 35.7388C8.00004 35.7641 8.0104 36.0767 8.68485 36.6314C9.34546 37.1746 10.4222 37.7531 11.9291 38.2772C14.9242 39.319 19.1919 40 24 40C28.8081 40 33.0758 39.319 36.0709 38.2772C37.5778 37.7531 38.6545 37.1746 39.3151 36.6314C39.9006 36.1499 39.9857 35.8511 39.998 35.764ZM4.95178 32.7688L21.4543 6.30267C22.6288 4.4191 25.3712 4.41909 26.5457 6.30267L43.0534 32.777C43.0709 32.8052 43.0878 32.8338 43.104 32.8629L41.3563 33.8352C43.104 32.8629 43.1038 32.8626 43.104 32.8629L43.1051 32.865L43.1065 32.8675L43.1101 32.8739L43.1199 32.8918C43.1276 32.906 43.1377 32.9246 43.1497 32.9473C43.1738 32.9925 43.2062 33.0545 43.244 33.1299C43.319 33.2792 43.4196 33.489 43.5217 33.7317C43.6901 34.1321 44 34.9311 44 35.7391C44 37.4427 43.003 38.7775 41.8558 39.7209C40.6947 40.6757 39.1354 41.4464 37.385 42.0552C33.8654 43.2794 29.133 44 24 44C18.867 44 14.1346 43.2794 10.615 42.0552C8.86463 41.4464 7.30529 40.6757 6.14419 39.7209C4.99695 38.7775 3.99999 37.4427 3.99999 35.7391C3.99999 34.8725 4.29264 34.0922 4.49321 33.6393C4.60375 33.3898 4.71348 33.1804 4.79687 33.0311C4.83898 32.9556 4.87547 32.8935 4.9035 32.8471C4.91754 32.8238 4.92954 32.8043 4.93916 32.7889L4.94662 32.777L4.95178 32.7688ZM35.9868 29.004L24 9.77997L12.0131 29.004C12.4661 28.8609 12.9179 28.7342 13.3617 28.6282C16.4281 27.8961 20.0901 27.4783 24 27.4783C27.9099 27.4783 31.5719 27.8961 34.6383 28.6282C35.082 28.7342 35.5339 28.8609 35.9868 29.004Z",
                    fill: "currentColor",
                    fill_rule: "evenodd",
                }
            }
        }
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
