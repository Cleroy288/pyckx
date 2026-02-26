//! Toast — Notification system with context

use crate::components::ui::icon::Icon;
use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Toast variant
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Single toast data
#[derive(Clone, PartialEq)]
pub struct ToastData {
    /// Unique ID
    pub id: usize,
    /// Display message
    pub message: String,
    /// Visual variant
    pub variant: ToastVariant,
}

/// Toast state held in context
#[derive(Clone, Copy)]
pub struct ToastState {
    /// Active toasts
    toasts: Signal<Vec<ToastData>>,
    /// Auto-incrementing ID counter
    next_id: Signal<usize>,
}

impl ToastState {
    /// Create new toast state
    fn new() -> Self {
        Self {
            toasts: Signal::new(Vec::new()),
            next_id: Signal::new(0),
        }
    }

    /// Read toasts snapshot
    pub fn toasts_snapshot(&self) -> Vec<ToastData> {
        self.toasts.read().clone()
    }

    /// Show a toast with given variant
    pub fn show(
        &mut self,
        msg: String,
        variant: ToastVariant,
    ) {
        let id = *self.next_id.read();
        self.next_id.set(id + 1);
        let toast = ToastData {
            id,
            message: msg,
            variant,
        };
        self.toasts.write().push(toast);
        // Auto-dismiss after 4 seconds
        let mut toasts = self.toasts;
        schedule_dismiss(id, move || {
            toasts.write().retain(|x| x.id != id);
        });
    }

    /// Show a success toast
    pub fn success(&mut self, msg: String) {
        self.show(msg, ToastVariant::Success);
    }

    /// Show an error toast
    pub fn error(&mut self, msg: String) {
        self.show(msg, ToastVariant::Error);
    }

    /// Dismiss a toast by ID
    pub fn dismiss(&mut self, id: usize) {
        self.toasts.write().retain(|x| x.id != id);
    }
}

/// Auto-dismiss timeout (4 seconds)
const DISMISS_MS: i32 = 4_000;

/// Schedule a callback via setTimeout
fn schedule_dismiss<F: FnOnce() + 'static>(
    _id: usize,
    callback: F,
) {
    let closure = Closure::once_into_js(callback);
    let win = web_sys::window().unwrap();
    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        DISMISS_MS,
    );
}

/// Variant-specific Tailwind classes
fn variant_class(v: ToastVariant) -> &'static str {
    match v {
        ToastVariant::Info => "\
            bg-[var(--color-info-bg)] \
            border border-[var(--color-info-border)] \
            text-[var(--color-info-text)]",
        ToastVariant::Success => "\
            bg-[var(--color-success-bg)] \
            border border-[var(--color-success-border)] \
            text-[var(--color-success-text)]",
        ToastVariant::Warning => "\
            bg-[var(--color-warning-bg)] \
            border border-[var(--color-warning-border)] \
            text-[var(--color-warning-text)]",
        ToastVariant::Error => "\
            bg-[var(--color-error-bg)] \
            border border-[var(--color-error-border)] \
            text-[var(--color-error-text)]",
    }
}

/// Provides toast context and renders container
#[component]
pub fn ToastProvider(children: Element) -> Element {
    let state = use_context_provider(ToastState::new);
    let toasts = state.toasts;

    rsx! {
        {children}
        div {
            class: "fixed top-4 right-4 z-[2000] \
                flex flex-col gap-2 max-w-[380px]",
            for toast in toasts.read().iter() {
                ToastItem {
                    key: "{toast.id}",
                    toast: toast.clone(),
                }
            }
        }
    }
}

/// Hook to get toast state from context
pub fn use_toast() -> ToastState {
    use_context::<ToastState>()
}

/// Single toast item display
#[component]
fn ToastItem(toast: ToastData) -> Element {
    let mut state = use_toast();
    let id = toast.id;
    let v_cls = variant_class(toast.variant);
    let cls = format!(
        "flex items-center gap-3 \
         py-3.5 px-4 shadow-lg \
         backdrop-blur-sm \
         animate-[slideIn_0.3s_ease] {}",
        v_cls,
    );

    rsx! {
        div { class: "{cls}",
            span {
                class: "flex-1 text-sm \
                    font-medium leading-relaxed",
                "{toast.message}"
            }
            button {
                class: "flex items-center \
                    justify-center w-6 h-6 p-0 \
                    bg-transparent border-none \
                    cursor-pointer opacity-70 \
                    transition-opacity \
                    duration-[var(--transition-base)] \
                    hover:opacity-100",
                onclick: move |_| state.dismiss(id),
                Icon {
                    icon_name: "X".to_string(),
                }
            }
        }
    }
}
