//! Decorative stickers — neo-brutalist side elements

use dioxus::prelude::*;

/// Pencil — yellow body, pink eraser
const SVG_PENCIL: &str = r##"<svg
  viewBox="0 0 60 90" fill="none">
  <rect x="18" y="8" width="20" height="60"
    rx="2" fill="#fbbf24"
    stroke="currentColor" stroke-width="2.5"/>
  <polygon points="18,68 38,68 28,88"
    fill="#fbbf24"
    stroke="currentColor" stroke-width="2.5"/>
  <polygon points="24,80 32,80 28,88"
    fill="#fde68a"/>
  <rect x="18" y="8" width="20" height="10"
    rx="1" fill="#f472b6"
    stroke="currentColor" stroke-width="2.5"/>
  <line x1="18" y1="18" x2="38" y2="18"
    stroke="currentColor" stroke-width="2"/>
  <circle cx="28" cy="88" r="1.5"
    fill="currentColor"/>
</svg>"##;

/// Lightbulb — with green rays
const SVG_BULB: &str = r##"<svg
  viewBox="0 0 70 80" fill="none">
  <path d="M35 15 C20 15, 12 28, 12 38
    C12 48, 20 54, 24 58 L24 65 L46 65
    L46 58 C50 54, 58 48, 58 38
    C58 28, 50 15, 35 15Z"
    fill="#fef3c7"
    stroke="currentColor" stroke-width="2.5"/>
  <line x1="24" y1="65" x2="46" y2="65"
    stroke="currentColor" stroke-width="2.5"/>
  <line x1="26" y1="70" x2="44" y2="70"
    stroke="currentColor" stroke-width="2"/>
  <line x1="28" y1="75" x2="42" y2="75"
    stroke="currentColor" stroke-width="2"/>
  <line x1="35" y1="2" x2="35" y2="9"
    stroke="#a3e635" stroke-width="2.5"
    stroke-linecap="round"/>
  <line x1="55" y1="12" x2="50" y2="17"
    stroke="#a3e635" stroke-width="2.5"
    stroke-linecap="round"/>
  <line x1="15" y1="12" x2="20" y2="17"
    stroke="#a3e635" stroke-width="2.5"
    stroke-linecap="round"/>
  <line x1="62" y1="35" x2="56" y2="35"
    stroke="#a3e635" stroke-width="2"
    stroke-linecap="round"/>
  <line x1="8" y1="35" x2="14" y2="35"
    stroke="#a3e635" stroke-width="2"
    stroke-linecap="round"/>
</svg>"##;

/// Checkmark — thick tick in dashed circle
const SVG_CHECK: &str = r##"<svg
  viewBox="0 0 60 60" fill="none">
  <circle cx="30" cy="30" r="26"
    stroke="currentColor" stroke-width="2.5"
    stroke-dasharray="4 2" fill="none"/>
  <polyline points="17,32 26,42 44,20"
    stroke="#a3e635" stroke-width="4"
    stroke-linecap="round"
    stroke-linejoin="round" fill="none"/>
</svg>"##;

/// Coffee cup — steam curls
const SVG_COFFEE: &str = r##"<svg
  viewBox="0 0 70 70" fill="none">
  <rect x="12" y="25" width="36" height="35"
    rx="4" fill="var(--card)"
    stroke="currentColor" stroke-width="2.5"/>
  <path d="M48 32 C56 32, 60 38, 60 44
    C60 50, 56 54, 48 54"
    stroke="currentColor" stroke-width="2.5"
    fill="none"/>
  <rect x="12" y="60" width="36" height="5"
    rx="2" fill="currentColor" opacity="0.15"/>
  <path d="M22 20 C22 14, 26 14, 26 18"
    stroke="currentColor" stroke-width="2"
    stroke-linecap="round" fill="none"
    opacity="0.4"/>
  <path d="M30 17 C30 11, 34 11, 34 15"
    stroke="currentColor" stroke-width="2"
    stroke-linecap="round" fill="none"
    opacity="0.4"/>
  <path d="M38 19 C38 13, 42 13, 42 17"
    stroke="currentColor" stroke-width="2"
    stroke-linecap="round" fill="none"
    opacity="0.4"/>
