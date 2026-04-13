# Keywords Service

Business logic for keyword identification game.

## Generation Logic

1. Extract content from uploaded documents
2. AI identifies key terms and concepts
3. For each question:
   - Creates a statement/context
   - Selects correct keywords from the content
   - Generates plausible but incorrect distractors
4. Difficulty level influences:
   - **Easy**: Obvious keywords, clear distractors
   - **Medium**: Related terms as distractors
   - **Hard**: Subtle distinctions between similar terms
5. Validates keyword content
6. Persists KeywordSet

## Storage Interactions
- Create: Insert new KeywordSet with all questions
- Read: Fetch by ID or list all for user
- Delete: Remove set and all associated questions
