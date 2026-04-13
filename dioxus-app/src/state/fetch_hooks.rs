//! Generic data-fetching hooks

use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;

/// Fetch callback type — returns a future with
/// Result<T, String>
pub type FetchFn<T> = Box<
    dyn Fn() -> Pin<
        Box<dyn Future<Output = Result<T, String>>>,
    > + 'static,
>;

/// Fetch data once on mount, store in signal,
/// show errors via log
pub fn use_fetch_on_mount<T: 'static>(
    data: Signal<T>,
    loading: Signal<bool>,
    fetch: FetchFn<T>,
) {
    use_effect(move || {
        let fut = fetch();
        spawn(async move {
            let mut d = data;
            let mut l = loading;
            match fut.await {
                Ok(val) => d.set(val),
                Err(e) => {
                    crate::api::log::log_error(
                        &e, "fetch_on_mount",
                    );
                }
            }
            l.set(false);
        });
    });
}

/// Derive a bool memo: true when vec is non-empty
pub fn has_items<T: PartialEq + Clone + 'static>(
    items: Signal<Vec<T>>,
) -> Memo<bool> {
    use_memo(move || !(items)().is_empty())
}
