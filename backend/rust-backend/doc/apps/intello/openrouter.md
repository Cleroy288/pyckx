# OpenRouter AI Service

The `services/openrouter` module handles all interactions with the OpenRouter API (and by extension, Google Gemini/LLMs). It interacts with the Intello application layer but is distinct from it.

## Architecture

The service follows a strict separation of concerns:

-   **`*_domain.rs`**: Contains Data Transfer Objects (DTOs), AI response schemas, and internal data structures.
    *   *Example*: `keywords_domain.rs` contains `KeywordsAiResponse`.
-   **`*_service.rs`**: Contains the business logic, prompt building, API calls, and response parsing.
    *   *Example*: `keywords_service.rs` implements `generate_keywords()`.

## Core Components

### 1. Client & Configuration
-   **`client_service.rs`**: Defines the `OpenRouterService` struct and manages HTTP requests/responses.
-   **`models_domain.rs`**: The registry of available AI models (e.g., `google/gemini-2.0-flash-exp:free`).
-   **`types_domain.rs`**: Shared types like `OpenRouterRequest`, `Message`, and `Usage`.

### 2. Verification & Safety
-   **`verification_service.rs`**: Handles "JSON Repair". If the AI returns malformed JSON, this service captures the error and asks the AI to fix it.
-   **`utils_service.rs`**: Helpers for extracting JSON from Markdown blocks and sanitizing duplicate keys.

### 3. Course Generation Pipeline
Located in **`course_generation_service.rs`**.
It implements the 3-Stage Pipeline:
1.  **Decryption**: Understanding user intent.
2.  **Expansion**: Fetching external knowledge.
3.  **Generation**: creating the final course structure.

## Game Services

Each game type has a dedicated pair of files:

| Game | Domain (Data) | Service (Logic) |
|------|---------------|-----------------|
| **QCM** | `qcm_domain.rs` | `qcm_service.rs` |
| **Flashcards** | `flashcard_domain.rs` | `flashcard_service.rs` |
| **Open Questions** | `open_question_domain.rs` | `open_question_service.rs` |
| **Keywords** | `keywords_domain.rs` | `keywords_service.rs` |
| **Fill Blank** | `fill_blank_domain.rs` | `fill_blank_service.rs` |
| **Order Phrase** | `order_phrase_domain.rs` | `order_phrase_service.rs` |
| **True/False** | `true_false_domain.rs` | `true_false_service.rs` |

## Usage Example

The Intello application layer (`services/intello`) calls these services:

```rust
// Intello Service calls OpenRouter Service
let (questions, usage) = open_router_service.generate_keywords(&input, model).await?;
```
