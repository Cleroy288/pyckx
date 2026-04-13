# Fill Blank Domain

Fill-in-the-blank game data structures.

## Data Model

```rust
FillBlankSet {
    id: Uuid,
    user_id: Uuid,
    name: String,
    level: Level,
    language: String,
    subjects: Vec<String>,
    questions: Vec<FillBlankQuestion>
}

FillBlankQuestion {
    id: Uuid,
    phrase: String,
    options: Vec<FillBlankOption>,
    explanation: String
}

FillBlankOption {
    id: Uuid,
    text: String,
    is_correct: bool
}
```

## Validation Rules

- `name`: required, non-empty
- `level`: "easy" | "medium" | "hard"
- `language`: default "en"
- `subjects`: max 3 items, max 20 chars each
- `phrase`: contains `___` marker for the blank
- `options`: mix of correct and incorrect choices
- Exactly one option should have `is_correct: true`
- `explanation`: explains the correct answer
