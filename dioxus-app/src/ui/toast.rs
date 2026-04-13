//! Toast — notification system with context and auto-dismiss.

use super::icon::Icon;
use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// Color variant for a toast.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// A single active toast entry.
#[derive(Clone, PartialEq)]
pub struct ToastData {
    pub id: usize,
    pub message: String,
    pub variant: ToastVariant,
}

/// Global toast state exposed through context.
#[derive(Clone, Copy)]
pub struct ToastState {
    toasts: Signal<Vec<ToastData>>,
    next_id: Signal<usize>,
}

impl ToastState {
    fn new() -> Self {
        Self {
            toasts: Signal::new(Vec::new()),
            next_id: Signal::new(0),
        }
    }

    /// Snapshot copy of all active toasts.
    pub fn toasts_snapshot(&self) -> Vec<ToastData> {
        self.toasts.read().clone()
    }

    /// Add a toast with the given variant. Auto-dismisses.
    pub fn show(&mut self, msg: String, variant: ToastVariant) {
        let id = *self.next_id.read();
        self.next_id.set(id + 1);
        self.toasts.write().push(ToastData {
            id, message: msg, variant,
        });
        let mut list = self.toasts;
        schedule_dismiss(move || list.write().retain(|x| x.id != id));
    }

    /// Add a success toast.
    pub fn success(&mut self, msg: String) {
        self.show(msg, ToastVariant::Success);
    }

    /// Add an error toast.
    pub fn error(&mut self, msg: String) {
        self.show(msg, ToastVariant::Error);
    }

    /// Remove a toast by ID.
    pub fn dismiss(&mut self, id: usize) {
        self.toasts.write().retain(|x| x.id != id);
    }
}

/// Provides the toast context and renders the stack.
#[component]
pub fn ToastProvider(children: Element) -> Element {
    let state = use_context_provider(ToastState::new);
    let toasts = state.toasts;
    rsx! {
        {children}
        div { class: "{STACK_CLS}",
            for toast in toasts.read().iter() {
                ToastItem { key: "{toast.id}", toast: toast.clone() }
            }
        }
    }
}

/// Read the toast state from context.
pub fn use_toast() -> ToastState {
    use_context::<ToastState>()
}

#[component]
fn ToastItem(toast: ToastData) -> Element {
    let mut state = use_toast();
    let id = toast.id;
    let cls = format!(
        "{ITEM_CLS} {}",
        variant_class(toast.variant),
    );
    rsx! {
        div { class: "{cls}",
            span { class: "flex-1 text-sm font-medium leading-relaxed",
                "{toast.message}" }
            button {
                class: "{CLOSE_CLS}",
                onclick: move |_| state.dismiss(id),
                Icon { icon_name: "X".to_string() }
            }
        }
    }
}

/// Auto-dismiss delay for a toast, in milliseconds.
const DISMISS_MS: i32 = 4_000;

/// Schedule `callback` after `DISMISS_MS` via `setTimeout`.
fn schedule_dismiss<F: FnOnce() + 'static>(callback: F) {
    let closure = Closure::once_into_js(callback);
    let win = web_sys::window().expect("window available");
    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        DISMISS_MS,
    );
}

/// Tailwind classes for a single variant.
fn variant_class(variant: ToastVariant) -> &'static str {
    match variant {
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

const STACK_CLS: &str = "\
    fixed top-4 right-4 z-[2000] \
    flex flex-col gap-2 max-w-[380px]";

const ITEM_CLS: &str = "\
    flex items-center gap-3 \
    py-3.5 px-4 shadow-lg \
    backdrop-blur-sm \
    animate-[slideIn_0.3s_ease]";

const CLOSE_CLS: &str = "\
    flex items-center justify-center \
    w-6 h-6 p-0 bg-transparent border-none \
    cursor-pointer opacity-70 \
    transition-opacity duration-[var(--transition-base)] \
    hover:opacity-100";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_class_error_has_error_bg() {
        assert!(variant_class(ToastVariant::Error).contains("error-bg"));
    }

    #[test]
    fn test_variant_class_success_has_success_text() {
        let cls = variant_class(ToastVariant::Success);
        assert!(cls.contains("success-text"));
    }

    #[test]
    fn test_variant_class_info_has_info_border() {
        let cls = variant_class(ToastVariant::Info);
        assert!(cls.contains("info-border"));
    }
}
