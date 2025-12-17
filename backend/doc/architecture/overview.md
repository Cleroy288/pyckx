# Architecture Overview

## Layered Architecture

```
┌──────────────────────────────────────────────────┐
│  HTTP Layer (api/handlers/)                       │
│  - Extract session, parse DTOs, call service      │
└──────────────────────────────────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────────┐
│  Business Layer (services/)                       │
│  - Validation, business rules, orchestration      │
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
│             │  3. Call service
└─────────────┘
      │
      ▼
┌─────────────┐
│   Service   │  4. Validate input
│             │  5. Apply business rules
│             │  6. Call repository
└─────────────┘
      │
      ▼
┌─────────────┐
│ Repository  │  7. Execute I/O (Supabase/JSON)
└─────────────┘
      │
      ▼
Response flows back up
```

## Folder Structure

```
src/
├── api/
│   ├── handlers/          # Route handlers by domain
│   │   ├── auth/
│   │   ├── apps/
│   │   ├── collection/
│   │   └── intello/
│   └── dto/               # Request/Response types
│       └── {domain}/
│           ├── request.rs
│           ├── response.rs
│           └── conversion.rs
├── services/
│   ├── auth/
│   ├── apps/
│   ├── collection/
│   ├── intello/           # Game business logic
│   └── openrouter/        # AI API calls
├── domain/
│   ├── apps/              # App, UserApp
│   ├── collection/        # Dvd, UserCollection
│   └── intello/           # QcmSet, Flashcard, OpenQuestion, etc.
├── infrastructure/
│   ├── repository/        # Trait definitions
│   └── supabase/          # Supabase implementations
├── error/                 # Domain-specific errors
│   ├── app.rs             # Main AppError
│   ├── auth.rs
│   ├── collection.rs
│   └── intello.rs
└── shared/
    ├── document_extractor.rs  # PDF, Word, PPTX parsing
    ├── prompt_builder.rs      # AI prompt generation
    └── open_question_cache.rs # Grading context cache
```

## Key Principles

1. **Handlers are thin** - Only HTTP concerns (parse, delegate, respond)
2. **Services own business logic** - Validation, rules, orchestration
3. **Repositories are traits** - Enables swappable backends, testing
4. **Domain is pure** - No I/O, no dependencies
5. **Errors per domain** - Convert to `AppError` via `From` trait

## Dependency Injection

All dependencies are wired in `app.rs`:

```rust
pub struct App {
    pub intello_service: IntelloService,
    pub collection_service: CollectionService,
    pub auth_service: AuthService,
    // ...
}

impl App {
    pub fn new(config: Config) -> Self {
        // Create repositories
        let qcm_repo = Arc::new(SupabaseQcmRepository::new(...));
        
        // Inject into services
        let intello_service = IntelloService::new(qcm_repo, ...);
        
        Self { intello_service, ... }
    }
}
```
