# Flashcard Domain

Front/back study card data structures.

## Data Model

```rust
FlashcardSet {
    id: Uuid,
    user_id: Uuid,
    name: String,
    level: Level,
    language: String,
    subjects: Vec<String>,
    cards: Vec<Flashcard>
}

Flashcard {
    id: Uuid,
    front: String,
    back: String
}
```

## Validation Rules

- `name`: required, non-empty
- `level`: "easy" | "medium" | "hard"
- `language`: default "en"
- `subjects`: max 3 items, max 20 chars each
- `front`: required, the question/term side
- `back`: required, the answer/definition side
