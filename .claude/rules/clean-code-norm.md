# Clean Code — Design, Readability, and Engineering Practices

This document covers **how to write clean code at the design level** — function design, readability, error strategy, security, git hygiene, logging, concurrency, and refactoring. It deliberately avoids repeating what is already defined in the sibling norms:

- `coding-norm.md` → formatting, naming, hard limits (40 lines, 5 params, 80 cols)
- `architecture-norm.md` → layer separation, folder structure, dependency rule
- `testing-norm.md` → test pyramid, AAA, mocking, coverage
- `CLAUDE.md` → philosophy (KISS, YAGNI, DRY, SOLID), codebase-first workflow

**Read those first.** This file builds on top of them.

---

## I. Function Design

### A. Pure Functions First

Default to **pure functions** — same input always produces same output, no side effects.

```
PURE (preferred):
  fn calculate_discount(price: f64, tier: &str) -> f64

IMPURE (isolate these):
  async fn send_email(to: &str, body: &str) -> Result<()>
  async fn save_user(db: &Pool, user: &User) -> Result<()>
```

Rules:
1. **Separate computation from side effects.** A function either calculates a result OR performs I/O. Never both.
2. **Push side effects to the edges.** Domain and service logic is pure. I/O happens in handlers and infrastructure.
3. **If a function must have side effects, name them explicitly.** `Save`, `Send`, `Write`, `Delete` — the verb must signal the effect.
4. **Never hide state changes.** A getter that silently updates a cache, a validator that logs errors — these are lies. Make every effect visible in the signature.

### B. Guard Clauses & Early Returns

Eliminate nesting by handling failures first and returning early.

```
BAD — deeply nested:
  fn process(input: Input) -> Result<Output> {
      if input.is_valid() {
          let data = fetch(input.id)?;
          if data.is_active() {
              return Ok(transform(data));
          }
          return Err(Error::Inactive);
      }
      Err(Error::Invalid)
  }

GOOD — flat with guard clauses:
  fn process(input: Input) -> Result<Output> {
      if !input.is_valid() {
          return Err(Error::Invalid);
      }
      let data = fetch(input.id)?;
      if !data.is_active() {
          return Err(Error::Inactive);
      }
      Ok(transform(data))
  }
```

Rules:
1. **Handle error/edge cases first.** Check preconditions at the top, return immediately on failure.
2. **The happy path is the least indented path.** If you have to scroll right to find the core logic, refactor.
3. **Maximum 3 levels of nesting** (already in coding-norm — this explains *how* to achieve it).
4. **Never use `else` after a `return`.** The `else` is implicit and adds unnecessary nesting.

### C. Function Composition

Build complex behavior from small, composable pieces.

Rules:
1. **Each function is a single transformation.** `validate_input → enrich_data → calculate_result → format_output`
2. **Functions at the same level of abstraction.** Don't mix high-level orchestration (`process_order`) with low-level details (`bytes.trim`) in the same function.
3. **The calling function reads like a recipe.** Someone reading it should understand the workflow without reading the called functions.
4. **Avoid deep call chains.** If function A calls B calls C calls D calls E, flatten. More than 4 levels of depth usually means the abstraction is wrong.

### D. Boolean Parameters

**Forbidden.** A boolean parameter means the function does two things.

```
BAD:
  fn create_user(name: &str, is_admin: bool)

GOOD:
  fn create_user(name: &str)
  fn create_admin(name: &str)

ALSO GOOD (options struct):
  fn create_user(name: &str, opts: UserOptions)
```

Exception: toggling a simple behavior flag in internal/private functions where the meaning is obvious from context.

---

## II. Readability

### A. Cognitive Complexity

A function's cognitive complexity = the mental effort to understand it. Keep it minimal.

**What increases cognitive complexity:**
- Nested control flow (each nesting level multiplies complexity)
- `else` / `else if` chains
- Boolean logic with `&&` and `||` mixed
- Break/continue in loops
- Recursion
- Callbacks inside callbacks

**How to reduce it:**
- Guard clauses (return early)
- Extract complex conditions into named booleans: `let is_eligible = age >= 18 && has_consent;`
- Extract nested blocks into named functions
- Replace conditionals with polymorphism or lookup tables when possible
- Use `match` instead of long `if/else if` chains

### B. Code as Documentation

The code should explain *what* and *how*. Comments explain *why*.

Rules:
1. **Write clear, self-explanatory code first — then add comments to guide the reader through the steps.** Code must be understandable on its own, and comments help follow along with the logic and decisions taken.
2. **Extract magic expressions into named variables.** `if user.age >= 18 && user.has_parental_consent` is better than `if a >= 18 && h`.
3. **Named constants over magic values** (already in coding-norm — emphasizing the readability benefit).
4. **Consistent vocabulary.** If the domain calls it a "tenant", don't alternate between "tenant", "organization", "company", and "account" in the code. Pick one term per concept and use it everywhere.
5. **Ubiquitous language.** Code terminology should match what the team / product / domain uses. If the business says "invoice", the code says `Invoice`, not `Bill` or `PaymentDocument`.