</svg>"##;

/// Grade stamp — "A+" in double circle
const SVG_GRADE: &str = r##"<svg
  viewBox="0 0 80 80" fill="none">
  <circle cx="40" cy="40" r="35"
    stroke="var(--primary)" stroke-width="3.5"
    fill="none"/>
  <circle cx="40" cy="40" r="29"
    stroke="var(--primary)" stroke-width="2"
    fill="none" stroke-dasharray="4 3"/>
  <text x="40" y="50"
    font-family="Unbounded, sans-serif"
    font-size="30" font-weight="900"
    fill="var(--primary)"
    text-anchor="middle">A+</text>
</svg>"##;

/// Notebook — ruled lines, red margin, scribble
const SVG_NOTEBOOK: &str = r##"<svg
  viewBox="0 0 70 85" fill="none">
  <rect x="10" y="5" width="50" height="75"
    rx="3" fill="var(--card)"
    stroke="currentColor" stroke-width="2.5"/>
  <line x1="22" y1="5" x2="22" y2="80"
    stroke="#f87171" stroke-width="1.5"
    opacity="0.5"/>
  <line x1="26" y1="22" x2="52" y2="22"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <line x1="26" y1="32" x2="52" y2="32"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <line x1="26" y1="42" x2="52" y2="42"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <line x1="26" y1="52" x2="52" y2="52"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <line x1="26" y1="62" x2="52" y2="62"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <path d="M26 24 C30 20, 36 28, 42 22
    C46 18, 50 26, 52 22"
    stroke="#a3e635" stroke-width="2"
    stroke-linecap="round" fill="none"/>
  <circle cx="10" cy="20" r="3"
    fill="currentColor" opacity="0.2"/>
  <circle cx="10" cy="35" r="3"
    fill="currentColor" opacity="0.2"/>
  <circle cx="10" cy="50" r="3"
    fill="currentColor" opacity="0.2"/>
  <circle cx="10" cy="65" r="3"
    fill="currentColor" opacity="0.2"/>
</svg>"##;

/// Bar chart — sketchy hand-drawn bars + arrow
const SVG_CHART: &str = r##"<svg
  viewBox="0 0 100 110" fill="none">
  <!-- Base line (wobbly) -->
  <path d="M8 98 C20 99, 50 97, 92 98"
    stroke="currentColor" stroke-width="2.5"
    stroke-linecap="round"/>
  <!-- Bar 1 (small) — rounded, filled -->
  <path d="M14 98 L14 78 C14 76, 16 74, 18 74
    L28 74 C30 74, 32 76, 32 78
    L32 98"
    fill="var(--primary)" opacity="0.3"
    stroke="currentColor" stroke-width="2.5"
    stroke-linejoin="round"/>
  <!-- Bar 2 (medium) -->
  <path d="M38 98 L38 55 C38 53, 40 51, 42 51
    L52 51 C54 51, 56 53, 56 55
    L56 98"
    fill="var(--primary)" opacity="0.5"
    stroke="currentColor" stroke-width="2.5"
    stroke-linejoin="round"/>
  <!-- Bar 3 (tall) -->
  <path d="M62 98 L62 28 C62 26, 64 24, 66 24
    L76 24 C78 24, 80 26, 80 28
    L80 98"
    fill="var(--primary)" opacity="0.75"
    stroke="currentColor" stroke-width="2.5"
    stroke-linejoin="round"/>
  <!-- Arrow (wobbly, hand-drawn feel) -->
  <path d="M22 72 C28 65, 38 52, 47 46
    C56 40, 64 30, 72 20
    C76 15, 82 8, 90 4"
    stroke="currentColor" stroke-width="3"
    stroke-linecap="round" fill="none"/>
  <!-- Arrow head -->
  <path d="M83 3 L91 4 L88 12"
    stroke="currentColor" stroke-width="2.5"
    stroke-linecap="round"
    stroke-linejoin="round" fill="none"/>
</svg>"##;

