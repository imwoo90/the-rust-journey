---
title: "SSG on GitHub Pages with Dioxus 0.7: Overcoming Routing and Base Path Challenges"
date: "2026-07-08"
author: "imwoo90"
description: "A deep dive into migrating this blog to Static Site Generation (SSG) with Dioxus 0.7, solving path routing mismatches using Axum middleware, and handling target compilation constraints."
image_url: "ssg_cover.jpg"
tags: ["rust", "dioxus", "github-pages", "ssg", "axum", "wasm", "tutorial"]
---

Static Site Generation (SSG) is a game-changer for content-rich websites. By pre-rendering Rust code into static HTML files at compile time, we gain blazing-fast load speeds, robust SEO, and complete independence from active backend servers.

However, when hosting a fullstack-capable Dioxus 0.7 project under a subpath on **GitHub Pages** (like `https://imwoo90.github.io/the-rust-journey/`), we run into a major routing paradox during the SSG crawler phase. 

In this article, we’ll explore how we solved this by rewriting our Axum server's entry point, separating target-specific dependencies, and unifying client-side asset loading.

---

## 🗺️ The Routing Paradox

In Dioxus 0.7, running `dx build --release --ssg` starts a temporary Axum server instance and queries it to discover and pre-render all static routes.

Here lies the problem:
1. **At build time**: The Dioxus CLI acts as a client at the root level, requesting `/blog` to save it as `blog/index.html`.
2. **At runtime**: The browser requests the page relative to the GitHub Pages base path, e.g., `/the-rust-journey/blog`.

If we configure the server router under `/the-rust-journey/`, it returns 404 for the CLI's `/blog` query. If we configure it at the root `/`, the browser fails to hydrate the page correctly on the subpath, throwing a **404 ("Lost in memory")** panic.

---

## 🛠️ The Solution: Axum URI Rewrite Middleware

Instead of managing duplicate routers or brittle text replacement hacks in HTML post-build, we solved this at the server level in `src/main.rs`. We added a custom Tokio-based Axum entry point and implemented a simple middleware to dynamically strip the base path prefix:

```rust
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
        let mut parts = uri.into_parts();
        let query = parts.path_and_query.as_ref().and_then(|pq| pq.query()).unwrap_or("");
        let new_pq = if query.is_empty() {
            new_path.to_string()
        } else {
            format!("{}?{}", new_path, query)
        };
        parts.path_and_query = Some(new_pq.parse().unwrap());
        *req.uri_mut() = axum::http::Uri::from_parts(parts).unwrap();
    }
    next.run(req).await
}
```

By applying this middleware to the Axum router:

```rust
    let router = axum::Router::new()
        .serve_dioxus_application(serve_cfg.clone(), App)
        .layer(axum::middleware::from_fn(strip_base_path));
```

The server becomes hybrid:
* **`GET /blog` (from SSG build crawler)** -> matches `/blog` and writes the rendered page successfully to `public/blog/index.html`.
* **`GET /the-rust-journey/blog` (from browser request)** -> strips the prefix, routes to `/blog` internally, and serves the page correctly.

---

## 📦 Separating Target-Specific Dependencies

To make Axum work, we needed crates like `tokio` (with `"full"` features) and `axum` in our server target. However, `tokio`'s full features include dependency on `mio`, which explicitly panics on the `wasm32-unknown-unknown` target.

We resolved this by separating native server-only dependencies inside `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.7.9", features = ["web", "router"] }
# target-specific configuration
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tokio = { version = "1.48.0", features = ["full"] }
axum = { version = "0.8.8" }
dioxus-cli-config = "0.7.9"
```

This prevents compiler contamination, ensuring the client WASM bundle remains lightweight and builds cleanly.

---

## 🎨 Clean Client-Side Asset Loading

Previously, client-side fetching of posts metadata indices (`posts_index.json`) parsed the HTML `<base>` tag manually. 

We replaced this with Dioxus's native CLI config API in `src/data/utils.rs`:

```rust
pub fn get_base_path() -> String {
    if let Some(base_path) = dioxus_cli_config::base_path() {
        let base_path = base_path.trim_matches('/');
        if base_path.is_empty() {
            "".to_string()
        } else {
            format!("/{}", base_path)
        }
    } else {
        "".to_string()
    }
}
```

This ensures assets are fetched correctly from `/the-rust-journey/content/posts_index.json` on the live site, while resolving to `/content/posts_index.json` in local development.

---

## 📈 The Result

The GitHub Actions workflow now operates with zero-config. We simply compile the app and let GitHub Pages serve the static directory:

* All pages are pre-rendered (`index.html`, `blog/index.html`, `projects/index.html`).
* Client-side page transitions are instant with zero hydration glitches.
* Theme switching (dark/light) functions natively on first load.

Hosting a Rust-powered SPA on GitHub Pages is incredibly satisfying once the routing puzzle pieces fall into place!

Happy coding!