### C. Ordering Within a File

Organize code within a file for top-down reading:

```
1. Module declaration / imports
2. Constants
3. Types / Structs / Enums
4. Trait implementations
5. Constructor / Factory function (new)
6. Public methods (in order of importance / usage)
7. Private/helper methods (in call order)
8. #[cfg(test)] module (if inline tests)
```

The reader should be able to read the file top-to-bottom and understand the API before the implementation details. Public surface first, internals last.

---

## III. Error Handling Strategy

The coding-norm defines *basic* error handling rules. This section defines the **strategy**.

### A. Error Types

Create **domain-specific error types**, not generic strings.

```
BAD:
  Err(anyhow!("user not found"))
  Err(format!("invalid email: {}", email))

GOOD:
  Err(AppError::UserNotFound)
  Err(AppError::Validation { field: "email", reason: "must contain @" })
```

Rules:
1. **Define a set of domain errors per domain area.** `UserNotFound`, `DuplicateEmail`, `InsufficientFunds`. Use Rust enums.
2. **Errors carry context.** Include the field name, the invalid value (sanitized), and the rule that was violated.
3. **Errors are typed, not stringly-typed.** Use `thiserror` for library errors, `anyhow` only at application boundaries.
4. **Never expose internal errors to the outside.** A database constraint violation becomes `DuplicateEmail` at the service layer, not a raw sqlx error.

### B. Error Propagation

```
Page/Component → displays user-friendly message
  ↑
Service/Action → wraps/translates infrastructure errors into domain errors
  ↑
API Client → returns raw HTTP/network errors or wraps them
  ↑
External API → raw response errors
```

Rules:
1. **Wrap errors with context when crossing layer boundaries.** Use `?` with `map_err` or `context()`.
2. **Never swallow errors silently.** Every error is either handled, propagated, or logged-and-propagated.
3. **Handle errors at the right level.** A retry belongs in the API layer. A user-friendly message belongs in the component. Business error decisions belong in the service.
4. **Use `Result<T, E>` and `?` operator.** Rust's error handling is explicit — embrace it.
5. **Fail fast on programmer errors.** Use `unwrap()` only for invariants that are truly impossible to violate. Prefer `expect("reason")` with a message.

### C. Error Logging

1. **Log errors once.** If a function logs an error AND propagates it, the error gets logged multiple times. Pick one: log it or propagate it.
2. **Log at the boundary.** The outermost layer logs the full error chain. Inner layers propagate.
3. **Include request context in logs.** Request ID, user ID, operation name. Not just the error message.
4. **Never log sensitive data.** No passwords, tokens, credit card numbers, PII in error messages or logs.

---

## IV. Security Hygiene

### A. Input Validation

1. **Validate all external input at the boundary.** Every API response, user input, URL parameter.
2. **Whitelist, don't blacklist.** Define what's allowed, not what's forbidden.
3. **Validate type, length, format, and range.** A "name" field should have a max length. An "age" field should have a min/max.
4. **Sanitize before rendering.** Escape HTML to prevent XSS in dynamic content.

### B. Secrets Management

1. **Zero secrets in code.** No API keys, passwords, tokens, connection strings in source files. Ever.
2. **Zero secrets in logs.** Redact sensitive fields before logging.
3. **Environment variables or secret managers only.** Load secrets from env vars or equivalent. Never from config files committed to git.
4. **`.env` files are gitignored.** Always. Provide a `.env.example` with placeholder values.

### C. Dependencies

1. **Pin dependency versions.** Use `Cargo.lock`. Commit it.
2. **Audit dependencies.** Run `cargo audit` regularly.
3. **Minimize dependency surface.** Each external dependency is an attack surface and a maintenance burden. If the standard library can do it, don't add a dependency.
4. **No `eval`, no dynamic code execution.**

---

## V. Logging & Observability

### A. Structured Logging

Use **structured logs** via `tracing` or `log` crate, never `println!`.

```
BAD:
  println!("User {} created order {}", user_id, order_id);

GOOD:
  tracing::info!(user_id = %user_id, order_id = %order_id, "order created");
```

### B. Log Levels

| Level | When to use |
|---|---|
| **ERROR** | Something failed and requires attention. A request failed, a dependency is unreachable. |
| **WARN** | Something unexpected but recoverable. Retry succeeded, fallback activated, deprecated usage. |
| **INFO** | Significant business events. User registered, order completed, deployment started. |
| **DEBUG** | Developer-useful detail. Request payloads, query plans, internal state. Disabled in production. |

Rules:
1. **One logger per application**, configured at startup.
2. **Every log entry has context.** At minimum: timestamp, level, message, and a correlation/request ID.
3. **No logging in pure functions.** Logging is a side effect. It belongs at boundaries.
4. **Don't log successful routine operations in production.**

---

## VI. Concurrency

### A. Principles

