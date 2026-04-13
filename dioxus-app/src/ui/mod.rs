//! UI module — re-exports every reusable component.
//!
//! One flat entry point: `use crate::ui::{Button, Badge, ...};`

pub mod badge;
pub mod button;
pub mod card;
pub mod code_editor;
pub mod feedback;
pub mod file_upload;
pub mod icon;
pub mod input;
pub mod layout;
pub mod modal;
pub mod progress;
pub mod toast;

pub use badge::{Badge, BadgeVariant};
pub use button::{btn_class, Button, ButtonSize, ButtonVariant};
pub use card::{
    Card, CardGrid, CardVariant, GameCard, ItemCard, NavCard,
};
pub use code_editor::{
    get_editor_value, CodeEditor, CodeLanguage,
};
pub use feedback::{
    EmptyState, LoadingBoundary, Skeleton, SkeletonCard, Spinner,
    SpinnerSize,
};
pub use file_upload::FileUpload;
pub use icon::Icon;
pub use input::{FormField, Input, Select, SelectOption, Textarea};
pub use layout::{HeroBanner, PageLayout, SectionDivider};
pub use modal::{ConfirmDialog, Modal};
pub use progress::{Progress, TabPanel, Tabs};
pub use toast::{
    use_toast, ToastData, ToastProvider, ToastState, ToastVariant,
};
