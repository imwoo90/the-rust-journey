//! # Main Application Views Module
//!
//! ## Overview
//! Exposes page-level view components mounted by the Dioxus router.
//! Includes Home, Blog, Projects, About, Contact, 404 NotFound, and global navigation layouts.
//!
//! ## Submodules
//! - [`about`]: Biography, engineering background, skill matrix, and experience timeline.
//! - [`blog`]: Paginated blog article directory and full-content markdown reading views.
//! - [`contact`]: Interactive contact message form and developer coordination sidebar.
//! - [`footer`]: Global persistent site footer with copyright, links, and social anchors.
//! - [`home`]: Primary landing view showcasing recent articles, projects, and mission statement.
//! - [`navbar`]: Persistent responsive navigation bar, mobile menu drawer, and theme toggler.
//! - [`not_found`]: Custom 404 panic fallback view with recovery links to homepage.
//! - [`projects`]: Showcase project catalog grid and individual case-study view templates.
//!
//! ## Search Tags
//! #views, #routes, #pages, #navigation, #router-views

pub mod about;
pub mod blog;
pub mod contact;
pub mod footer;
pub mod home;
pub mod navbar;
pub mod not_found;
pub mod projects;

pub use about::About;
pub use blog::{BlogList, BlogPost};
pub use contact::Contact;
pub use footer::Footer;
pub use home::Home;
pub use navbar::Navbar;
pub use not_found::NotFound;
pub use projects::{ProjectList, ProjectPost};
