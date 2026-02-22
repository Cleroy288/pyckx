# Coding Norm — Inspired by 42 School's Norminette (v3), Adapted for All Languages

This is a **strict** coding standard. Every rule applies to every language unless a language-specific exception is noted. The spirit of these rules is: write small, readable, predictable code where every line is obvious at a glance.

---

## I. Naming

- **snake_case** is the default for variables, functions, files, and directories. Follow the project's existing convention if it differs (e.g., camelCase in JS/TS, PascalCase for classes in most OOP languages).
- All identifiers must be in **English**.
- Names must be **explicit and mnemonic**. A reader should understand what a variable holds or what a function does from its name alone. Only loop counters (`i`, `j`, `k`) may use single-letter names.
- Abbreviations are acceptable only if they are widely understood in the domain (e.g., `ctx`, `req`, `res`, `cfg`, `buf`, `len`, `idx`, `src`, `dst`, `tmp`).
- **Prefix conventions** — apply when the language doesn't have visibility modifiers or when the project uses them:
  - Private/unexported: underscore prefix `_helper()` or language convention (Go unexported = lowercase first letter).
  - Global/module-level mutable state: prefix `g_` or equivalent marker. Global mutable state should be avoided whenever possible.
  - Constants: `UPPER_SNAKE_CASE`.
  - Types/Structs/Classes: `PascalCase`.
  - Enums: type in `PascalCase`, values in `UPPER_SNAKE_CASE`.
- File and directory names: **lowercase with underscores or hyphens** (follow project convention). No spaces, no special characters.
- Characters outside standard ASCII are forbidden in identifiers.

---

## II. Formatting

