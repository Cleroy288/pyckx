# Use Cases Layer

The Use Cases layer provides application orchestration between HTTP handlers and domain services.

## Architecture

```
Handler → Use Case → Service(s) → Repository
```

### When to Use

| Scenario | Use Case Layer |
|----------|---------------|
| Simple CRUD with single service | Optional |
| Multi-step workflows | Required |
| Cross-service coordination | Required |
| Shared validation logic | Required |
| Complex business rules | Required |

### Exempt: Auth Services
Auth operations remain simple handler → service flows (no Use Case needed).

## Current Status

**17 Use Cases implemented, 12 handlers migrated.**

### Migrated Handlers

| Handler File | Handlers | Use Cases Used |
|-------------|----------|----------------|
| `ai_generation.rs` | 3 | GenerateQcm, GenerateOpenQuestions, GenerateFlashcards |
| `true_false.rs` | 1 | GenerateTrueFalse |
| `keywords.rs` | 1 | GenerateKeywords |
| `order_phrase.rs` | 1 | GenerateOrderPhrase |
| `fill_blank.rs` | 1 | GenerateFillBlank |
| `open_question.rs` | 1 | CheckOpenQuestionAnswers |
| `dvd_handlers.rs` | 4 | AddDvd, ListDvds, UpdateDvd, DeleteDvd |

## Folder Structure

```
src/use_cases/
├── mod.rs                    # Main exports
├── intello/                  # Intello app use cases (10)
│   ├── generate_qcm.rs
│   ├── generate_flashcards.rs
│   ├── generate_open_questions.rs
│   ├── generate_true_false.rs
│   ├── generate_keywords.rs
│   ├── generate_order_phrase.rs
│   ├── generate_fill_blank.rs
│   ├── check_answers.rs
│   ├── get_user_sets.rs      # Not yet connected
│   └── delete_set.rs         # Not yet connected
├── collection/               # Collection app use cases (5)
│   ├── add_dvd.rs
│   ├── update_dvd.rs
│   ├── delete_dvd.rs
│   ├── list_dvds.rs
│   └── delete_collection.rs  # Not yet connected
└── ai/                       # Shared AI use cases (2)
    ├── generate_content.rs   # Utility, not directly used
    └── verify_answers.rs     # Utility, not directly used
```

## Use Case Structure

Each use case follows this pattern:

```rust
pub struct GenerateQcmInput {
    pub user_id: String,
    // ... input fields
}

pub struct GenerateQcmOutput {
    pub qcm_set: QcmSet,
    // ... output fields
}

pub struct GenerateQcmUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateQcmUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    pub async fn execute(&self, input: GenerateQcmInput) -> AppResult<GenerateQcmOutput> {
        // 1. Validate input
        // 2. Coordinate services
        // 3. Return result
    }
}
```

## Handler Integration

Handlers instantiate and execute use cases:

```rust
#[post("/qcm/generate")]
pub async fn create_qcm_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let (metadata, documents) = parse_multipart::<CreateQcmRequest>(payload).await?;
    
    // Build use case input
    let input = GenerateQcmInput {
        user_id,
        name: metadata.name,
        // ... map other fields
    };
    
    // Execute use case
    let use_case = GenerateQcmUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;
    
    // Map to response
    Ok(HttpResponse::Created().json(QcmResponse::from(output)))
}
```

## Adding a New Use Case

1. Create file in appropriate module (e.g., `use_cases/intello/my_feature.rs`)
2. Define input/output structs
3. Implement use case struct with `new()` and `execute()`
4. Export in module's `mod.rs`
5. Update handler to use the new use case

## Testing

Use cases are easily testable with stubbed services:

```rust
#[tokio::test]
async fn test_generate_qcm() {
    let service = Arc::new(create_test_intello_service());
    let use_case = GenerateQcmUseCase::new(service);
    
    let input = GenerateQcmInput {
        user_id: "user-1".to_string(),
        // ...
    };
    
    let result = use_case.execute(input).await;
    assert!(result.is_ok());
}
```
