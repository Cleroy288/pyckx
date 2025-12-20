# Adding a New Feature

Step-by-step guide for adding a new game, service, or CRUD feature.

## 1. Create Domain Entity

Location: `domain/{feature}/`

```rust
// domain/intello/my_game.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyGameSet {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub questions: Vec<MyQuestion>,
}
```

Export in `domain/intello/mod.rs`.

## 2. Create Repository Trait

Location: `infrastructure/repository/{feature}/`

```rust
// infrastructure/repository/my_game.rs
#[async_trait]
pub trait MyGameRepository: Send + Sync {
    async fn insert(&self, set: &MyGameSet) -> Result<MyGameSet, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<MyGameSet>, IntelloError>;
    async fn find_by_id(&self, id: &str, user_id: &str) -> Result<Option<MyGameSet>, IntelloError>;
    async fn delete(&self, id: &str, user_id: &str) -> Result<bool, IntelloError>;
}
```

## 3. Implement Repository

Location: `infrastructure/supabase/{feature}/`

```rust
// infrastructure/supabase/my_game/repository.rs
pub struct SupabaseMyGameRepository { url: String, client: Client }

#[async_trait]
impl MyGameRepository for SupabaseMyGameRepository {
    async fn insert(&self, set: &MyGameSet) -> Result<MyGameSet, IntelloError> {
        // PostgREST API call
    }
}
```

## 4. Add Service Methods

Location: `services/intello/my_game_ops.rs`

```rust
impl IntelloService {
    pub async fn create_my_game(&self, input: GenerateContentInput) -> Result<MyGameSet, IntelloError> {
        // 1. Validate input
        self.validate_input(&input)?;
        
        // 2. Generate with AI (optional)
        let questions = self.openrouter.generate_my_game(&input).await?;
        
        // 3. Build entity
        let set = MyGameSet { id: uuid(), user_id: input.user_id, questions, ... };
        
        // 4. Save to repository
        self.my_game_repo.insert(&set).await
    }
}
```

## 5. Create Use Case

Location: `use_cases/intello/my_game.rs`

```rust
pub struct GenerateMyGameInput {
    pub user_id: String,
    pub name: String,
    // ... other input fields
}

pub struct GenerateMyGameOutput {
    pub my_game_set: MyGameSet,
}

pub struct GenerateMyGameUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateMyGameUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    pub async fn execute(&self, input: GenerateMyGameInput) -> AppResult<GenerateMyGameOutput> {
        // 1. Validate input
        // 2. Delegate to service
        let set = self.intello_service.create_my_game(input).await?;
        Ok(GenerateMyGameOutput { my_game_set: set })
    }
}
```

Export in `use_cases/intello/mod.rs`.

## 6. Create DTOs

Location: `api/dto/intello/my_game/`

```rust
// request.rs
#[derive(Deserialize)]
pub struct CreateMyGameRequest {
    pub name: String,
    pub level: String,
    // ...
}

// response.rs
#[derive(Serialize)]
pub struct MyGameResponse {
    pub success: bool,
    pub id: String,
    pub questions: Vec<MyQuestionResponse>,
}
```

## 7. Create Handler

Location: `api/handlers/intello/my_game.rs`

```rust
#[post("/my-game/create")]
pub async fn create_my_game_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let input = parse_multipart(payload).await?;
    
    // Use the use case for orchestration
    let use_case = GenerateMyGameUseCase::new(Arc::clone(&app.intello_service));
    let output = use_case.execute(input).await?;
    
    Ok(HttpResponse::Created().json(MyGameResponse::from(output)))
}
```

## 8. Wire Everything

```rust
#[post("/my-game/create")]
pub async fn create_my_game_handler(
    app: web::Data<App>,
    session: Session,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    let user_id = session.user_id()?;
    let input = parse_multipart(payload).await?;
    let result = app.intello_service.create_my_game(user_id, input).await?;
    Ok(HttpResponse::Created().json(MyGameResponse::from(result)))
}
```

## 7. Wire Everything

### Add repository to `IntelloRepositories` in `services/intello/types.rs`:
```rust
/// Bundle of all Intello repositories
pub struct IntelloRepositories {
    // Add new repository
    pub my_game_repo: Arc<dyn MyGameRepository>,
    // ... existing repos
}
```

### Create and inject repository in `app.rs`:
```rust
pub fn new() -> Result<Self, ConfigError> {
    // Create repository bundle with new game repo
    let intello_repos = IntelloRepositories {
        my_game_repo: Arc::new(SupabaseMyGameRepository::new(&cfg)),
        // ... existing repos
    };
    
    // Build IntelloService using builder pattern
    let intello_service = Arc::new(
        IntelloService::builder()
            .with_repositories(intello_repos)
            .with_openrouter(openrouter_service)
            .with_cache(open_question_cache)
            .build()
            .expect("IntelloService must have all dependencies"),
    );
    
    // ...
}
```

> **Note:** Services are wrapped in `Arc` for efficient cloning across Actix workers. Repositories are bundled in `IntelloRepositories` for cleaner construction.

### Update test helpers in `tests/intello/helpers.rs`:
```rust
pub fn stub_intello_repositories() -> IntelloRepositories {
    IntelloRepositories {
        my_game_repo: Arc::new(StubMyGameRepository),
        // ... existing repos
    }
}
```

### Register routes in `api/handlers/intello/mod.rs`:
```rust
.service(my_game::create_my_game_handler)
.service(my_game::list_my_game_handler)
```

### Export types in each `mod.rs`:
- `domain/intello/mod.rs`
- `infrastructure/repository/mod.rs`
- `api/dto/intello/mod.rs`

## Checklist

- [ ] Domain entity created and exported
- [ ] Repository trait defined
- [ ] Supabase implementation added
- [ ] Service methods implemented
- [ ] **Use Case created and exported**
- [ ] DTOs created (request + response)
- [ ] Handler created and registered
- [ ] Dependencies wired in `app.rs`
- [ ] Database table created in Supabase
