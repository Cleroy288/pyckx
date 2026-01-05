# Architecture Overview

## Layered Architecture

```
┌──────────────────────────────────────────────────┐
│  HTTP Layer (api/handlers/)                       │
│  - Extract session, parse DTOs, call use case     │
└──────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────┐
│  Use Cases Layer (use_cases/)                     │
│  - Orchestration, validation, coordination        │
│  - See: doc/architecture/use-cases.md             │
└──────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────┐
│  Services Layer (services/)                       │
│  - Business rules, domain operations              │
└──────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────┐
│  Domain Layer (domain/)                           │
│  - Pure data structures, no I/O                   │
└──────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────┐
│  Infrastructure (infrastructure/)                 │
│  - Repository traits → Supabase/JSON impls        │
└──────────────────────────────────────────────────┘
```

## Request Flow

```
Client Request
      │
      ▼
┌─────────────┐
│   Handler   │  1. Extract session
│             │  2. Parse DTO
│             │  3. Build use case input
└─────────────┘
      │
      ▼
┌─────────────┐
│  Use Case   │  4. Validate input
│             │  5. Coordinate services
└─────────────┘
      │
      ▼
┌─────────────┐
│   Service   │  6. Apply business rules
│             │  7. Call repository
└─────────────┘
      │
      ▼
┌─────────────┐
│ Repository  │  8. Execute I/O (Supabase/JSON)
└─────────────┘
      │
      ▼
Response flows back up
```

## Folder Structure

```
src/
├── http_api/                 # HTTP layer
│   ├── handlers/             # Route handlers by domain
│   │   ├── auth/
│   │   ├── apps/
│   │   ├── collection/
│   │   └── intello/
│   ├── data_transfer_object/ # Request/Response types
│   ├── middlewares/          # Rate limiting, etc.
│   └── utils/                # Validation & error utilities
│       ├── validation/        # ValidationError, validate_request()
│       └── internal/          # InternalError
├── services/                 # Business logic layer
├── services/                 # Business logic & Domain layer
│   ├── app_registry/         # App management & generic domain
│   │   ├── domain/           # App, AppInstance entities
│   │   ├── app_error.rs      # Self-contained App errors
│   │   └── service.rs        # App registry operations
│   ├── intello/              # Intello service
│   │   ├── domain/           # Domain entities (QcmSet, Flashcard, etc.)
│   │   │   ├── intello_ids.rs
│   │   │   └── intello_error.rs
│   │   ├── qcm_ops.rs        # Game-specific operations
│   │   └── ...
│   ├── collection/           # Collection service
│   │   ├── domain/           # Domain entities (Collection, DVD)
│   │   │   └── app.rs        # CollectionApp entity
│   │   └── collection_error.rs
│   ├── auth/                 # Auth service & errors
│   │   └── auth_error.rs
│   └── openrouter/           # AI API calls
├── infra/                    # Infrastructure layer
│   ├── supabase/             # Supabase client & repositories
│   │   └── error/            # Self-contained Supabase errors
│   │       └── supabase_error.rs
│   ├── user/                 # User entity, UserId, session utils
│   ├── session/              # Session management (Supabase persistence)
│   │   ├── service.rs        # SessionStore (in-memory + async Supabase sync)
│   │   └── session_error.rs  # Self-contained Session errors
│   └── database/             # Repository trait definitions
├── shared/                   # Cross-cutting utilities
│   ├── constants/            # Global error aggregator, URLs
│   │   ├── errors/
│   │   │   └── global.rs     # AppError (wraps all service errors), AppResult
│   │   └── urls/
│   │       └── supabase.rs   # Supabase API paths
│   └── utils/                # Generic utilities (deserializers)
├── configs/                  # Environment configuration
└── tests/                    # Integration tests
```


## Key Principles

1. **Handlers are thin** - Only HTTP concerns (parse, delegate, respond)
2. **Services own business logic** - Validation, rules, orchestration
3. **Pipelining** - Heavy AI workflows use multi-stage pipelines (e.g., Course Generation)
4. **Repositories are traits** - Enables swappable backends, testing
5. **Domain is pure** - No I/O, no dependencies
6. **Errors are decentralized** - Each service owns error codes/messages/statuses, wraps via `AppError`

