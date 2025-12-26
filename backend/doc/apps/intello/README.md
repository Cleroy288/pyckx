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
| **Courses** | Organize resources + AI generation | `POST /courses` | `GET /courses` |

Base URL: `/api/intello`

---

## QCM (Multiple Choice)

### Data Model
```rust
QcmSet { id, user_id, name, description, level, language, subjects, questions: Vec<QcmQuestion> }
QcmQuestion { id, question, wrong_answers: Vec<String>, right_answer: String, explanation: String }
```

### Endpoints
- `POST /qcm` - **Manual creation** from JSON (user types questions)
- `POST /custom-question` - **AI generation** from documents (multipart)
- `GET /qcm` - List all user's sets (manual + AI-generated)
- `GET /qcm/{id}` - Get specific set
- `PUT /qcm/{id}` - Update set
- `DELETE /qcm/{id}` - Delete set

### Manual QCM Validation
- Name: required, non-empty
- Description: required, non-empty
- Level: "easy" | "medium" | "hard"
- Language: default "en"
- Subjects: max 3, max 20 chars each
- Questions: each must have unique answers (case-insensitive)

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

## Courses & Resources

The course system allows users to organize resources and generate AI-powered courses.

### Data Model
```rust
Course { id, user_id, name, description, created_at, updated_at }
CourseResource { id, course_id, filename, content, token_count, created_at }
```

### ✅ Implemented Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/courses` | `POST` | Create a new course |
| `/courses` | `GET` | List user's courses |
| `/courses/{id}/resources` | `POST` | Upload resource to course |
| `/courses/{id}/resources` | `GET` | Get course resources |
| `/courses/{id}/sessions` | `POST` | Create study session |
| `/courses/{id}/sessions` | `GET` | List course sessions |
| `/generate-course` | `POST` | AI-generate course content |

### Course CRUD

**Create Course:**
```bash
POST /api/intello/courses
{
  "name": "Mathematics",
  "description": "Algebra and calculus"
}
```

**List Courses:**
```bash
GET /api/intello/courses
# Returns: { "success": true, "courses": [...] }
```

**Upload Resource:**
```bash
POST /api/intello/courses/{id}/resources
{
  "filename": "notes.pdf",
  "content": "...",
  "token_count": 1500
}
```

**Get Resources:**
```bash
GET /api/intello/courses/{id}/resources
# Returns: { "success": true, "resources": [...] }
```

### Session CRUD

**Create Session:**
```bash
POST /api/intello/courses/{id}/sessions
{
  "topic": "Introduction to Calculus",
  "instructions": "Focus on derivatives",
  "keywords": ["derivative", "limit"],
  "language": "en"
}
```

**List Sessions:**
```bash
GET /api/intello/courses/{id}/sessions
# Returns: { "success": true, "sessions": [...] }
```

### AI Course Generation

**Generate Course:**
```bash
POST /api/intello/generate-course
{
  "topic": "Rust Programming",
  "keywords": ["ownership", "borrowing"],
  "instructions": "Beginner level",
  "resource_ids": ["uuid-1", "uuid-2"]  // Fetches from Supabase
}
```

**Three-Stage Pipeline:**
The backend uses a sophisticated 3-stage pipeline to generate high-quality courses:

1.  **Stage 0: Decryption & Knowledge Expansion**
    *   **Intent Decryption**: Analyzing the user's prompt to understand the core intent, target audience, and pedagogical goals.
    *   **Knowledge Expansion (Ensemble)**: Querying multiple AI models in parallel to build a comprehensive knowledge base about the topic, supplementing provided documents.

2.  **Stage 1: Core Knowledge Extraction**
    *   Synthesizing the expanded knowledge and user documents into a coherent "Core Knowledge" summary.
    *   Defining key concepts and definitions to ensure accuracy.

3.  **Stage 2: Structure Generation**
    *   **Dynamic Prompt Building**: Using `prompt_builder.rs` as the Single Source of Truth to inject strict JSON schemas for embedded games (QCM, Flashcards, etc.).
    *   **Course Construction**: Generating the final `GeneratedCourse` structure, including modules, text content, Mermaid diagrams, and verification exercises.

**Output:**
Returns a `GeneratedCourse` object containing:
-   `course_metadata`: Title, description, level.
-   `modules`: Ordered list of modules, each containing content blocks (text, schemas, games).

**Implementation:**
-   `src/api/handlers/intello/course_generation.rs` - AI generation handler
-   `src/services/openrouter/simple_course_generation.rs` - Implementation of the 3-stage pipeline
-   `src/shared/prompt_builder.rs` - SSOT for game schemas and prompts

### ❌ Not Yet Implemented

| Feature | Status |
|---------|--------|
| Session Content Storage | Deferred |
| Session Games | Deferred |
| Session Q&A | Deferred |
| Session Synthesis | Deferred |

These require additional repository integration. See `backend/doc/db_shema/course.txt` for the full schema.

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
