# Order Phrase Domain

Word arrangement game data structures.

## Data Model

```rust
OrderPhraseSet {
    id: Uuid,
    user_id: Uuid,
    name: String,
    level: Level,
    language: String,
    subjects: Vec<String>,
    questions: Vec<OrderPhraseQuestion>
}

OrderPhraseQuestion {
    id: Uuid,
    original_phrase: String,
    words: Vec<OrderPhraseWord>,
    hint: Option<String>
}

OrderPhraseWord {
    id: Uuid,
    word: String,
    position: u8
}
```

## Validation Rules

- `name`: required, non-empty
- `level`: "easy" | "medium" | "hard"
- `language`: default "en"
- `subjects`: max 3 items, max 20 chars each
- `original_phrase`: the complete correct sentence
- `words`: each word with its correct position (0-indexed)
- `position`: must be unique within a question, sequential from 0
