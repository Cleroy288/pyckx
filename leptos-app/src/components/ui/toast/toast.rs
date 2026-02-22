// ** toast.rs **
// ==> Toast notification system with context provider

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/toast/toast.module.css"
);

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
#[derive(Clone)]
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
    toasts: RwSignal<Vec<ToastData>>,
    /// Auto-incrementing ID counter
    next_id: RwSignal<usize>,
}

impl ToastState {
    fn new() -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(0),
        }
    }

    /// Public constructor for WASM integration tests
    pub fn new_for_test() -> Self {
        Self::new()
    }

    /// Read toasts snapshot for integration tests
    pub fn toasts_for_test(&self) -> Vec<ToastData> {
        self.toasts.get()
    }

    /// Show a toast with given variant
    pub fn show(&self, msg: String, variant: ToastVariant) {
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        let toast = ToastData {
            id,
            message: msg,
            variant,
        };
        self.toasts.update(|t| t.push(toast));
        // Auto-dismiss after 4 seconds
        let toasts = self.toasts;
        set_timeout(
            move || {
                toasts.update(|t| t.retain(|x| x.id != id));
            },
            std::time::Duration::from_secs(4),
        );
    }

    /// Show a success toast
    pub fn success(&self, msg: String) {
        self.show(msg, ToastVariant::Success);
    }

    /// Show an error toast
    pub fn error(&self, msg: String) {
        self.show(msg, ToastVariant::Error);
    }

    /// Dismiss a toast by ID
    pub fn dismiss(&self, id: usize) {
        self.toasts.update(|t| t.retain(|x| x.id != id));
    }
}

/// Provides toast context and renders toast container
#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let state = ToastState::new();
    provide_context(state);

    let toasts = state.toasts;

    view! {
        {children()}
        <div class=style::container>
            <For
                each=move || toasts.get()
                key=|t| t.id
                let:toast
            >
                <ToastItem toast=toast state=state />
            </For>
        </div>
    }
}

/// Hook to get toast state from context
pub fn use_toast() -> ToastState {
    use_context::<ToastState>().expect(
        "ToastState not found. Wrap app with ToastProvider."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: show/success/error use `set_timeout`
    // which requires browser — tested via WASM.
    // Native tests cover dismiss + defaults.

    /// Helper: push a toast directly (no timer)
    fn push_toast(
        state: &ToastState,
        msg: &str,
        variant: ToastVariant,
    ) -> usize {
        let id = state.next_id.get();
        state.next_id.set(id + 1);
        state.toasts.update(|t| {
            t.push(ToastData {
                id,
                message: msg.into(),
                variant,
            });
        });
        id
    }

    #[test]
    fn test_new_state_is_empty() {
        let state = ToastState::new();
        assert!(state.toasts.get().is_empty());
        assert_eq!(state.next_id.get(), 0);
    }

    #[test]
    fn test_push_increments_id() {
        let state = ToastState::new();
        let id0 = push_toast(
            &state,
            "A",
            ToastVariant::Info,
        );
        let id1 = push_toast(
            &state,
            "B",
            ToastVariant::Error,
        );
        assert_eq!(id0, 0);
        assert_eq!(id1, 1);
        assert_eq!(state.toasts.get().len(), 2);
    }

    #[test]
    fn test_dismiss_removes_by_id() {
        let state = ToastState::new();
        push_toast(&state, "A", ToastVariant::Info);
        push_toast(&state, "B", ToastVariant::Info);
        state.dismiss(0);
        let toasts = state.toasts.get();
        assert_eq!(toasts.len(), 1);
        assert_eq!(toasts[0].message, "B");
    }

    #[test]
    fn test_dismiss_nonexistent_is_noop() {
        let state = ToastState::new();
        push_toast(&state, "A", ToastVariant::Info);
        state.dismiss(99);
        assert_eq!(state.toasts.get().len(), 1);
    }

    #[test]
    fn test_variant_default_is_info() {
        assert_eq!(
            ToastVariant::default(),
            ToastVariant::Info,
        );
    }
}

/// Single toast item display
#[component]
fn ToastItem(
    toast: ToastData,
    state: ToastState,
) -> impl IntoView {
    let variant_class = match toast.variant {
        ToastVariant::Info => style::info,
        ToastVariant::Success => style::success,
        ToastVariant::Warning => style::warning,
        ToastVariant::Error => style::error,
    };
    let id = toast.id;

    view! {
        <div class=format!(
            "{} {}", style::toast, variant_class
        )>
            <span class=style::message>
                {toast.message}
            </span>
            <button
                class=style::dismiss
                on:click=move |_| state.dismiss(id)
            >
                <crate::components::ui::icon::Icon
                    icon_name="X".to_string()
                />
            </button>
        </div>
    }
}