## Error System

The error system follows a **decentralized architecture** where each service owns its error definitions:

### Service-Level Errors (Self-Contained)

Each error type implements `code()`, `message()`, and `status()` methods:

| Error Type | Location | Errors |
|------------|----------|--------|
| `AuthError` | `services/auth/auth_error.rs` | InvalidCredentials, NotAllowed, External |
| `IntelloError` | `services/intello/domain/intello_error.rs` | GameSetNotFound, ValidationFailed, StorageError, etc. |
| `AppsError` | `services/app_registry/app_error.rs` | NotFound, AlreadyExists, UserAppAlreadyAdded, UserAppNotFound |
| `CollectionError` | `services/collection/collection_error.rs` | DvdNotFound, DvdDuplicate, UserNotFound, StorageError |
| `SupabaseError` | `infra/supabase/error/supabase_error.rs` | Http, Network, Parse, Timeout |
| `SessionError` | `infra/session/session_error.rs` | NotFound, Expired |
| `ValidationError` | `http_api/utils/validation/error.rs` | Generic input validation |
| `InternalError` | `http_api/utils/internal/error.rs` | Generic internal errors |

### Global Error Aggregator

`shared/constants/errors/global.rs` contains `AppError` which wraps all service errors:

```rust
pub enum AppError {
    Auth(AuthError),
    Validation(ValidationError),
    Collection(CollectionError),
    Intello(IntelloError),
    App(AppsError),
    Session(SessionError),
    Internal(InternalError),
}
```

Service errors convert to `AppError` via `From<ServiceError> for AppError`, and `AppError` implements actix-web's `ResponseError` trait for HTTP responses.

## Pipelines

For complex AI workflows (like Course Generation), we use a staged pipeline pattern:

```
Stage 0: Decryption & Expansion (Parallel AI calls)
         │
         ▼
Stage 1: Core Knowledge Extraction (Synthesis)
         │
         ▼
Stage 2: Structure Generation (Final Output)
```

This separates concerns (intent vs. knowledge vs. structure) and improves reliability.

## Dependency Injection

All dependencies are wired in `app.rs`. Services are wrapped in `Arc` for efficient cloning across Actix-web worker threads:

```rust
pub struct App {
    // Services wrapped in Arc for cheap cloning
    pub intello_service: Arc<IntelloService>,
    pub collection_service: Arc<CollectionService>,
    // ...
}

impl App {
    pub fn new(config: Config) -> Self {
        // ...
    }
}

// Clone is O(1) - just increments reference counts
impl Clone for App {
    fn clone(&self) -> Self {
        Self {
            intello_service: Arc::clone(&self.intello_service),
            collection_service: Arc::clone(&self.collection_service),
            // ...
        }
    }
}
```

### Why Arc-Wrapped Services?

Actix-web clones the app state for each worker thread (typically 1 per CPU core). Without `Arc`, this would recreate 12+ repositories on every clone. With `Arc`, clone becomes O(1) (just incrementing reference counts).

**Benefits:**
- ⚡ Fast startup (single initialization)
- 💾 Lower memory usage (shared instances)
- 🔒 Thread-safe (Arc + Send + Sync)

**Handler Compatibility:**
Handlers work unchanged because Rust auto-derefs `Arc<Service>` to `Service`:

```rust
// Works transparently:
app.intello_service.get_user_qcm_sets(&user_id).await?
```

## Testing

### Stub Repositories

For unit tests, use stub repository implementations instead of real Supabase connections. Stubs are defined in `src/tests/intello/helpers.rs`:

```rust
// Example stub (returns empty/default values)
pub struct StubTrueOrFalseRepository;

#[async_trait]
impl TrueOrFalseRepository for StubTrueOrFalseRepository {
    async fn insert(&self, set: &TrueOrFalseSet) -> Result<TrueOrFalseSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        Ok(vec![])
    }
    // ...
}
```

### Creating Test Services