/// Formula — math symbols
const SVG_FORMULA: &str = r##"<svg
  viewBox="0 0 90 45" fill="none">
  <text x="8" y="32"
    font-family="serif" font-style="italic"
    font-size="24" fill="currentColor"
    opacity="0.8">∑</text>
  <text x="30" y="28"
    font-family="serif" font-style="italic"
    font-size="16" fill="currentColor"
    opacity="0.7">x² + </text>
  <text x="66" y="28"
    font-family="serif" font-style="italic"
    font-size="16" fill="#a3e635"
    font-weight="bold">π</text>
</svg>"##;

/// Bookmark — ribbon shape
const SVG_BOOKMARK: &str = r##"<svg
  viewBox="0 0 40 70" fill="none">
  <path d="M5 0 L5 62 L20 50 L35 62 L35 0 Z"
    fill="var(--primary)" opacity="0.85"
    stroke="currentColor" stroke-width="2.5"/>
  <line x1="12" y1="16" x2="28" y2="16"
    stroke="white" stroke-width="2"
    stroke-linecap="round" opacity="0.6"/>
  <line x1="12" y1="24" x2="24" y2="24"
    stroke="white" stroke-width="2"
    stroke-linecap="round" opacity="0.4"/>
</svg>"##;

/// Sticky note — yellow square, folded corner
const SVG_STICKY: &str = r##"<svg
  viewBox="0 0 70 70" fill="none">
  <path d="M5 5 H65 V50 L50 65 H5 Z"
    fill="#fef08a"
    stroke="currentColor" stroke-width="2.5"/>
  <path d="M50 65 V50 H65"
    fill="#fde047"
    stroke="currentColor" stroke-width="2"/>
  <line x1="14" y1="22" x2="50" y2="22"
    stroke="currentColor" opacity="0.2"
    stroke-width="1.5"/>
  <line x1="14" y1="32" x2="45" y2="32"
    stroke="currentColor" opacity="0.2"
    stroke-width="1.5"/>
  <line x1="14" y1="42" x2="38" y2="42"
    stroke="currentColor" opacity="0.2"
    stroke-width="1.5"/>
  <text x="16" y="24"
    font-family="Outfit, sans-serif"
    font-size="8" fill="currentColor"
    opacity="0.5">TODO:</text>
</svg>"##;

/// Paper airplane — folded paper
const SVG_PLANE: &str = r##"<svg
  viewBox="0 0 70 60" fill="none">
  <path d="M5 30 L65 5 L45 55 L35 35 Z"
    fill="var(--card)"
    stroke="currentColor" stroke-width="2.5"
    stroke-linejoin="round"/>
  <line x1="65" y1="5" x2="35" y2="35"
    stroke="currentColor" stroke-width="2"/>
  <path d="M35 35 L45 55"
    stroke="currentColor" stroke-width="2"/>
</svg>"##;

/// Target — bullseye with arrow
const SVG_TARGET: &str = r##"<svg
  viewBox="0 0 70 70" fill="none">
  <circle cx="35" cy="35" r="28"
    stroke="currentColor" stroke-width="2.5"
    fill="none"/>
  <circle cx="35" cy="35" r="18"
    stroke="currentColor" stroke-width="2"
    fill="none"/>
  <circle cx="35" cy="35" r="8"
    fill="#a3e635"
    stroke="currentColor" stroke-width="2"/>
  <line x1="55" y1="10" x2="38" y2="32"
    stroke="currentColor" stroke-width="2.5"
    stroke-linecap="round"/>
  <polyline points="55,10 55,20"
    stroke="currentColor" stroke-width="2"
    stroke-linecap="round"/>
  <polyline points="55,10 48,12"
    stroke="currentColor" stroke-width="2"
    stroke-linecap="round"/>
</svg>"##;

