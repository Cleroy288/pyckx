# End-to-End Feature Implementation Guide

Complete guide for implementing a new Intello game from database to playable UI.

---

## Overview

Adding a new game requires changes in **3 areas**:
1. **Database** - SQL schema for data persistence
2. **Backend** - API endpoints and business logic
3. **Frontend** - UI components and state management

This guide uses "Order Phrase" as a reference implementation.

---

## Phase 1: Backend Implementation

### Step 1: Database Schema

**Location:** `backend/doc/db_shema/<game_name>.txt`

Create tables matching this pattern:
```sql
-- Sets table (parent)
CREATE TABLE intello_<game>_sets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    level TEXT NOT NULL DEFAULT 'medium',
    language TEXT NOT NULL DEFAULT 'en',
    subjects TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Questions table (child of sets)
CREATE TABLE intello_<game>_questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    set_id UUID NOT NULL REFERENCES intello_<game>_sets(id) ON DELETE CASCADE,
    -- game-specific fields
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Enable RLS
ALTER TABLE intello_<game>_sets ENABLE ROW LEVEL SECURITY;
CREATE POLICY "users_own_sets" ON intello_<game>_sets
    FOR ALL USING (auth.uid() = user_id);
```

Run the SQL in Supabase dashboard.

---

### Step 2: Domain Entities

**Location:** `backend/src/domain/intello/<game>.rs`

```rust
use serde::{Deserialize, Serialize};
use crate::domain::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyGameSet {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: Level,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<MyGameQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyGameQuestion {
    pub id: String,
    // game-specific fields
}
```

**Export in** `domain/intello/mod.rs`:
```rust
pub mod my_game;
pub use my_game::*;
```

---

### Step 3: Repository Trait

**Location:** `backend/src/infrastructure/repository/<game>/trait.rs`

```rust
use async_trait::async_trait;
use crate::domain::intello::MyGameSet;
use crate::error::IntelloError;

#[async_trait]
pub trait MyGameRepository: Send + Sync {
    async fn insert(&self, set: &MyGameSet) -> Result<MyGameSet, IntelloError>;
    async fn find_by_id(&self, id: &str, user_id: &str) -> Result<Option<MyGameSet>, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<MyGameSet>, IntelloError>;
    async fn delete(&self, id: &str, user_id: &str) -> Result<bool, IntelloError>;
}
```

**Export in** `infrastructure/repository/mod.rs`

---

### Step 4: Supabase Repository Implementation

**Location:** `backend/src/infrastructure/supabase/<game>/`

Create:
- `types.rs` - Row structs for Supabase
- `repository.rs` - Implementation
- `mod.rs` - Module exports

Key pattern for cascading inserts:
```rust
async fn insert(&self, set: &MyGameSet) -> Result<MyGameSet, IntelloError> {
    // 1. Insert set
    self.client.post(&self.sets_endpoint()).json(&row).send().await?;
    
    // 2. Insert questions (child records)
    self.insert_questions(&set.id, &set.questions).await?;
    
    Ok(set.clone())
}
```

---

### Step 5: Service Operations

**Location:** `backend/src/services/intello/<game>_ops.rs`

```rust
impl IntelloService {
    pub async fn generate_ai_my_game(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<MyGameSet, IntelloError> {
        // 1. Validate
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;
        
        // 2. Generate with AI
        let questions = self.openrouter_service
            .generate_my_game(&prompt_input, input.model.as_deref())
            .await?;
        
        // 3. Build set
        let set = MyGameSet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            questions,
            // ...
        };
        
        // 4. Store
        self.my_game_repo.insert(&set).await
    }
    
    pub async fn get_user_my_game_sets(&self, user_id: &str) -> Result<Vec<MyGameSet>, IntelloError> {
        self.my_game_repo.find_by_user(user_id).await
    }
}
```

**Update** `services/intello/types.rs` to add repository field.

---

### Step 6: AI Prompt Builder

**Location:** `backend/src/shared/prompt_builder.rs`

Add game format to `GAME_FORMATS`:
```rust
("my_game", r#"[{"question": "...", "field2": "..."}]"#),
```

Add prompt builder function:
```rust
pub fn build_my_game_prompt(input: &MyGamePromptInput) -> String {
    // Build prompt with GAME_FORMATS["my_game"]
}
```

---

### Step 7: OpenRouter Integration

**Location:** `backend/src/services/openrouter/<game>.rs`

```rust
impl OpenRouterService {
    pub async fn generate_my_game(
        &self,
        input: &MyGamePromptInput,
        model: Option<&str>,
    ) -> Result<Vec<MyGameQuestion>, IntelloError> {
        let prompt = build_my_game_prompt(input);
        let content = self.generate_content(&prompt, model).await?;
        let questions: Vec<MyGameQuestion> = serde_json::from_str(&content)?;
        Ok(questions)
    }
}
```

