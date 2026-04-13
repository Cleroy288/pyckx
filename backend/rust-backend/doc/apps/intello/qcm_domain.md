# QCM Domain

Multiple choice questions data structures and validation rules.

## Data Model

```rust
QcmSet {
    id: Uuid,
    user_id: Uuid,
    name: String,
    description: String,
    level: Level,
    language: String,
    subjects: Vec<String>,
    questions: Vec<QcmQuestion>
}

QcmQuestion {
    id: Uuid,
    question: String,
    wrong_answers: Vec<String>,
    right_answer: String,
    explanation: String
}
```

## Validation Rules

### QcmSet Validation
- `name`: required, non-empty
- `description`: required, non-empty
- `level`: "easy" | "medium" | "hard"
- `language`: default "en"
- `subjects`: max 3 items, max 20 chars each

### QcmQuestion Invariants
- All answers must be unique (case-insensitive comparison)
- `question` field must be non-empty
- `right_answer` must be non-empty
- `wrong_answers` must contain at least one item
- `explanation` should provide context for the correct answer
