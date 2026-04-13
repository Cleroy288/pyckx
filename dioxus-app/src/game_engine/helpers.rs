//! Pure helpers shared by game players.

use dioxus::prelude::*;

/// Derive a string field from the current item of a
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

/// Shuffle a slice in-place using Fisher-Yates.
pub fn shuffle<T>(items: &mut [T]) {
    let len = items.len();
    for i in (1..len).rev() {
        let j = random_index(i + 1);
        items.swap(i, j);
    }
}

/// Return a pseudo-random index in `[0, bound)`.
fn random_index(bound: usize) -> usize {
    let r = js_sys::Math::random();
    (r * bound as f64) as usize % bound
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle_preserves_length() {
        let mut items = vec![1, 2, 3, 4, 5];
        shuffle(&mut items);
        assert_eq!(items.len(), 5);
    }

    #[test]
    fn test_shuffle_empty_slice_is_noop() {
        let mut items: Vec<i32> = vec![];
        shuffle(&mut items);
        assert!(items.is_empty());
    }

    #[test]
    fn test_shuffle_single_element_stays() {
        let mut items = vec![42];
        shuffle(&mut items);
        assert_eq!(items, vec![42]);
    }
}
