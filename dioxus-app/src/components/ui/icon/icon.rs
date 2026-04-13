//! Icon — SVG icon registry by name.
//!
//! Each icon is a private component returning an
//! inline SVG. `Icon` dispatches by name string.

use dioxus::prelude::*;

/// Common SVG attrs for stroke-based icons
const SVG_NS: &str =
    "http://www.w3.org/2000/svg";

/// Renders an SVG icon matched by name.
/// Falls back to a square placeholder.
#[component]
pub fn Icon(icon_name: String) -> Element {
    match icon_name.as_str() {
        "Book" => rsx! { IconBook {} },
        "Disc" => rsx! { IconDisc {} },
        "Check" => rsx! { IconCheck {} },
        "CheckCircle" => {
            rsx! { IconCheckCircle {} }
        }
        "Star" => rsx! { IconStar {} },
        "ArrowRight" => {
            rsx! { IconArrowRight {} }
        }
        "ArrowLeft" => {
            rsx! { IconArrowLeft {} }
        }
        "Plus" => rsx! { IconPlus {} },
        "X" => rsx! { IconX {} },
        "Trash" => rsx! { IconTrash {} },
        "HelpCircle" => {
            rsx! { IconHelpCircle {} }
        }
        "List" => rsx! { IconList {} },
        "Layers" => rsx! { IconLayers {} },
        "Sun" => rsx! { IconSun {} },
        "Moon" => rsx! { IconMoon {} },
        "MessageSquare" => {
            rsx! { IconMsg {} }
        }
        _ => rsx! { IconFallback {} },
    }
}

#[component]
fn IconBook() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        xmlns: SVG_NS,
        path {
            d: "M4 19.5A2.5 2.5 \
                0 0 1 6.5 17H20"
        }
        path {
            d: "M6.5 2H20v20H6.5A2.5 \
                2.5 0 0 1 4 19.5v-15\
                A2.5 2.5 0 0 1 6.5 2z"
        }
    }}
}

#[component]
fn IconDisc() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        xmlns: SVG_NS,
        circle { cx: "12", cy: "12", r: "10" }
        circle { cx: "12", cy: "12", r: "3" }
    }}
}

#[component]
fn IconCheck() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        polyline { points: "20 6 9 17 4 12" }
    }}
}

#[component]
fn IconCheckCircle() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        circle { cx: "12", cy: "12", r: "10" }
        path { d: "M9 12l2 2 4-4" }
    }}
}

#[component]
fn IconStar() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        xmlns: SVG_NS,
        path {
            d: "M12 2l3.09 6.26L22 \
                9.27l-5 4.87 1.18 \
                6.88L12 17.77l-6.18 \
                3.25L7 14.14 2 \
                9.27l6.91-1.01L12 2z"
        }
    }}
}

#[component]
fn IconArrowRight() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        path { d: "M9 18l6-6-6-6" }
    }}
}

#[component]
fn IconArrowLeft() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        path { d: "M15 18l-6-6 6-6" }
    }}
}

#[component]
fn IconPlus() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        line {
            x1: "12", y1: "5",
            x2: "12", y2: "19",
        }
        line {
            x1: "5", y1: "12",
            x2: "19", y2: "12",
        }
    }}
}

#[component]
fn IconX() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        line {
            x1: "18", y1: "6",
            x2: "6", y2: "18",
        }
        line {
            x1: "6", y1: "6",
            x2: "18", y2: "18",
        }
    }}
}

#[component]
fn IconTrash() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        polyline {
            points: "3 6 5 6 21 6",
        }
        path {
            d: "M19 6v14a2 2 0 0 \
                1-2 2H7a2 2 0 0 1\
                -2-2V6m3 0V4a2 2 \
                0 0 1 2-2h4a2 2 \
                0 0 1 2 2v2"
        }
    }}
}

#[component]
fn IconHelpCircle() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        circle { cx: "12", cy: "12", r: "10" }
        path {
            d: "M9.09 9a3 3 0 0 1 \
                5.83 1c0 2-3 3-3 3"
        }
        line {
            x1: "12", y1: "17",
            x2: "12.01", y2: "17",
        }
    }}
}

#[component]
fn IconList() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        line {
            x1: "8", y1: "6",
            x2: "21", y2: "6",
        }
        line {
            x1: "8", y1: "12",
            x2: "21", y2: "12",
        }
        line {
            x1: "8", y1: "18",
            x2: "21", y2: "18",
        }
        line {
            x1: "3", y1: "6",
            x2: "3.01", y2: "6",
        }
        line {
            x1: "3", y1: "12",
            x2: "3.01", y2: "12",
        }
        line {
            x1: "3", y1: "18",
            x2: "3.01", y2: "18",
        }
    }}
}

#[component]
fn IconLayers() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        polygon {
            points: "12 2 2 7 12 \
                12 22 7 12 2",
        }
        polyline {
            points: "2 17 12 22 22 17",
        }
        polyline {
            points: "2 12 12 17 22 12",
        }
    }}
}

#[component]
fn IconSun() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        circle { cx: "12", cy: "12", r: "5" }
        line {
            x1: "12", y1: "1",
            x2: "12", y2: "3",
        }
        line {
            x1: "12", y1: "21",
            x2: "12", y2: "23",
        }
        line {
            x1: "4.22", y1: "4.22",
            x2: "5.64", y2: "5.64",
        }
        line {
            x1: "18.36", y1: "18.36",
            x2: "19.78", y2: "19.78",
        }
        line {
            x1: "1", y1: "12",
            x2: "3", y2: "12",
        }
        line {
            x1: "21", y1: "12",
            x2: "23", y2: "12",
        }
        line {
            x1: "4.22", y1: "19.78",
            x2: "5.64", y2: "18.36",
        }
        line {
            x1: "18.36", y1: "5.64",
            x2: "19.78", y2: "4.22",
        }
    }}
}

#[component]
fn IconMoon() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        path {
            d: "M21 12.79A9 9 0 1 1 \
                11.21 3 7 7 0 0 0 \
                21 12.79z"
        }
    }}
}

#[component]
fn IconMsg() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        stroke_linecap: "round",
        stroke_linejoin: "round",
        xmlns: SVG_NS,
        path {
            d: "M21 15a2 2 0 0 1-2 \
                2H7l-4 4V5a2 2 0 0 \
                1 2-2h14a2 2 0 0 1 \
                2 2z"
        }
    }}
}

#[component]
fn IconFallback() -> Element {
    rsx! { svg {
        view_box: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        stroke_width: "2",
        xmlns: SVG_NS,
        rect {
            x: "3", y: "3",
            width: "18", height: "18",
            rx: "2",
        }
    }}
}
