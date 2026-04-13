//! Home feature — authenticated dashboard / game hub.
//!
//! Public surface: [`HomePage`] only. Everything else is
//! an internal building block of the page.

mod deco;
mod hero;
pub mod logo;
mod page;
pub mod top_bar;

pub use logo::{NavLogo, SplitLogo, SPLIT_LINE_SVG};
pub use page::HomePage;
pub use top_bar::HomeTopBar;