---

### Step 8: DTOs

**Location:** `backend/src/api/dto/intello/<game>/`

Create:
- `request.rs` - `CreateMyGameRequest`
- `response.rs` - `MyGameResponse`, `MyGameSetListResponse`
- `mod.rs` - Exports

---

### Step 9: Handlers

**Location:** `backend/src/api/handlers/intello/<game>.rs`

```rust
#[post("/my-game/create")]
pub async fn create_my_game_handler(
    app: web::Data<App>,
    req: HttpRequest,
    payload: Multipart,
) -> AppResult<HttpResponse> {
    let user_id = get_user_id_from_session(&app, &req)?;
    let (metadata, documents) = parse_multipart::<CreateMyGameRequest>(payload).await?;
    let set = app.intello_service.generate_ai_my_game(&user_id, input).await?;
    Ok(HttpResponse::Created().json(response))
}

#[get("/my-game/list")]
pub async fn list_my_game_handler(...) { ... }
```

**Register in** `api/handlers/intello/mod.rs`:
```rust
.service(my_game::create_my_game_handler)
.service(my_game::list_my_game_handler)
```

---

### Step 10: Wire in App

**Location:** `backend/src/app.rs`

```rust
pub struct App {
    // Add repository
    my_game_repo: Arc<dyn MyGameRepository>,
}

impl App {
    pub fn new(config: &Config) -> Self {
        let my_game_repo = Arc::new(SupabaseMyGameRepository::new(config));
        
        let intello_service = IntelloService::new(
            // ... existing repos
            my_game_repo.clone(),
        );
        // ...
    }
}
```

---

## Phase 2: Frontend Implementation

### Step 1: API Configuration

**Location:** `lib/api/config.ts`

```typescript
intello: {
  // existing...
  myGameCreate: () => `${BACKEND}/app/intello/my-game/create`,
  myGameList: () => `${BACKEND}/app/intello/my-game/list`,
}
```

---

### Step 2: API Types and Functions

**Location:** `lib/api/intello.ts`

```typescript
// Types
export interface MyGameQuestionData {
  id: string;
  // game-specific fields
}

export interface MyGameSetData {
  id: string;
  name: string;
  questions: MyGameQuestionData[];
  // ...
}

// API Functions
export async function createMyGame(
  input: CreateMyGameInput,
  files: File[]
): Promise<CreateMyGameResponse> {
  const formData = new FormData();
  formData.append("metadata", JSON.stringify(input));
  files.forEach(f => formData.append("files", f));
  
  const res = await fetch(endpoints.intello.myGameCreate(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });
  return res.json();
}

export async function getAllMyGameSets(): Promise<MyGameSetListResponse> {
  const res = await fetch(endpoints.intello.myGameList(), {
    credentials: "include",
  });
  return res.json();
}
```

---

### Step 3: Context Updates

**Location:** `app/intello/context.tsx`

Add to `ViewState` type:
```typescript
| "create-ai-my-game"
| "play-my-game"
| "playing-my-game"
```

Add state:
```typescript
const [myGameSets, setMyGameSets] = useState<MyGameSetData[]>([]);
const [selectedMyGameSet, setSelectedMyGameSet] = useState<MyGameSetData | null>(null);
```

Add handlers:
```typescript
const loadMyGameSets = async () => {
  const response = await getAllMyGameSets();
  setMyGameSets(response.sets);
};

const handlePlayMyGame = async () => {
  await loadMyGameSets();
  setView("play-my-game");
};

const handleSelectMyGameSet = (set: MyGameSetData) => {
  setSelectedMyGameSet(set);
  setView("playing-my-game");
};
```

Update `handleNavigateToGame`:
```typescript
case "my_game":
  await loadMyGameSets();
  setView("play-my-game");
  break;
```

Add to context value object.

---

### Step 4: Create View

**Location:** `app/intello/views/create-views.tsx`

```tsx
export function CreateAiMyGameView() {
  const { games, handleBackToHome, handleNavigateToGame } = useIntello();
  
  return (
    <div className="space-y-6 max-w-2xl mx-auto">
      {/* Header */}
      <CustomQuestionForm 
        games={games} 
        onBack={handleBackToHome} 
        defaultOutputGame="my_game" 
        onNavigateToGame={handleNavigateToGame} 
      />
    </div>
  );
}
```

---

### Step 5: Play View (Set Selection)

**Location:** `app/intello/views/play-views.tsx`

