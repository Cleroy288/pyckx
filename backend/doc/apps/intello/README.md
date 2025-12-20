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
| **Fill Blank** | Fill in missing words | `POST /fill-blank/create` | `GET /fill-blank/list` |

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

## Fill Blank

### Data Model
```rust
FillBlankSet { id, user_id, name, level, language, subjects, questions }
FillBlankQuestion { id, phrase, options: Vec<FillBlankOption>, explanation }
FillBlankOption { id, text, is_correct: bool }
```

### Endpoints
- `POST /fill-blank/create` - Generate from documents
- `GET /fill-blank/list` - List user's sets

### Game Logic
AI generates phrases with blanks (marked as `___`). Users select the correct word from multiple options to fill in the blank.

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
  "model": "google/gemini-2.0-flash-exp:free"
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

---

## AI Response Handling

### Duplicate Key Sanitization

AI models may occasionally generate invalid JSON with duplicate keys:
```json
{
  "expected_answer": "...",
  "expected_answer": "..."  // Invalid duplicate
}
```

The backend automatically sanitizes AI responses before parsing:
1. **Prompt Prevention**: Prompts include explicit JSON validation rules
2. **Defensive Parsing**: `sanitize_json_duplicates()` removes duplicates, keeping first occurrence
3. **Warning Logs**: Duplicates are logged for monitoring

**Files:**
- `src/services/openrouter/utils.rs` - Sanitization utility
- `src/shared/prompt_builder.rs` - JSON validation rules in prompts

---

See [AI Models](ai-models.md) for available models.
