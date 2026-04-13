//! FileUpload — drag-drop zone with a file picker fallback.

use super::icon::Icon;
use dioxus::prelude::*;

/// Drag-and-drop file upload zone.
///
/// Drag events toggle a highlighted border. Drop itself is
/// a visual-only cue — actual file access happens through
/// the embedded `<input type="file">` picker.
#[component]
pub fn FileUpload(
    on_files: EventHandler<Vec<String>>,
    #[props(default)] accept: String,
    #[props(default = false)] multiple: bool,
    #[props(default)] class: String,
) -> Element {
    let mut dragging = use_signal(|| false);
    let zone_cls = build_zone_class(*dragging.read(), &class);
    rsx! {
        div {
            class: "{zone_cls}",
            ondragover: move |ev: DragEvent| {
                ev.prevent_default();
                dragging.set(true);
            },
            ondragleave: move |_| dragging.set(false),
            ondrop: move |ev: DragEvent| {
                ev.prevent_default();
                dragging.set(false);
            },
            Icon { icon_name: "Layers".to_string() }
            p { class: "m-0 text-sm text-center",
                "Drop files here or "
                label { class: "{BROWSE_CLS}",
                    "browse"
                    input {
                        r#type: "file",
                        class: "hidden",
                        accept: "{accept}",
                        multiple: multiple,
                        onchange: move |ev: Event<FormData>| {
                            let fe = ev.files();
                            on_files.call(
                                fe.iter().map(|f| f.name()).collect(),
                            );
                        },
                    }
                }
            }
        }
    }
}

/// Build the drop-zone class string (base + dragging + extra).
fn build_zone_class(is_dragging: bool, extra: &str) -> String {
    format!(
        "{ZONE_CLS} {} {}",
        if is_dragging { ZONE_DRAG } else { "" },
        extra,
    )
}

const ZONE_CLS: &str = "\
    flex flex-col items-center justify-center \
    gap-3 p-8 \
    border-2 border-dashed \
    border-[color-mix(in_srgb,var(--primary)_20%,transparent)] \
    bg-[var(--glass-bg)] \
    backdrop-blur-[var(--glass-blur-sm)] \
    text-[var(--color-text-secondary)] \
    cursor-pointer \
    transition-all duration-[var(--transition-base)] \
    [&_svg]:w-7 [&_svg]:h-7 \
    [&_svg]:text-[var(--color-primary)] \
    hover:border-[var(--color-primary)] \
    hover:bg-[var(--glass-bg-hover)]";

const ZONE_DRAG: &str = "\
    !border-[var(--color-primary)] \
    !bg-[var(--glass-bg-hover)]";

const BROWSE_CLS: &str = "\
    text-[var(--color-primary)] \
    cursor-pointer underline \
    hover:opacity-80";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_zone_class_dragging_has_override() {
        let cls = build_zone_class(true, "");
        assert!(cls.contains("!border-[var(--color-primary)]"));
    }

    #[test]
    fn test_build_zone_class_idle_has_no_override() {
        let cls = build_zone_class(false, "extra");
        assert!(!cls.contains("!border-"));
        assert!(cls.contains("extra"));
    }
}
