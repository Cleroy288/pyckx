//! Collection feature — DVD list, add form, card.
//!
//! Public surface: [`CollectionPage`] and [`DvdAddPage`].

mod add;
mod api;
mod card;
mod list;
mod types;

pub use add::DvdAddPage;
pub use list::CollectionPage;
