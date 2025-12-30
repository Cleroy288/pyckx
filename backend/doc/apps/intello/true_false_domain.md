# True/False Domain

Statement verification data structures.

## Data Model

```rust
TrueOrFalseSet {
    id: Uuid,
    user_id: Uuid,
    name: String,
    level: Level,
    language: String,
    subjects: Vec<String>,
    statements: Vec<TrueOrFalseStatement>
}

TrueOrFalseStatement {
    id: Uuid,
    statement: String,
    answer: bool,
    explanation: String
}
```

## Validation Rules

- `name`: required, non-empty
- `level`: "easy" | "medium" | "hard"
- `language`: default "en"
- `subjects`: max 3 items, max 20 chars each
- `statement`: required, the claim to verify
- `answer`: boolean indicating truth value
- `explanation`: required, explains why the statement is true or false
