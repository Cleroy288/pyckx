# Keywords Domain

Keyword identification game data structures.

## Data Model

```rust
KeywordSet {
    id: Uuid,
    user_id: Uuid,
    name: String,
    level: Level,
    language: String,
    subjects: Vec<String>,
    questions: Vec<KeywordQuestion>
}

KeywordQuestion {
    id: Uuid,
    statement: String,
    keywords: Vec<Keyword>,
    explanation: String
}

Keyword {
    id: Uuid,
    word: String,
    is_correct: bool
}
```

## Validation Rules

- `name`: required, non-empty
- `level`: "easy" | "medium" | "hard"
- `language`: default "en"
- `subjects`: max 3 items, max 20 chars each
- `statement`: required, context for keyword identification
- `keywords`: mix of correct and incorrect options
- `explanation`: explains which keywords are correct and why
