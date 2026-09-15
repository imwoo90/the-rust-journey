//! # Global Site Constants and Configuration
//!
//! ## Overview
//! Defines central application-wide metadata, author credentials, social platform URLs,
//! and asset links used across views, meta tags, and the global navigation footer.
//!
//! ## Search Tags
//! #constants, #configuration, #metadata, #site-config, #social-links

use dioxus::prelude::*;

/// Primary application title displayed in navigation and SEO tags.
pub const APP_TITLE: &str = "Wim's Rust Journey";

/// Subtitle describing the overarching vision of the blog.
pub const APP_SUBTITLE: &str = "Navigating the Rust ecosystem from silicon to screen. I document the journey of building high-performance software across the entire stack—spanning bare-metal MCUs, backend services, and native mobile apps.";

/// Canonical name of the blog author.
pub const AUTHOR_NAME: &str = "Wim";

/// Primary contact email for author inquiries.
pub const AUTHOR_EMAIL: &str = "wim90@kakao.com";

/// GitHub username of the author.
pub const GITHUB_USERNAME: &str = "imwoo90";

/// Full URL to the author's GitHub profile.
pub const GITHUB_URL: &str = "https://github.com/imwoo90";

/// Canonical asset path for the developer avatar mascot.
pub const AVATAR: Asset = asset!("assets/avatar.jpg");

/// Copyright year shown in footer and legal notices.
pub const COPYRIGHT_YEAR: &str = "2026";

/// Canonical asset path for the application favicon.
pub const FAVICON: Asset = asset!("assets/favicon.png");
