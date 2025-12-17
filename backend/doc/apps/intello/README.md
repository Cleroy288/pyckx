# Intello App

AI-powered learning games generated from your documents.

## Available Games

| Game | Description | Create | List |
|------|-------------|--------|------|
| **QCM** | Multiple choice questions | `POST /qcm` | `GET /qcm` |
| **Open Questions** | AI-graded written answers | `POST /open-question/create` | `GET /open-question/list` |
| **Flashcards** | Front/back study cards | `POST /flashcard/create` | `GET /flashcard/list` |
| **True/False** | Statement verification | `POST /true-false/create` | `GET /true-false/list` |
| **Keywords** | Keyword identification | `POST /keywords/create` | `GET /keywords/list` |
| **Order Phrase** | Arrange shuffled words | `POST /order-phrase/create` | `GET /order-phrase/list` |

Base URL: `/app/intello`

---

## QCM (Multiple Choice)

### Data Model
```rust
QcmSet { id, user_id, name, description, level, questions: Vec<QcmQuestion> }
QcmQuestion { id, question, choices: Vec<String>, correct_answer: usize }
```

### Endpoints
- `POST /qcm` - Create from JSON
- `POST /custom-question` - Generate from documents (multipart)
- `GET /qcm` - List user's sets

---

## Open Questions

### Data Model
```rust
OpenQuestionSet { id, user_id, name, level, language, subjects, questions }
OpenQuestion { id, question, expected_answer, hint }
```

### Endpoints
- `POST /open-question/create` - Generate from documents
- `GET /open-question/list` - List user's sets
- `POST /open-question/check` - Grade answers with AI

### Grading
| Grade | Meaning |
|-------|---------|
| `right` | Correct answer |
| `medium` | Partially correct |
| `error` | Incorrect |

---

## Flashcards

### Data Model
```rust
FlashcardSet { id, user_id, name, level, language, subjects, cards }
Flashcard { id, front, back }
```

### Endpoints
- `POST /flashcard/create` - Generate from documents
- `GET /flashcard/list` - List user's sets

---

## True/False

### Data Model
```rust
TrueOrFalseSet { id, user_id, name, level, language, subjects, statements }
TrueOrFalseStatement { id, statement, answer: bool, explanation }
```

### Endpoints
- `POST /true-false/create` - Generate from documents
- `GET /true-false/list` - List user's sets

---

## Keywords

### Data Model
```rust
KeywordSet { id, user_id, name, level, language, subjects, questions }
KeywordQuestion { id, statement, keywords: Vec<Keyword>, explanation }
Keyword { id, word, is_correct: bool }
```

### Endpoints
- `POST /keywords/create` - Generate from documents
- `GET /keywords/list` - List user's sets

---

## Order Phrase

### Data Model
```rust
OrderPhraseSet { id, user_id, name, level, language, subjects, questions }
OrderPhraseQuestion { id, original_phrase, words: Vec<OrderPhraseWord>, hint }
OrderPhraseWord { id, word, position: u8 }
```

### Endpoints
- `POST /order-phrase/create` - Generate from documents
- `GET /order-phrase/list` - List user's sets

### Game Logic
Users arrange shuffled words into the correct order. Each word has a `position` field indicating its correct placement (0-indexed).

---

## Creating Content (All Games)

All AI-generation endpoints accept **multipart form data**:

**Fields:**
- `metadata` (JSON): Configuration
- `files`: Document files (PDF, DOCX, PPTX, TXT)

**Metadata Schema:**
```json
{
  "name": "Quiz Name",
  "description": "Optional description",
  "instructions": "AI instructions",
  "language": "en",
  "level": "easy|medium|hard",
  "subjects": ["Subject1", "Subject2"],
  "num_questions": 10,
  "model": "amazon/nova-2-lite-v1:free"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Created with N questions",
  "id": "uuid",
  "total_token_count": 5000,
  "documents_processed": 2,
  "questions": [...]
}
```

---

## File Structure

```
src/
├── domain/intello/           # Entities for all games
├── services/intello/         # Business logic (*_ops.rs)
├── services/openrouter/      # AI generation per game type
├── api/dto/intello/          # DTOs per game
└── api/handlers/intello/     # HTTP handlers
```

See [AI Models](ai-models.md) for available models.
