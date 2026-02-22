# Rust Test Norm

These rules are NOT enforced by clippy. They are enforced by discipline.
Copy this file into each `exNN/` project as a reference.

## Rules

### 1. One unit test per public function (mandatory)
Every public function MUST have at least one corresponding test.

### 2. Test module name: `mod tests` (mandatory)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // ...
}
```

### 3. Always wrap with `#[cfg(test)]` (mandatory)
Prevents test code from being compiled into release builds.

### 4. Test function prefix: `test_` (mandatory)
```rust
#[test]
fn test_parse_empty_input() {}  // good
#[test]
fn it_works() {}                // bad
#[test]
fn test1() {}                   // bad
```

### 5. Test names describe behavior (mandatory)
Format: `test_<function_name>_<scenario>`
```rust
#[test]
fn test_parse_code_with_empty_string() {}
#[test]
fn test_parse_code_with_valid_rust() {}
#[test]
fn test_read_file_nonexistent_path() {}
```

### 6. One assert per test (preferred)
Keep tests focused. One test = one behavior.
```rust
// good: two separate tests
#[test]
fn test_add_positive() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add_negative() {
    assert_eq!(add(-1, -2), -3);
}

// bad: multiple behaviors in one test
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, -2), -3);
    assert_eq!(add(0, 0), 0);
}
```

### 7. Location (mandatory)
- Unit tests: in the same file as the code, inside `mod tests`
- Integration tests: in `tests/` directory

### 8. No `dbg!` or `println!` in tests (enforced by clippy)
Use `assert_*` macros. If a test fails, the assert message tells you why.

### 9. `unwrap()` and `expect()` are allowed in tests (enforced by clippy)
Tests should panic on unexpected errors -- that is what unwrap does.

## Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_<function_name>_<scenario>() {
        // arrange
        let input = ...;

        // act
        let result = function_name(input);

        // assert
        assert_eq!(result, expected);
    }
}
```

## Checklist before submitting
- [ ] Every public function has at least one test
- [ ] All tests are in `#[cfg(test)] mod tests`
- [ ] All test names start with `test_` and describe behavior
- [ ] One assert per test (preferred)
- [ ] No `dbg!` or `println!` in tests
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
