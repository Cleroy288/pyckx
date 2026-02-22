# Testing Norm — Strict Standards for Rust / Leptos Projects

This document defines a **strict** testing standard adapted for Rust. Every test written must follow these rules. Tests are first-class code — they receive the same care as production code.

---

## I. The Testing Pyramid

Follow the pyramid. No exceptions.

```
        /  E2E  \           ~10%  — Critical user journeys only
       /----------\
      / Integration \       ~20%  — Layer boundaries, API contracts
     /----------------\
    /    Unit Tests     \    ~70%  — Business logic, pure functions
   /------------------------\
```

| Level | What it tests | Speed | Dependencies | Runs when |
|---|---|---|---|---|
| **Unit** | Single function/method in isolation | < 10ms each | Zero (all mocked) | Every save / pre-commit |
| **Integration** | Two+ real components together | < 5s each | Real APIs (local) | Every PR / push |
| **E2E** | Full system from user perspective | < 30s each | Full running application | Pre-release / nightly |

---

## II. Test File Placement & Organization

Rust uses a **dual approach** for test placement:

### A. Unit Tests — Inline `#[cfg(test)]` Modules

Unit tests for a module live **inside the same file** as the code they test, in a `#[cfg(test)]` module at the bottom. This is idiomatic Rust.

```rust
// src/domain/validation.rs

/// Validates an email address format
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    // ... implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_email_with_valid_input_returns_ok() {
        let result = validate_email("alice@example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_email_with_missing_at_returns_error() {
        let result = validate_email("aliceexample.com");
        assert!(result.is_err());
    }
}
```

Rules:
- `#[cfg(test)]` module is always the **last item** in the file.
- `use super::*;` to import the parent module's items.
- Test functions are `#[test]` annotated, no `pub`.
- One `#[cfg(test)] mod tests` per file, never multiple.

### B. Integration Tests — `tests/` Directory

Integration tests live in a **dedicated `tests/` directory** at the crate root. Each file is compiled as a separate crate.

```
tests/
├── api_auth_test.rs          ← tests real API auth flow
├── api_intello_test.rs       ← tests real API intello calls
├── helpers/
│   ├── mod.rs                ← shared test utilities
│   ├── factories.rs          ← test data builders
│   └── mocks.rs              ← shared mock implementations
└── fixtures/
    └── sample_data.json      ← static test data
```

### C. E2E Tests

For Leptos apps, E2E tests may use browser automation (e.g., `wasm-bindgen-test`, or external tools like Playwright).

```
tests/
├── e2e/
│   ├── auth_flow_test.rs     ← full login → action → logout
│   └── helpers/
│       └── test_client.rs    ← HTTP client for E2E calls
```

### D. Naming Summary

| Element | Convention | Example |
|---|---|---|
| Unit test module | `#[cfg(test)] mod tests` inline | Bottom of `validation.rs` |
| Integration test file | `{component}_test.rs` in `tests/` | `tests/api_auth_test.rs` |
| E2E test file | `{flow}_test.rs` in `tests/e2e/` | `tests/e2e/auth_flow_test.rs` |
| Test helper | descriptive name in `tests/helpers/` | `tests/helpers/factories.rs` |
| Fixture file | descriptive, matches entity | `tests/fixtures/sample_data.json` |

---

## III. Test Naming Convention

Every test name must answer: **What is being tested, under what conditions, and what is the expected outcome?**

### The Formula

```
{action}_{scenario}_{expected_result}
```

### Examples

```rust
#[test]
fn validate_email_with_valid_input_returns_ok() { ... }

#[test]
fn validate_email_with_empty_string_returns_error() { ... }

#[test]
fn create_user_with_duplicate_email_returns_conflict() { ... }

#[test]
fn parse_score_with_negative_value_returns_none() { ... }
```

### Naming Rules

1. **Describe behavior, not implementation.** Test what the code does, not how.
2. **Never use generic names.** No `test1`, `test_success`, `test_error`, `it_works`.
3. **No filler words.** Avoid `correctly`, `properly`, `successfully` — describe what correct *means*.
4. **Long names are fine.** A 60-character test name that is clear beats a 20-character name that is cryptic.
5. **Use the language of the domain.** Say "returns insufficient funds error" not "returns error code 402".

---

## IV. Test Structure — AAA Pattern

Every test follows **Arrange → Act → Assert**. Three blocks, clearly separated. No exceptions.

```rust
#[test]
fn transfer_money_insufficient_funds_returns_error() {
    // Arrange
    let account = Account::new(50.0);
    let mut service = TransferService::new(
        MockAccountRepo::with_account(account),
    );

    // Act
    let result = service.transfer("acc-1", "acc-2", 100.0);

    // Assert
    assert!(matches!(
        result,
        Err(TransferError::InsufficientFunds)
    ));
}
```

### AAA Rules

