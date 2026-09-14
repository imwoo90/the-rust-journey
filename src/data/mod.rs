//! # Data Access Layer Subsystem (index.md)
//!
//! ## Overview
//! Provides data structures, loaders, and indexing utilities for blog posts and project showcase entries.
//! Centralizes path resolution, markdown frontmatter parsing, and runtime static JSON ingestion.
//!
//! ## Submodules
//! - [`blog`]: Blog post data structures, metadata parsing, and list ingestion.
//! - [`projects`]: Showcase project entry models, link targets, and catalog loaders.
//! - [`constants`]: Site-wide global configuration keys, titles, and layout constants.
//! - [`utils`]: Markdown-to-HTML parser with syntax highlighting and base path resolution (`get_base_path`).
//!
//! ## Search Tags
//! #data, #blog-metadata, #project-metadata, #markdown-parser, #base-path

pub mod blog;
pub mod constants;
pub mod projects;
pub mod utils;
