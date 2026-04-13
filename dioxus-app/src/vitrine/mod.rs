//! Vitrine — public landing page feature.
//!
//! Read this module top-down like a newspaper:
//! - `page`    → orchestration of the landing page
//! - `nav`     → sticky top navigation
//! - `hero`, `stage`, `apps`, `pricing` → sections
//! - `buttons` → reusable CTA / ghost button primitives
//! - `data`    → static copy (stats, features, games)

mod apps;
mod buttons;
mod data;
mod hero;
mod nav;
mod page;
mod pricing;
mod stage;

pub use nav::VitrineNav;
pub use page::VitrinePage;
