# Database Schema

Supabase PostgreSQL tables. Raw SQL files in `../db_shema/`.

## Users & Auth

### `users`
Supabase auth users (managed by Supabase).

### `user_profiles`
Extended user data.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | FK → auth.users |
| email | TEXT | |
| name | TEXT | |
| created_at | TIMESTAMP | |

---

## Apps

### `apps`
Available apps on the platform.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| name | TEXT | Unique |
| description | TEXT | |

### `user_apps`
Apps enabled per user.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| user_id | UUID | FK → users |
| app_name | TEXT | |

---

## Collection

### `user_collections`
Collection metadata per user.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| user_id | UUID | FK → users |
| type | TEXT | e.g., "dvd" |

### `dvds`
DVD items.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| user_id | UUID | FK → users |
| name | TEXT | Unique per user |
| description | TEXT | |
| release_date | DATE | |
| actors | TEXT | Comma-separated |
| directors | TEXT | |
| genre | TEXT | |
| created_at | TIMESTAMP | |
| updated_at | TIMESTAMP | |

---

## Intello - Courses & Resources

### `intello_courses`
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| user_id | UUID | FK → users |
| name | VARCHAR(255) | |
| description | TEXT | |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `intello_user_resources`
User-owned resources (single source of truth).

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| user_id | UUID | FK → users |
| filename | TEXT | Unique per user |
| content | TEXT | Full document content |
| token_count | INTEGER | For AI context limits |
| created_at | TIMESTAMPTZ | |

### `intello_course_resource_links`
Junction table linking resources to courses (many-to-many).

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| course_id | UUID | FK → intello_courses |
| resource_id | UUID | FK → intello_user_resources |
| created_at | TIMESTAMPTZ | |

### `intello_study_sessions`
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| course_id | UUID | FK → intello_courses |
| topic | VARCHAR(255) | |
| instructions | TEXT | |
| keywords | TEXT[] | |
| language | VARCHAR(5) | en, fr, es, de, nl |
| status | VARCHAR(20) | in_progress, completed |
| extracted_knowledge | JSONB | Structure concepts (Stage 1) |
| educational_content | JSONB | Detailed content (Stage 1.5) |
| expanded_knowledge | TEXT | AI expansion (Stage 0.5) |
| created_at | TIMESTAMPTZ | |
| completed_at | TIMESTAMPTZ | |

### `intello_session_resources`
Links resources used in a specific session.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| session_id | UUID | FK → intello_study_sessions |
| resource_id | UUID | FK → intello_user_resources |

---

## Intello - QCM

### `qcm_sets`
| Column | Type |
|--------|------|
| id | UUID |
| user_id | UUID |
| name | TEXT |
| description | TEXT |
| level | TEXT |

### `qcm_questions`
| Column | Type |
|--------|------|
| id | UUID |
| set_id | UUID |
| question | TEXT |
| choices | JSONB |
| correct_answer | INT |

---

## Intello - Open Questions

### `open_question_sets`
| Column | Type |
|--------|------|
| id | UUID |
| user_id | UUID |
| name | TEXT |
| level | TEXT |
| language | TEXT |
| subjects | JSONB |

### `open_questions`
| Column | Type |
|--------|------|
| id | UUID |
| set_id | UUID |
| question | TEXT |
| expected_answer | TEXT |
| hint | TEXT |

---

## Intello - Flashcards

### `flashcard_sets`
| Column | Type |
|--------|------|
| id | UUID |
| user_id | UUID |
| name | TEXT |
| level | TEXT |

### `flashcards`
| Column | Type |
|--------|------|
| id | UUID |
| set_id | UUID |
| front | TEXT |
| back | TEXT |

---

## Intello - True/False

### `intello_true_false_sets`
| Column | Type |
|--------|------|
| id | UUID |
| user_id | UUID |
| name | TEXT |
| level | TEXT |

### `intello_true_false_statements`
| Column | Type |
|--------|------|
| id | UUID |
| set_id | UUID |
| statement | TEXT |
| answer | BOOLEAN |
| explanation | TEXT |

---

## Intello - Keywords

### `keywords_sets`
| Column | Type |
|--------|------|
| id | UUID |
| user_id | UUID |
| name | TEXT |
| level | TEXT |

### `keywords_questions`
| Column | Type |
|--------|------|
| id | UUID |
| set_id | UUID |
| statement | TEXT |
| explanation | TEXT |

### `keywords`
| Column | Type |
|--------|------|
| id | UUID |
| question_id | UUID |
| word | TEXT |
| is_correct | BOOLEAN |

---

## Intello - AI Usage Tracking

### `intello_ai_usage_log`
Tracks token usage and costs for all AI requests.

| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| user_id | UUID | FK → users |
| model_id | TEXT | Model used (e.g., gemini-3-flash-preview) |
| feature_type | TEXT | course_generation, qcm, flashcard, etc. |
| input_tokens | INTEGER | Tokens sent to model |
| output_tokens | INTEGER | Tokens received |
| input_cost_usd | DECIMAL | Cost for input tokens |
| output_cost_usd | DECIMAL | Cost for output tokens |
| total_cost_usd | DECIMAL | Total cost |
| created_at | TIMESTAMPTZ | |

---

## SQL Files

See [db_shema/](../db_shema/) for full CREATE TABLE statements:
- `user_profiles.txt`
- `apps.txt`, `user_apps.txt`
- `user_collections.txt`, `dvd.txt`
- `qcm_flashcard_openquestion.txt`
- `true_false.txt`
- `keywords.txt`
- `course.txt`
- `ai_usage.txt`