1. **Don't share mutable state.** Use channels, message passing, or atomic operations.
2. **If you must share state, protect it.** Use `Mutex`, `RwLock`, or `Arc`. Document what the lock protects.
3. **Prefer immutability.** Immutable data structures eliminate race conditions entirely.
4. **Keep the critical section minimal.** Hold locks for the shortest possible duration. Never do I/O while holding a lock.

### B. Rust-Specific Patterns

1. **Use Tokio for async runtime** when needed.
2. **`Arc<Mutex<T>>` for shared mutable state** across async tasks.
3. **Channels (`mpsc`, `broadcast`, `watch`)** for task communication.
4. **`Send + Sync` bounds** — understand when and why they're needed.

### C. Rules

1. **No fire-and-forget tasks.** Every spawned task must be awaited or have its errors handled.
2. **Set timeouts on everything.** HTTP calls, database queries, lock acquisition. No infinite waits.
3. **Race conditions are bugs.** Use `--release` with thread sanitizer or `loom` for testing.

---

## VII. Git & Commit Standards

### A. Commit Messages

Follow **Conventional Commits**:

```
<type>(<scope>): <short description>

<optional body>

<optional footer>
```

Types:
| Type | When |
|---|---|
| `feat` | New feature or behavior |
| `fix` | Bug fix |
| `refactor` | Code change that doesn't fix a bug or add a feature |
| `docs` | Documentation only |
| `test` | Adding or fixing tests |
| `chore` | Build, CI, tooling, dependencies |
| `style` | Formatting only (no logic change) |
| `perf` | Performance improvement |

Rules:
1. **Subject line ≤ 72 characters.** Imperative mood: "Add user validation" not "Added user validation".
2. **One logical change per commit.** Don't mix a feature with a refactor with a dependency update.
3. **Reference the issue/ticket.** `feat(auth): add OAuth2 login (#142)`
4. **Body explains *why*, not *what*.** The diff shows what changed. The body explains the reasoning.

### B. Branches

```
main            ← always deployable
├── feat/142-oauth-login
├── fix/155-null-pointer-checkout
├── refactor/cleanup-user-service
└── chore/update-dependencies
```

Rules:
1. **Branch from `main`, merge to `main`.** Short-lived feature branches only.
2. **Branch name = `type/ticket-short-description`.** Matches the commit type.
3. **Keep branches short-lived.** If a branch lives longer than a few days, it's too big.
4. **No direct commits to `main`.** Everything goes through a PR with review.

### C. Pull Requests

1. **One concern per PR.** A PR that adds a feature, fixes a bug, AND refactors unrelated code gets rejected. Split it.
2. **PR description explains the *what* and *why*.** Link to the ticket/issue. Explain design decisions.
3. **Small PRs.** Target < 400 lines changed.
4. **Self-review before requesting review.** Read your own diff. Run the tests.

---

## VIII. Refactoring Triggers

When you see these signals, refactor before continuing:

| Signal | What it means | Action |
|---|---|---|
| Same code in 2+ places | DRY violation | Extract into shared function/module |
| Function > 40 lines | Too much responsibility | Split into smaller functions |
| > 3 levels of nesting | Hard to follow | Guard clauses, extract inner blocks |
| Boolean parameter | Function does two things | Split into two functions |
| `// TODO: fix later` without ticket | Forgotten tech debt | Fix now or create a ticket and reference it |
| Long parameter list (> 5) | Missing abstraction | Group into options struct |
| Match/if-else on type | Missing polymorphism | Use traits or strategy pattern |
| Comment explaining *what* code does | Code isn't self-documenting | Rename, restructure, extract |
| Test is hard to write | Code is too coupled | Refactor the code, not the test |
| Same change touches 5+ files | Feature is scattered | Reorganize by feature/domain |
| God module (> 300 lines) | Too many responsibilities | Split by responsibility |

### Refactoring Rules

1. **Refactor in a separate commit.** Never mix refactoring with feature work in the same commit.
2. **Tests pass before AND after.** Refactoring means changing structure without changing behavior.
3. **Small steps.** Rename → extract → move → inline. One transformation at a time.
4. **Don't refactor code you're not working on.** Unless it's blocking your task.

---

## IX. Quick Reference — When to Apply Which Norm

| Situation | Primary norm |
|---|---|
| "How long can my function be?" | `coding-norm.md` |
| "Where does this file go?" | `architecture-norm.md` |
| "How do I write a test for this?" | `testing-norm.md` |
| "How should I design this function?" | **This file** (clean-code) |
| "How should I handle this error?" | **This file** (clean-code) |
| "Should I add this dependency?" | **This file** (clean-code) |
| "How do I name this variable?" | `coding-norm.md` |
| "How do I structure my commit?" | **This file** (clean-code) |
| "Which layer does this logic go in?" | `architecture-norm.md` |
| "What should this test assert?" | `testing-norm.md` |
| "Should I refactor this?" | **This file** (clean-code) |
| "What's the overall philosophy?" | `CLAUDE.md` |
