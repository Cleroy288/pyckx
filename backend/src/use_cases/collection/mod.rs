//! Collection Use Cases
//!
//! Use cases for the Collection app: DVD CRUD and collection management.

#![allow(unused_imports)]

mod add_dvd;
mod delete_collection;
mod delete_dvd;
mod list_dvds;
mod update_dvd;

pub use add_dvd::{AddDvdInput, AddDvdUseCase};
pub use delete_collection::DeleteCollectionUseCase;
pub use delete_dvd::{DeleteDvdInput, DeleteDvdUseCase};
pub use list_dvds::ListDvdsUseCase;
pub use update_dvd::{UpdateDvdInput, UpdateDvdUseCase};