| Rule | Details |
|---|---|
| **One Act per test** | A test calls exactly **one** function/method. Never two. |
| **Arrange is setup only** | Create objects, configure mocks, prepare input. No assertions here. |
| **Assert is verification only** | Check results. No further calls to the system under test. |
| **Separate with comments or blank lines** | Visually distinguish the three sections. Use `// Arrange`, `// Act`, `// Assert` comments. |
| **Keep Arrange minimal** | If Arrange exceeds 10 lines, extract a helper or factory. |
| **Prefer specific assertions** | `assert_eq!(user.email, "alice@mail.com")` not `assert!(user.email.is_some())` |
| **One logical assertion per test** | You may have multiple `assert` calls, but they must verify **one behavior**. |

---

## V. Table-Driven Tests

When a function has multiple input/output scenarios, use **parameterized tests**. This is mandatory — not optional.

```rust
#[test]
fn validate_email_scenarios() {
    let cases = vec![
        ("valid email", "alice@example.com", true),
        ("missing @ symbol", "aliceexample.com", false),
        ("empty string", "", false),
        ("missing domain", "alice@", false),
        ("valid with subdomain", "a@sub.example.com", true),
    ];

    for (name, input, expected_valid) in cases {
        let result = validate_email(input);
        assert_eq!(
            result.is_ok(),
            expected_valid,
            "Case '{}': validate_email({:?})",
            name,
            input
        );
    }
}
```

### Table-Driven Rules

1. **Every entry needs a name/description.** The name describes the scenario, not the input.
2. **Table holds data, loop holds logic.** No `if` statements inside the table.
3. **Include descriptive assertion messages** with the case name for debugging.
4. **Cover edge cases explicitly.** Every table must include: a happy path, a boundary, an empty input, and at least one error case.
5. **Order entries logically.** Happy path first, then edge cases, then error cases.

---

## VI. Mocking Rules

### When to Mock

| Mock | Don't mock |
|---|---|
| HTTP clients, external APIs | The system under test itself |
| File system, network calls | Pure functions with no dependencies |
| Time/clock, random generators | Simple structs or enums |
| Slow or non-deterministic deps | Standard library functions |

### Mocking in Rust

1. **Use traits for mockable boundaries.** Define a trait for the behavior, implement it for production and for tests.
2. **Use `mockall` crate** for auto-generated mocks when trait-based mocking is needed.
3. **Prefer hand-written fakes** for simple cases. A `FakeUserRepo` backed by a `HashMap` is often clearer than a mock with 10 expectations.
4. **Verify interactions sparingly.** Assert on the *result*, not on which internal methods were called.
5. **One mock per dependency.** If a service has 3 dependencies, create 3 separate mocks.

### Mock Naming

| Element | Convention |
|---|---|
| Mock struct | `Mock{Trait}` |
| Fake (simplified impl) | `Fake{Trait}` or `Test{Trait}` |
| Stub (returns canned data) | `Stub{Trait}` |

---

## VII. What Each Test Level Covers

### A. Unit Tests (`#[cfg(test)]`)

Test one function/method in **complete isolation**. All dependencies are mocked via traits.

**Must test:**
- Happy path (valid input → expected output)
- Edge cases (empty `""`, `None`, `0`, boundary values, max length)
- Error paths (invalid input → correct error variant)
- Business rules (domain constraints, validation logic)

**Must NOT:**
- Touch the network or file system
- Depend on environment variables or config files
- Depend on test execution order

**Mandatory coverage per input type:**

| Input type | Required test cases |
|---|---|
| `&str` parameter | Empty `""`, whitespace `" "`, valid, too long |
| Numeric parameter | Zero, negative, positive, boundary (min/max) |
| `bool` parameter | `true`, `false` |
| `Option<T>` | `None`, `Some(valid)` |
| `Vec<T>` / slice | Empty `[]`, single element, multiple, duplicates |
| Struct | All required fields, missing fields, invalid values |

### B. Integration Tests (`tests/`)

Test two or more **real** components together.

**Must test:**
- API client functions against sandbox/mock server
- Data transformation pipelines
- Multi-step workflows

**Must:**
- Be idempotent (running twice produces the same result)
- Use factories for test data (never hardcoded IDs)
- Not depend on external services being available

**Must NOT:**
- Test pure business logic (that's unit tests' job)
- Share state between test functions

### C. E2E Tests

Test **critical user journeys** through the full running application.

**Must test:**
- Authentication flow (login → access protected page → logout)
- Core business flow (create → process → complete)
- Permission boundaries

**Must NOT:**
- Test every edge case (that's unit tests' job)
- Be flaky (if a test fails intermittently, fix it or delete it)

---

## VIII. Test Data Management

### A. Factories / Builders

Never hardcode test data inline. Use **factory functions**.

```rust
// tests/helpers/factories.rs

pub fn test_user() -> User {
    User {
        id: Uuid::new_v4(),
        name: "Alice Test".to_string(),
        email: "alice@test.com".to_string(),
    }
}

pub fn test_user_with_email(email: &str) -> User {
    User {
        email: email.to_string(),
        ..test_user()
    }
}
```

### B. Rules

1. **Test data is deterministic.** No random values unless testing randomness.
2. **Test data is isolated.** Each test creates its own data.
3. **Test data is minimal.** Create only the fields needed for the specific test.
4. **No production data in tests.** Never copy real user data or PII.