```tsx
export function PlayMyGameView() {
  const { loading, error, myGameSets, handleBackToHome, handleSelectMyGameSet } = useIntello();
  
  return (
    <div className="space-y-6">
      {/* Header with back button */}
      {/* Loading/error states */}
      {/* Grid of set cards that call handleSelectMyGameSet on click */}
    </div>
  );
}
```

---

### Step 6: Playing View

**Location:** `app/intello/views/playing-views.tsx`

```tsx
export function PlayingMyGameView() {
  const { selectedMyGameSet, setView } = useIntello();
  
  if (!selectedMyGameSet) return null;
  
  return (
    <MyGamePlayer
      questions={selectedMyGameSet.questions}
      title={selectedMyGameSet.name}
      onBack={() => setView("play-my-game")}
    />
  );
}
```

---

### Step 7: Player Component

**Location:** `components/my-game-player.tsx`

```tsx
interface MyGamePlayerProps {
  questions: MyGameQuestionData[];
  title: string;
  onBack: () => void;
}

export function MyGamePlayer({ questions, title, onBack }: MyGamePlayerProps) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [results, setResults] = useState<boolean[]>([]);
  const [isComplete, setIsComplete] = useState(false);
  
  // Game-specific state and logic
  
  // Initialize on question change
  useEffect(() => {
    // Setup for current question
  }, [currentIndex]);
  
  // Results screen
  if (isComplete) {
    return <ResultsScreen score={score} onBack={onBack} />;
  }
  
  // Game UI
  return (
    <div>
      {/* Progress bar */}
      {/* Question display */}
      {/* Interactive game elements */}
      {/* Check/Next buttons */}
    </div>
  );
}
```

---

### Step 8: Update CustomQuestionForm

**Location:** `components/custom-question-form.tsx`

Add import:
```typescript
import { createMyGame, type MyGameQuestionData } from "@/lib/api/intello";
import { MyGamePlayer } from "./my-game-player";
```

Add to `onNavigateToGame` type:
```typescript
| "my_game"
```

Add to `successData.gameType` type:
```typescript
| "my_game"
```

Add state:
```typescript
const [myGameQuestions, setMyGameQuestions] = useState<MyGameQuestionData[] | null>(null);
const [myGameTitle, setMyGameTitle] = useState("");
```

Add handler in `handleSubmit`:
```typescript
} else if (outputGame === "my_game") {
  const response = await createMyGame(input, files);
  setSuccessData({ message: `Generated ${response.questions.length} questions!`, gameType: "my_game", ... });
  setMyGameQuestions(response.questions);
  setMyGameTitle(name.trim());
}
```

Add to `gameTypeLabels`:
```typescript
my_game: "My Game Sets",
```

Add to `handleBackFromPlayer` reset:
```typescript
setMyGameQuestions(null);
setMyGameTitle("");
```

Add player rendering:
```tsx
if (myGameQuestions && myGameQuestions.length > 0 && !successData) {
  return <MyGamePlayer questions={myGameQuestions} title={myGameTitle} onBack={handleBackFromPlayer} />;
}
```

---

### Step 9: Home View Buttons

**Location:** `app/intello/views/home-view.tsx`

Add create button in Create section:
```tsx
<Button onClick={() => setView("create-ai-my-game")}>
  <Icon /> AI My Game
</Button>
```

Add play button in Play section:
```tsx
<Button onClick={handlePlayMyGame}>
  <Icon /> My Game Sets
</Button>
```

---

### Step 10: Export and Route

**Location:** `app/intello/views/index.ts`
```typescript
export { CreateAiMyGameView } from "./create-views";
export { PlayMyGameView } from "./play-views";
export { PlayingMyGameView } from "./playing-views";
```

**Location:** `app/intello/page.tsx`
```tsx
{view === "create-ai-my-game" && <CreateAiMyGameView />}
{view === "play-my-game" && <PlayMyGameView />}
{view === "playing-my-game" && <PlayingMyGameView />}
```

---

## Verification Checklist

### Backend
- [ ] SQL schema executed in Supabase
- [ ] `cargo build` passes
- [ ] Test with curl/Postman

### Frontend
- [ ] `npm run build` passes
- [ ] Create button appears on home
- [ ] Form submits successfully
- [ ] "Play Now" works after creation
- [ ] Game is playable
- [ ] "Go to Sets" shows saved games
- [ ] Saved games are playable

---

## Common Pitfalls

1. **CustomQuestionForm missing handler** - The form falls through to QCM if your game type isn't handled
2. **Context not exposing state** - Add all state/handlers to context value
3. **useEffect dependencies** - Ensure player initializes on question change
4. **Type mismatches** - Keep response types consistent between backend DTO and frontend interface
