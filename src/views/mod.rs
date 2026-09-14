//! # Main Application Views Module
//!
//! Exposes page-level view components mounted by the Dioxus router.
//! Includes Home, Blog, Projects, About, Contact, 404 NotFound, and global navigation layouts.

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