- **Indentation**: Use the project's standard (tabs or spaces). If no standard exists, use tabs representing 4 spaces. Be consistent — never mix tabs and spaces.
- **Line length**: Maximum **80 columns**, comments included. A tab counts as the number of columns it represents, not as one character.
- **One instruction per line.** Never chain multiple statements on a single line.
- **One declaration per line.** Never declare multiple variables on the same line.
- **Empty lines must be truly empty** — no trailing spaces or tabs.
- **No line may end with trailing whitespace.**
- **No consecutive duplicate blank lines.** One blank line maximum between logical sections.
- **Braces/brackets** on their own line or at end of line — follow the project convention, but be consistent within a file.
- **Start a new line** after each closing brace or end of control structure.
- **Commas and semicolons**: always followed by a space (unless end of line).
- **Operators**: surrounded by exactly one space on each side (`a + b`, not `a+b` or `a  +  b`).
- **Keywords** (`if`, `while`, `for`, `return`, etc.): followed by a space before the opening parenthesis.
- **No consecutive spaces** within a line (alignment of declarations is the only exception, if the project convention requires it).
- **Variable declarations at the top of the scope.** Declare all variables at the beginning of the function or block. Separate declarations from logic with one blank line. (Exception: languages where deferred declaration is idiomatic, like Rust's `let` — but still group declarations near the top when practical.)
- **Declaration and initialization on the same line** are allowed only for constants, static/class-level variables, and simple initializers. Complex initialization should be a separate statement.

---

## III. Functions / Methods

- **Maximum 40 lines** per function body (excluding the signature line and closing brace/bracket). If a function exceeds this, split it.
- **Maximum 5 parameters.** If you need more, group related parameters into a struct, object, or options type.
- **Maximum 5 local variables** per function. If you need more, the function is doing too much — refactor.
- A function must do **one thing**. If you can describe what it does with "and" in the middle, split it.
- **Functions must be simple, concise, and clear.** Every function should be immediately understandable at a glance — no mental gymnastics required.
- **Functions operate at one level of abstraction.** Don't mix high-level orchestration with low-level details. If a function calls `process_order()`, it should not also manipulate raw bytes in the same body. Extract lower-level steps into their own functions.
- Functions that take no arguments must make this explicit (e.g., empty parens with type annotation in typed languages).
- **All parameters must be named** in declarations and prototypes. No unnamed parameters.
- **One blank line** between each function definition.
- **Return values** must be explicit. Avoid implicit returns in languages that support them unless the function is a one-liner (e.g., closures).
- Each function's **return type and name** should be clearly separated and readable.
- **No more than 5 function definitions per file.** If a file grows beyond this, split it into multiple files organized by responsibility. (Exception: test files and files where the language/framework convention dictates otherwise, but still keep files focused.)

---

## IV. Types, Structs, Classes, Enums

- One type/struct/class per file when practical. Keep type definitions focused.
- **Align member declarations** within a type definition for readability.
- Prefer **composition over inheritance**. Keep class hierarchies shallow.
- Enums should have explicit values when ordering matters.
- Traits should be small and focused — prefer multiple small traits over one large one.

---

## V. Imports / Includes / Dependencies

- All imports at the **top of the file**, grouped and ordered:
  1. Standard library / built-in imports.
  2. Third-party / external library imports.
  3. Internal / project imports.
  - Blank line between each group.
- **No unused imports.** Every import must be justified.
- **Never import implementation files** that have side effects unless intentional.

---

## VI. Constants and Macros

- Constants/defines must be used only for **literal and constant values**.
- Never use constants or macros to bypass coding standards or obfuscate code.
- Constant names: `UPPER_SNAKE_CASE`.
- **No magic numbers.** Every numeric literal (except `0`, `1`, `-1` in obvious contexts) must be a named constant.
- **No magic strings.** All user-facing text, error messages, log messages, and status strings must be defined as **named constants or sentinel errors** in a centralized location (e.g., a dedicated `errors.rs`, `messages.rs`, or `constants` module). Never write string literals inline in logic. This applies to:
  - Error messages (use error enums, sentinel constants, or a dedicated error module).
  - Log messages and status text.
  - UI labels, prompts, and user-facing output.
  - The goal: every string the user or a log can see is defined **once**, in **one place**, and referenced everywhere else.
- **No multiline macros.** If you need multiline logic, write a function.
- Prefer `const` variables or enums over preprocessor macros when the language supports it.

---

## VII. Forbidden Patterns

These patterns produce hard-to-read, hard-to-debug code. **Avoid them.**

- **Deeply nested logic** (more than 3 levels of indentation). Use early returns, guard clauses, or extract inner logic into functions.
- **Multiple assignments on one line** (`a = b = c = 0`).
- **Implicit type conversions** that obscure intent. Be explicit about types.
- **Variable-length / dynamically-sized stack allocations** when the size is not bounded and known.
- **goto** — forbidden in all circumstances.
- **Overly clever one-liners** that sacrifice readability for brevity. Code is read far more than it is written.
- **Dead code** — commented-out code blocks, unreachable code, unused variables, unused functions. Delete it; version control remembers.
- **Hardcoded values** — paths, URLs, credentials, configuration values embedded in logic. Extract to constants or configuration.
- **Side effects in conditions** — separate the assignment from the condition.
- **Functions with both side effects and return values** that create ambiguity about their purpose. A function should either compute a value or perform an action, not both (command-query separation).

---

## VIII. Comments

- **Above every function**: write a short description of what the function does.
- **Above every struct/enum/type**: write a short description of what it represents.
- **Declare all variables at the top** of the function, with a brief comment next to each explaining its purpose.
- **Inside function bodies**: use comments to explain complex logic, non-obvious decisions, and the *why* behind the code.
- TODO/FIXME markers with a ticket reference are always acceptable.
- Comments at the **top of a file** are encouraged to describe the module's purpose.
- All comments in **English**.
- A comment must never be used to justify bad code. Fix the code instead.
- **No decorative comments** (comment banners, ASCII art separators, box-drawing characters). Let the code structure speak for itself.
- Use `///` doc comments for public functions and types in Rust.

---

## IX. Control Structures

- Control structures (`if`, `while`, `for`, etc.) **must have braces**, even for single-line bodies.
- Prefer `for` loops and iterators for iteration over collections.
- **Avoid `loop`** unless the loop must run indefinitely or has complex exit conditions.
- Use `match` instead of long `if/else if` chains. Every arm must be handled or have a `_` default.
- **Ternary operators**: not available in Rust — use `if/else` expressions. Keep them single-line when simple.

---

## X. Error Handling

- Every function that can fail must return `Result<T, E>` and handle or propagate errors explicitly.
- Never silently swallow errors (empty `match` arms, `let _ = ...` on Results).
- Use `?` operator for propagation. Use `map_err` to add context.
- Error messages must be **descriptive and actionable** — include what failed, why, and what to do about it.
- Use `thiserror` for library error types, `anyhow` at application boundaries.

---

## XI. File Organization

- Each file has a **single responsibility**. One module, one struct, one concern per file.
- Maximum **5 function definitions per file** for implementation files. (Test files and configuration files are exempt but should still stay focused.)
- File length should ideally stay under **300 lines**. If it grows beyond that, look for opportunities to split.
- Related files are grouped in directories by feature or domain.

---

## XII. Summary of Hard Limits

| Rule | Limit |
|---|---|
| Function body length | ≤ 40 lines |
| Function parameters | ≤ 5 |
| Local variables per function | ≤ 5 |
| Line width | ≤ 80 columns |
| Functions per file | ≤ 5 |
| File length | ≤ 300 lines |
| Nesting depth | ≤ 3 levels |
| Blank lines in a row | ≤ 1 |

These are **hard limits**, not guidelines. If you exceed them, refactor before delivering.

---

## XIII. Self-Check Before Delivering

Before considering any code complete, verify:

1. No function exceeds 40 lines.
2. No function has more than 5 parameters or 5 variables.
3. No line exceeds 80 columns.
4. No file has more than 5 function definitions.
5. No trailing whitespace, no double blank lines.
6. All names are explicit and in English.
7. No dead code, no commented-out blocks, no unused imports.
8. No magic numbers.
9. Comments explain *why*, not *what*.
10. Error handling is explicit everywhere.
11. The code matches the project's existing conventions.
