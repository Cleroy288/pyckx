//! Intello App - Learning Games Application
//!
//! A learning application that allows users to create, manage, and take
//! QCM quizzes, flashcards, and other games.
//!
//! # Architecture
//! - Domain entities: `domain/intello/`
//! - Business logic: `services/intello_service.rs`
//! - Storage: `infrastructure/json_storage/`
//!
//! This module contains only app metadata and constants.
//!
//! # Games Registry
//! The `games_registry` module contains the central list of all available games.

mod app;
pub mod games_registry;
mod qcm;

pub use app::IntelloApp;
pub use games_registry::{GameInstance, AVAILABLE_GAMES};