```rust
use super::helpers::{StubTrueOrFalseRepository, StubKeywordsRepository, ...};

fn create_test_service() -> IntelloService {
    let qcm_repo = Arc::new(JsonQcmRepository::new("path/to/test.json"));
    let true_false_repo = Arc::new(StubTrueOrFalseRepository);
    // ...
    IntelloService::new(qcm_repo, true_false_repo, ...)
}
```

### Available Stubs

| Stub | Purpose |
|------|---------|
| `StubTrueOrFalseRepository` | Unit tests not using True/False features |
| `StubKeywordsRepository` | Unit tests not using Keywords features |
| `StubOrderPhraseRepository` | Unit tests not using Order Phrase features |
| `StubFillBlankRepository` | Unit tests not using Fill Blank features |

---

## Shared Use Case Types

The Intello use cases share common input/output types to reduce code duplication.

### GenerateGameInput

All AI game generation use cases use a shared input struct:

```rust
// use_cases/intello/shared.rs
pub struct GenerateGameInput {
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub language: String,
    pub level: Level,
    pub subjects: Vec<String>,
    pub num_questions: u8,
    pub documents: Vec<(String, String, u32)>,  // (filename, content, token_count)
    pub model: Option<String>,
}

// Type aliases for backwards compatibility
pub use GenerateGameInput as GenerateQcmInput;
pub use GenerateGameInput as GenerateKeywordsInput;
// ... etc
```

### GenerateGameOutput<T>

All game generation use cases return a generic output:

```rust
pub struct GenerateGameOutput<T> {
    pub game_set: T,  // The generated game set (QcmSet, KeywordSet, etc.)
    pub total_token_count: u32,
    pub documents_processed: usize,
}

pub type GenerateQcmOutput = GenerateGameOutput<QcmSet>;
pub type GenerateKeywordsOutput = GenerateGameOutput<KeywordSet>;
// ... etc
```

### validate_generation_input()

Shared validation logic for all use cases:

```rust
pub fn validate_generation_input(input: &GenerateGameInput) -> AppResult<u32> {
    // 1. Validate model
    // 2. Validate token count
    // 3. Validate subjects (at least one required)
    // 4. Validate num_questions (1-50)
    Ok(total_token_count)
}
```

---

## Generic Repository Trait

The `GameSetRepository<T>` trait provides a unified interface for all game set persistence:

```rust
// infrastructure/repository/game_set.rs
#[async_trait]
pub trait GameSetRepository<T: Clone + Send + Sync>: Send + Sync {
    async fn insert(&self, set: &T) -> Result<T, IntelloError>;
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<T>, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<T>, IntelloError>;
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError>;
}
```

Implemented by: `JsonFlashcardRepository`, all Supabase repositories (fill_blank, flashcard, keywords, order_phrase, true_false).

---

## Service CRUD Helpers

The `crud_ops` module provides generic CRUD operations for all game services:

```rust
// services/intello/crud_ops.rs
pub async fn get_user_sets<T>(repo: &dyn GameSetRepository<T>, user_id: &str, game_type: &str) -> Result<Vec<T>, IntelloError>;
pub async fn get_set<T>(repo: &dyn GameSetRepository<T>, set_id: &str, user_id: &str) -> Result<Option<T>, IntelloError>;
pub async fn delete_set<T>(repo: &dyn GameSetRepository<T>, set_id: &str, user_id: &str, game_type: &str) -> Result<bool, IntelloError>;
```

Usage in `*_ops.rs` files:

```rust
pub async fn get_user_keyword_sets(&self, user_id: &str) -> Result<Vec<KeywordSet>, IntelloError> {
    crud_ops::get_user_sets(self.keywords_repo.as_ref(), user_id, "keywords").await
}
```

---

## Generic Prompt Input

The `GamePromptInput` struct provides a unified input for all game prompt builders:

```rust
// shared/prompt_builder.rs
pub struct GamePromptInput {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub language: String,
    pub level: Level,
    pub subjects: Vec<String>,
    pub num_questions: u8,
    pub documents: Vec<(String, String)>,
}
```

This consolidates the 7 identical game-specific prompt input structs.
