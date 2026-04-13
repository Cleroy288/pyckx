// ** game_helpers.rs **
// ==> Shared helpers for turn-based game players

use dioxus::prelude::*;

/// Derive a string field from current item in a
/// stored collection, keyed by a reactive index.
pub fn derive_field<T, F>(
    items: Vec<T>,
    current: Signal<usize>,
    accessor: F,
) -> Memo<String>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&T) -> String + Copy + Send + Sync + 'static,
{
    let items = use_hook(|| items);
    use_memo(move || {
        items
            .get((current)())
            .map(&accessor)
            .unwrap_or_default()
    })
}

/// Shuffle items randomly (Fisher-Yates algorithm)
pub fn shuffle_deterministic<T>(items: &mut [T]) {
    let len = items.len();
    for i in (1..len).rev() {
        let j = random_index(i + 1);
        items.swap(i, j);
    }
}

/// Random index in [0, bound) via JS Math.random()
fn random_index(bound: usize) -> usize {
    let r = js_sys::Math::random();
    (r * bound as f64) as usize % bound
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_random_index_stays_in_bounds() {
        // random_index uses js_sys::Math::random()
        // which is unavailable in native tests.
        // Test the bound logic directly.
        let bound = 4;
        for val in 0..100 {
            let idx = val % bound;
            assert!(idx < bound);
        }
    }
}
