//! # Application Entrypoint and Routing Table
//!
//! ## Overview
//! Configures top-level router hierarchy, static pre-rendering discovery,
//! server-side static file serving, and browser client hydration shells.
//!
//! ## Submodules
//! - [`components`]: Reusable UI primitives, layout wrappers, and interactive widgets.
//! - [`data`]: Data access models, metadata loaders, and parsing utilities.
//! - [`hooks`]: Client-side lifecycle hooks for syntax highlighting and diagrams.
//! - [`views`]: Page-level route views, navigation bars, and layout footers.
//!
//! ## Search Tags
//! #entrypoint, #routing, #router, #server, #hydration

use dioxus::prelude::*;
use views::{About, BlogList, BlogPost, Contact, Home, Navbar, NotFound, ProjectList, ProjectPost};

mod components;
mod data;
mod hooks;
mod views;

use data::constants::FAVICON;

/// Top-level application routing table mapped to page view components.
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    /// Home overview page.
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    /// Blog listing gallery.
    #[route("/blog")]
    BlogList {},

    /// Detailed blog post view by slug.
    #[route("/blog/:id")]
    BlogPost {
        /// Target article identifier.
        id: String,
    },

    /// Projects portfolio listing.
    #[route("/projects")]
    ProjectList {},

    /// Detailed project view by slug.
    #[route("/projects/:id")]
    ProjectPost {
        /// Target project identifier.
        id: String,
    },

    /// Author biography and engineering philosophy.
    #[route("/about")]
    About {},

    /// Contact inquiry form.
    #[route("/contact")]
    Contact {},
    #[end_layout]
    /// Catch-all 404 handler.
    #[route("/:..segments")]
    NotFound {
        /// Unmatched URI path segments.
        segments: Vec<String>,
    },
}

/// Bundled Tailwind CSS stylesheet asset.
const MAIN_CSS: Asset = asset!("assets/tailwind.css");

/// Discovers static pre-render routes for SSG export.
#[server(endpoint = "static_routes")]
pub async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes = vec![
        "/".to_string(),
        "/blog".to_string(),
        "/projects".to_string(),
        "/about".to_string(),
        "/contact".to_string(),
    ];

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let posts_dir = manifest_dir.join("public/content/posts");
    let projects_dir = manifest_dir.join("public/content/projects");

    if let Ok(entries) = std::fs::read_dir(&posts_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                if let Some(name) = entry.file_name().to_str() {
                    routes.push(format!("/blog/{}", name));
                }
            }
        }
    }

    if let Ok(entries) = std::fs::read_dir(&projects_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                if let Some(name) = entry.file_name().to_str() {
                    routes.push(format!("/projects/{}", name));
                }
            }
        }
    }

    Ok(routes)
}

/// Normalizes GitHub Pages base path prefix on incoming server requests.
#[cfg(feature = "server")]
async fn strip_base_path(
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    if path.starts_with("/the-rust-journey") {
        let new_path = &path["/the-rust-journey".len()..];
        let new_path = if new_path.is_empty() { "/" } else { new_path };
        let parts = uri.into_parts();
        let query = parts.path_and_query.as_ref().and_then(|pq| pq.query()).unwrap_or("");
        let new_pq = if query.is_empty() {
            new_path.to_string()
        } else {
            format!("{}?{}", new_path, query)
        };
        if let Ok(new_uri) = new_pq.parse::<axum::http::Uri>() {
            req.extensions_mut().insert(axum::extract::OriginalUri(new_uri.clone()));
            *req.uri_mut() = new_uri;
        }
    }
    next.run(req).await
}

/// Server entrypoint running Axum with Dioxus Fullstack SSR handler.
#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let static_dir = std::env::var("DIOXUS_PUBLIC_PATH")
        .map(std::path::PathBuf::from)
        .ok()
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|parent| parent.join("public")))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("./public"));

    let incremental_cfg = dioxus::server::IncrementalRendererConfig::new()
        .static_dir(static_dir);

    let serve_cfg = dioxus::server::ServeConfig::builder()
        .incremental(incremental_cfg);

    let addr = dioxus_cli_config::fullstack_address_or_localhost();

    use dioxus::server::DioxusRouterExt;
    let router = axum::Router::new()
        .serve_dioxus_application(serve_cfg.clone(), App)
        .layer(axum::middleware::from_fn(strip_base_path));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

/// Client entrypoint for browser WASM hydration.
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

/// Detects initial theme from localStorage or system color-scheme preference.
fn get_initial_theme() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(saved)) = storage.get_item("theme") {
                    return saved == "dark";
                }
            }
            if let Ok(Some(mql)) = window.match_media("(prefers-color-scheme: dark)") {
                return mql.matches();
            }
        }
    }
    false
}

/// Synchronizes the theme state to the DOM document element and persistent storage.
#[warn(unused_variables)]
fn sync_theme(_is_dark: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    let _ = root.class_list().toggle_with_force("dark", _is_dark);
                }
            }
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item("theme", if _is_dark { "dark" } else { "light" });
            }
        }
    }
}

/// Root application shell providing global providers, scripts, fonts, and router layout.
#[allow(non_snake_case)]
#[component]
fn App() -> Element {
    let is_dark = use_signal(get_initial_theme);
    use_context_provider(|| is_dark);

    use_effect(move || sync_theme(is_dark()));

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "true",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600;700&family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap",
            rel: "stylesheet",
        }
        document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js" }
        document::Script { src: "https://cdn.jsdelivr.net/npm/mermaid@10.9.1/dist/mermaid.min.js" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Meta {
            name: "google-site-verification",
            content: "Z60MMCKeg9laVsPok7iQuQWayuy9ptB0kgrC2AgGh3s",
        }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta {
            name: "googlebot",
            content: "index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1",
        }

        div { class: if is_dark() { "dark" } else { "" },
            div { class: "bg-background-light dark:bg-background-dark text-text-dark dark:text-text-light min-h-screen transition-colors duration-300",
                Router::<Route> {}
            }
        }
    }
}
