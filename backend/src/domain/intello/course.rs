//! Course domain entities
//!
//! A Course is a structured learning experience generated from user documents.
//! It contains lessons with explanations and various exercise types.
//!
//! # Structure
//! - Course: Top-level container with metadata and lessons
//! - Lesson: A single learning unit with content and exercises
//! - Exercise: A practice activity (QCM, Flashcard, or OpenQuestion)
//!
//! NOTE: This module is reserved for future use and not currently active.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::{Flashcard, Level, OpenQuestion, QcmQuestion};

// ============================================================================
// EXERCISE TYPES
// ============================================================================

/// Type of exercise in a lesson
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseType {
    Qcm,
    Flashcard,
    OpenQuestion,
}

/// Content of an exercise, varies by type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExerciseContent {
    /// Multiple choice questions (reuses existing QcmQuestion)
    Qcm { questions: Vec<QcmQuestion> },
    /// Flashcards for memorization
    Flashcard { cards: Vec<Flashcard> },
    /// Open-ended questions (reuses existing OpenQuestion)
    OpenQuestion { questions: Vec<OpenQuestion> },
}

/// An exercise within a lesson
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Exercise {
    /// Unique identifier
    pub id: String,
    /// Exercise name/title
    pub name: String,
    /// Brief description of what this exercise covers
    pub description: String,
    /// Type of exercise (for quick filtering)
    pub exercise_type: ExerciseType,
    /// Helpful tips for completing the exercise
    pub tips: String,
    /// Order within the lesson (0-indexed)
    pub order: u32,
    /// The actual exercise content
    pub content: ExerciseContent,
}

// ============================================================================
// LESSON
// ============================================================================

/// A lesson within a course
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lesson {
    /// Unique identifier
    pub id: String,
    /// Lesson name/title
    pub name: String,
    /// Brief description of what this lesson covers
    pub description: String,
    /// Order within the course (0-indexed)
    pub order: u32,
    /// AI-generated learning content/explanation
    pub learning_content: String,
    /// Exercises for this lesson
    pub exercises: Vec<Exercise>,
}

// ============================================================================
// COURSE
// ============================================================================

/// A complete course generated from user documents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Course {
    /// Unique identifier
    pub id: String,
    /// Owner user ID
    pub user_id: String,
    /// Course name/title
    pub name: String,
    /// Course description
    pub description: String,
    /// Language of the course content
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subject tags (max 3)
    pub subjects: Vec<String>,
    /// Lessons in this course (ordered)
    pub lessons: Vec<Lesson>,
    /// Creation timestamp (ISO 8601)
    pub created_at: String,
    /// Last update timestamp (ISO 8601)
    pub updated_at: String,
}

// ============================================================================
// HELPER IMPLEMENTATIONS
// ============================================================================

impl Course {
    /// Get total number of exercises across all lessons
    pub fn total_exercises(&self) -> usize {
        self.lessons.iter().map(|l| l.exercises.len()).sum()
    }

    /// Get total number of lessons
    pub fn total_lessons(&self) -> usize {
        self.lessons.len()
    }

    /// Get exercises of a specific type
    pub fn exercises_by_type(&self, exercise_type: &ExerciseType) -> Vec<&Exercise> {
        self.lessons
            .iter()
            .flat_map(|l| l.exercises.iter())
            .filter(|e| &e.exercise_type == exercise_type)
            .collect()
    }
}

impl Lesson {
    /// Get exercises of a specific type in this lesson
    pub fn exercises_by_type(&self, exercise_type: &ExerciseType) -> Vec<&Exercise> {
        self.exercises
            .iter()
            .filter(|e| &e.exercise_type == exercise_type)
            .collect()
    }
}

impl Exercise {
    /// Get the number of items in this exercise
    pub fn item_count(&self) -> usize {
        match &self.content {
            ExerciseContent::Qcm { questions } => questions.len(),
            ExerciseContent::Flashcard { cards } => cards.len(),
            ExerciseContent::OpenQuestion { questions } => questions.len(),
        }
    }
}