---

## IX. Test Isolation Rules

1. **Tests run in any order.** No test depends on another test running first.
2. **Tests run in parallel.** Rust runs tests in parallel by default — design for it.
3. **Each test has its own setup.** No shared mutable state between tests.
4. **No `std::thread::sleep` or polling.** Use synchronization primitives or channels.
5. **No network calls in unit tests.** If a test makes a real HTTP request, it's an integration test.

---

## X. Assertion Rules

### Do

- **Assert on specific values.** `assert_eq!(result, 42)` not `assert!(result > 0)`
- **Assert on error variants.** `assert!(matches!(err, AppError::NotFound))` not `assert!(result.is_err())`
- **Assert on structure.** Verify the returned struct has the expected fields.
- **Use custom assertion messages.** `assert_eq!(x, y, "discount should be 10% for premium users")`

### Don't

- **Don't assert implementation details.** Check the public output.
- **Don't use `assert!(true)` or trivial assertions.**
- **Don't assert on more than one behavior per test.** Split into multiple tests.
- **Don't use hardcoded magic values in assertions.** Define expected values as named constants.

---

## XI. Forbidden Patterns

| Pattern | Why it's forbidden | Fix |
|---|---|---|
| **Testing private functions directly** | Couples tests to implementation | Test through the public interface |
| **Test interdependence** | Test A sets up data that Test B needs | Each test arranges its own state |
| **Sleeping for timing** | `thread::sleep` makes tests slow and flaky | Use channels or `tokio::time::pause` |
| **Catch-all assertions** | `assert!(result.is_some())` tells nothing on failure | Assert on specific fields and values |
| **Mocking everything** | If every dependency is mocked, the test proves nothing | Mock boundaries only |
| **Copy-paste tests** | 10 identical tests with one line different | Use table-driven tests |
| **Commented-out tests** | Dead tests rot | Delete them. Git remembers |
| **`#[ignore]` without reason** | Hides broken tests | Add a reason and ticket reference |
| **`println!` in tests** | Debug output left behind | Remove all debug output |
| **Testing the standard library** | Asserting that `serde` works | Only test YOUR code |
| **Flaky tests** | Tests that fail randomly | Fix the root cause or delete |
| **Huge test functions** | 100+ line test | Split into focused tests. Max 40 lines |

---

## XII. Test Code Quality

Tests are production code. They follow the same standards:

1. **Max 40 lines per test body** (same as coding norm). If longer, extract helpers.
2. **No dead code in tests.** No commented-out assertions, no unused variables.
3. **DRY applies to tests.** Shared setup goes in helpers/factories. But: clarity > DRY.
4. **Test files follow the same formatting rules** as production code (80 columns, consistent spacing).
5. **No conditional logic in tests.** A test that has `if/else` is testing two things. Split it.
6. **Test helpers don't assert.** Helpers return values. The test function does the assertions.

---

## XIII. Coverage Expectations

| Test level | Target | Notes |
|---|---|---|
| **Unit** | >= 80% line coverage on services and domain | 100% on critical business logic |
| **Integration** | Every API client has at least one test | Every server function has at least one test |
| **E2E** | Every critical user journey is covered | Auth, core flow, permissions |

### Coverage Rules

1. **Coverage is a floor, not a ceiling.** 80% is the minimum, not the goal.
2. **Coverage without assertions is worthless.** Calling a function without checking its result doesn't count.
3. **Don't game coverage.** Writing tests that execute code but verify nothing is worse than no tests.
4. **Uncovered code must be justified.** Use `cargo tarpaulin` for coverage reports.

---

## XIV. CI/CD Integration

| Level | When it runs | Max duration | Failure = |
|---|---|---|---|
| **Unit** | Every commit / pre-push hook | < 30 seconds total | Block merge |
| **Integration** | Every PR / push to main | < 5 minutes total | Block merge |
| **E2E** | Pre-release / nightly | < 15 minutes total | Block release |

### Rules

1. **All tests must pass before merge.** No exceptions.
2. **Flaky tests are bugs.** Fix them within the sprint.
3. **Tests are part of the definition of done.** A feature without tests is not done.
4. **New code requires new tests.** Every PR that adds behavior must include tests.

---

## XV. Self-Check Before Submitting Tests

- [ ] Every test has a descriptive name following `{action}_{scenario}_{expected}` convention
- [ ] Every test follows AAA: Arrange → Act → Assert, clearly separated
- [ ] No test exceeds 40 lines
- [ ] No test depends on another test's state or execution order
- [ ] No `thread::sleep`, no polling, no flaky timing dependencies
- [ ] All mocks are at trait boundaries (not mocking internals)
- [ ] Table-driven tests are used for functions with 3+ scenarios
- [ ] Edge cases covered: empty, None, zero, boundary, error paths
- [ ] No `println!` / `dbg!` debug output left behind
- [ ] No commented-out tests or `#[ignore]` without ticket references
- [ ] Test data uses factories, not hardcoded inline values
- [ ] Assertions are specific (not just `is_some()` or `is_ok()`)
