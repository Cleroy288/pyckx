# Open Question Domain

AI-graded written answer data structures.

## Data Model

```rust
OpenQuestionSet {
    id: Uuid,
    user_id: Uuid,
    name: String,
    level: Level,
    language: String,
    subjects: Vec<String>,
    questions: Vec<OpenQuestion>
}

OpenQuestion {
    id: Uuid,
    question: String,
    expected_answer: String,
    hint: Option<String>
}
```

## Validation Rules

- `name`: required, non-empty
- `level`: "easy" | "medium" | "hard"
- `language`: default "en"
- `subjects`: max 3 items, max 20 chars each
- `question`: required, non-empty
- `expected_answer`: required, provides reference for AI grading
- `hint`: optional guidance for the user