/// Trophy — winner cup
const SVG_TROPHY: &str = r##"<svg
  viewBox="0 0 70 80" fill="none">
  <path d="M20 12 L20 40 C20 52, 50 52, 50 40
    L50 12 Z"
    fill="#fbbf24"
    stroke="currentColor" stroke-width="2.5"/>
  <path d="M20 20 C12 20, 8 28, 10 36
    C12 42, 18 42, 20 38"
    stroke="currentColor" stroke-width="2.5"
    fill="none"/>
  <path d="M50 20 C58 20, 62 28, 60 36
    C58 42, 52 42, 50 38"
    stroke="currentColor" stroke-width="2.5"
    fill="none"/>
  <rect x="30" y="52" width="10" height="12"
    fill="currentColor" opacity="0.2"
    stroke="currentColor" stroke-width="2"/>
  <rect x="22" y="64" width="26" height="8"
    rx="2"
    stroke="currentColor" stroke-width="2.5"
    fill="var(--card)"/>
  <text x="35" y="36"
    font-family="Unbounded, sans-serif"
    font-size="14" font-weight="900"
    fill="currentColor" opacity="0.6"
    text-anchor="middle">1</text>
</svg>"##;

/// Open book — pages fanning
const SVG_BOOK: &str = r##"<svg
  viewBox="0 0 80 60" fill="none">
  <path d="M40 15 L40 55"
    stroke="currentColor" stroke-width="2"/>
  <path d="M40 15 C30 12, 15 10, 5 15
    L5 55 C15 50, 30 52, 40 55"
    fill="var(--card)"
    stroke="currentColor" stroke-width="2.5"/>
  <path d="M40 15 C50 12, 65 10, 75 15
    L75 55 C65 50, 50 52, 40 55"
    fill="var(--card)"
    stroke="currentColor" stroke-width="2.5"/>
  <line x1="12" y1="25" x2="34" y2="28"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <line x1="12" y1="33" x2="34" y2="36"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <line x1="12" y1="41" x2="34" y2="44"
    stroke="currentColor" opacity="0.15"
    stroke-width="1"/>
  <line x1="46" y1="28" x2="68" y2="25"
    stroke="#a3e635" opacity="0.4"
    stroke-width="1.5"/>
  <line x1="46" y1="36" x2="68" y2="33"
    stroke="#a3e635" opacity="0.4"
    stroke-width="1.5"/>
</svg>"##;

/// Ruler — tilted measuring ruler
const SVG_RULER: &str = r##"<svg
  viewBox="0 0 100 30" fill="none">
  <rect x="2" y="4" width="96" height="22"
    rx="2" fill="var(--card)"
    stroke="currentColor" stroke-width="2.5"/>
  <line x1="15" y1="4" x2="15" y2="14"
    stroke="currentColor" stroke-width="1.5"/>
  <line x1="25" y1="4" x2="25" y2="10"
    stroke="currentColor" stroke-width="1"/>
  <line x1="35" y1="4" x2="35" y2="14"
    stroke="currentColor" stroke-width="1.5"/>
  <line x1="45" y1="4" x2="45" y2="10"
    stroke="currentColor" stroke-width="1"/>
  <line x1="55" y1="4" x2="55" y2="14"
    stroke="currentColor" stroke-width="1.5"/>
  <line x1="65" y1="4" x2="65" y2="10"
    stroke="currentColor" stroke-width="1"/>
  <line x1="75" y1="4" x2="75" y2="14"
    stroke="currentColor" stroke-width="1.5"/>
  <line x1="85" y1="4" x2="85" y2="10"
    stroke="currentColor" stroke-width="1"/>
  <text x="14" y="24" font-size="6"
    fill="currentColor" opacity="0.4"
    font-family="Outfit">cm</text>
</svg>"##;

/// Sparkle cluster — stars like student doodles
const SVG_SPARKLES: &str = r##"<svg
  viewBox="0 0 60 60" fill="none">
  <path d="M30 4 L33 20 L48 16 L36 27
    L46 40 L30 32 L14 40 L24 27
    L12 16 L27 20 Z"
    fill="#a3e635"
    stroke="currentColor" stroke-width="2.5"/>
  <path d="M50 42 L51 48 L57 46 L53 50
    L56 56 L50 52 L44 56 L47 50
    L43 46 L49 48 Z"
    fill="#fbbf24"
    stroke="currentColor" stroke-width="2"/>
  <path d="M8 44 L9 48 L13 47 L11 50
    L13 53 L8 51 L4 53 L6 50
    L3 47 L8 48 Z"
    fill="#fbbf24"
    stroke="currentColor" stroke-width="1.5"/>
</svg>"##;

