# Order Phrase Service

Business logic for word arrangement game.

## Game Logic

Users arrange shuffled words into the correct order. Each word has a `position` field indicating its correct placement (0-indexed).

## Generation Logic

1. Extract content from uploaded documents
2. AI selects meaningful phrases from the content
3. For each phrase:
   - Splits into individual words
   - Assigns position index to each word
   - Optionally generates a hint
4. Difficulty level influences:
   - **Easy**: Short phrases (3-5 words)
   - **Medium**: Medium phrases (5-8 words)
   - **Hard**: Long phrases with complex structure
5. Validates word positions are distinct and sequential
6. Persists OrderPhraseSet

## Shuffling

Words are stored with their correct positions. Client-side shuffling:
1. Display words in random order
2. User drags/selects to reorder
3. Compare user order against `position` values
4. All positions must match for correct answer

## Storage Interactions
- Create: Insert new OrderPhraseSet with all questions
- Read: Fetch by ID or list all for user
- Delete: Remove set and all associated questions
