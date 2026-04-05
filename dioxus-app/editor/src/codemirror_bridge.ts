/**
 * CodeMirror 6 bridge for Dioxus WASM.
 *
 * Exposes global functions on `window` that the Rust
 * frontend calls via `document::eval()`.
 */

import {
  EditorView,
  keymap,
  lineNumbers,
  highlightActiveLine,
  highlightActiveLineGutter,
} from '@codemirror/view'
import {
  EditorState,
  Compartment,
  Extension,
} from '@codemirror/state'
import {
  defaultKeymap,
  indentWithTab,
} from '@codemirror/commands'
import {
  syntaxHighlighting,
  defaultHighlightStyle,
} from '@codemirror/language'
import { javascript } from '@codemirror/lang-javascript'
import { python } from '@codemirror/lang-python'
import { go } from '@codemirror/lang-go'
import { rust } from '@codemirror/lang-rust'
import { oneDark } from '@codemirror/theme-one-dark'

type LangFactory = () => ReturnType<typeof javascript>

const LANGUAGES: Record<string, LangFactory> = {
  javascript,
  python,
  go,
  rust,
}

/** Light theme — background/gutters follow CSS vars */
const lightTheme = EditorView.theme({
  '&': {
    backgroundColor: 'var(--background)',
    color: 'var(--foreground)',
  },
  '.cm-content': {
    caretColor: 'var(--foreground)',
  },
  '.cm-cursor, .cm-dropCursor': {
    borderLeftColor: 'var(--foreground)',
  },
  '.cm-gutters': {
    backgroundColor: 'var(--muted)',
    color: 'var(--muted-foreground)',
    borderRight: '1px solid var(--border)',
  },
  '.cm-activeLineGutter': {
    backgroundColor: 'var(--secondary)',
  },
  '.cm-activeLine': {
    backgroundColor: 'var(--secondary)',
  },
  '&.cm-focused .cm-selectionBackground, \
   .cm-selectionBackground': {
    backgroundColor:
      'color-mix(in srgb, var(--ring) 25%, transparent)',
  },
}, { dark: false })

type EditorEntry = {
  view: EditorView
  themeCompartment: Compartment
}

const editors = new Map<string, EditorEntry>()

/**
 * Build theme + syntax highlighting extensions.
 * oneDark bundles its own highlight style;
 * light mode needs defaultHighlightStyle.
 */
function themeFor(isDark: boolean): Extension {
  if (isDark) return oneDark
  return [
    lightTheme,
    syntaxHighlighting(defaultHighlightStyle),
  ]
}

/** Create a CodeMirror editor in a DOM element */
function cmInit(
  elementId: string,
  lang: string,
  initialCode: string,
  isDark: boolean,
): void {
  const parent = document.getElementById(elementId)
  if (!parent) return

  cmDestroy(elementId)

  const langFactory = LANGUAGES[lang]
  const langExtension = langFactory
    ? langFactory()
    : []
  const themeCompartment = new Compartment()

  const state = EditorState.create({
    doc: initialCode,
    extensions: [
      lineNumbers(),
      highlightActiveLine(),
      highlightActiveLineGutter(),
      keymap.of([...defaultKeymap, indentWithTab]),
      themeCompartment.of(themeFor(isDark)),
      langExtension,
      EditorView.theme({
        '&': { height: '100%' },
        '.cm-scroller': { overflow: 'auto' },
      }),
    ],
  })

  const view = new EditorView({ state, parent })
  editors.set(elementId, { view, themeCompartment })
}

/** Switch dark/light theme without reinitializing */
function cmSetTheme(
  elementId: string,
  isDark: boolean,
): void {
  const entry = editors.get(elementId)
  if (!entry) return

  entry.view.dispatch({
    effects: entry.themeCompartment.reconfigure(
      themeFor(isDark),
    ),
  })
}

/** Get the current editor content */
function cmGetValue(elementId: string): string {
  const entry = editors.get(elementId)
  return entry?.view.state.doc.toString() ?? ''
}

/** Destroy an editor instance and clean up */
function cmDestroy(elementId: string): void {
  const entry = editors.get(elementId)
  if (entry) {
    entry.view.destroy()
    editors.delete(elementId)
  }
}

// Expose on window for Dioxus eval() access
;(window as any).cmInit = cmInit
;(window as any).cmSetTheme = cmSetTheme
;(window as any).cmGetValue = cmGetValue
;(window as any).cmDestroy = cmDestroy
