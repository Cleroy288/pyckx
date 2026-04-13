# Flashcard Service

Business logic for flashcard generation and management.

## Generation Logic

1. Extract content from uploaded documents
2. AI identifies key concepts and definitions
3. Generates front (term/question) and back (definition/answer) pairs
4. Difficulty level influences:
   - **Easy**: Basic definitions and simple concepts
   - **Medium**: Relationships between concepts
   - **Hard**: Complex applications and edge cases
5. Validates card content
6. Persists FlashcardSet

## Storage Interactions
- Create: Insert new FlashcardSet with all cards
- Read: Fetch by ID or list all for user
- Delete: Remove set and all associated cards
