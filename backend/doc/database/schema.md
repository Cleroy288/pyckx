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

## SQL Files

See [db_shema/](../db_shema/) for full CREATE TABLE statements:
- `user_profiles.txt`
- `apps.txt`, `user_apps.txt`
- `user_collections.txt`, `dvd.txt`
- `qcm_flashcard_openquestion.txt`
- `true_false.txt`
- `keywords.txt`