/// Todo list — vertical line + handwritten items
const SVG_TODO: &str = r##"<svg
  viewBox="0 0 130 150" fill="none">
  <line x1="18" y1="8" x2="18" y2="142"
    stroke="currentColor" stroke-width="2.5"
    stroke-linecap="round"/>
  <rect x="26" y="10" width="12" height="12"
    rx="2" stroke="currentColor"
    stroke-width="2" fill="none"/>
  <polyline points="29,16 32,20 37,13"
    stroke="#a3e635" stroke-width="2.5"
    stroke-linecap="round"
    stroke-linejoin="round" fill="none"/>
  <text x="44" y="22"
    font-family="Outfit, sans-serif"
    font-size="11" fill="currentColor"
    font-weight="600">Work hard</text>
  <rect x="26" y="42" width="12" height="12"
    rx="2" stroke="currentColor"
    stroke-width="2" fill="none"/>
  <polyline points="29,48 32,52 37,45"
    stroke="#a3e635" stroke-width="2.5"
    stroke-linecap="round"
    stroke-linejoin="round" fill="none"/>
  <text x="44" y="54"
    font-family="Outfit, sans-serif"
    font-size="11" fill="currentColor"
    font-weight="600">Play hard</text>
  <rect x="26" y="74" width="12" height="12"
    rx="2" stroke="currentColor"
    stroke-width="2" fill="none"/>
  <polyline points="29,80 32,84 37,77"
    stroke="#a3e635" stroke-width="2.5"
    stroke-linecap="round"
    stroke-linejoin="round" fill="none"/>
  <text x="44" y="86"
    font-family="Outfit, sans-serif"
    font-size="11" fill="currentColor"
    font-weight="600">Kick ass</text>
  <rect x="26" y="106" width="12" height="12"
    rx="2" stroke="currentColor"
    stroke-width="2" fill="none"/>
  <text x="44" y="118"
    font-family="Outfit, sans-serif"
    font-size="11" fill="currentColor"
    font-weight="600"
    opacity="0.5">Succeed</text>
  <text x="26" y="142"
    font-family="Unbounded, sans-serif"
    font-size="7" fill="var(--primary)"
    font-weight="700"
    letter-spacing="1">TODO</text>
</svg>"##;

/// All sticker elements
#[component]
pub fn DecoStickers() -> Element {
    rsx! {
        div { class: "deco-stickers",
            // ── Left side ──
            div {
                class: "deco deco-pencil",
                dangerous_inner_html: SVG_PENCIL,
            }
            div {
                class: "deco deco-chart",
                dangerous_inner_html: SVG_CHART,
            }
            div {
                class: "deco deco-notebook",
                dangerous_inner_html: SVG_NOTEBOOK,
            }
            div {
                class: "deco deco-formula",
                dangerous_inner_html: SVG_FORMULA,
            }
            div {
                class: "deco deco-book",
                dangerous_inner_html: SVG_BOOK,
            }
            div {
                class: "deco deco-ruler",
                dangerous_inner_html: SVG_RULER,
            }
            // ── Right side ──
            div {
                class: "deco deco-grade",
                dangerous_inner_html: SVG_GRADE,
            }
            div {
                class: "deco deco-bulb",
                dangerous_inner_html: SVG_BULB,
            }
            div {
                class: "deco deco-coffee",
                dangerous_inner_html: SVG_COFFEE,
            }
            div {
                class: "deco deco-sticky",
                dangerous_inner_html: SVG_STICKY,
            }
            div {
                class: "deco deco-target",
                dangerous_inner_html: SVG_TARGET,
            }
            div {
                class: "deco deco-trophy",
                dangerous_inner_html: SVG_TROPHY,
            }
            div {
                class: "deco deco-check",
                dangerous_inner_html: SVG_CHECK,
            }
            div {
                class: "deco deco-plane",
                dangerous_inner_html: SVG_PLANE,
            }
            div {
                class: "deco deco-bookmark",
                dangerous_inner_html: SVG_BOOKMARK,
            }
            div {
                class: "deco deco-sparkles",
                dangerous_inner_html: SVG_SPARKLES,
            }
            div {
                class: "deco deco-todo",
                dangerous_inner_html: SVG_TODO,
            }
        }
    }
}
