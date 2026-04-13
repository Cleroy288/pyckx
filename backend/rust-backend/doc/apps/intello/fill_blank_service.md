# Fill Blank Service

Business logic for fill-in-the-blank game.

## Game Logic

AI generates phrases with blanks (marked as `___`). Users select the correct word from multiple options to fill in the blank.

## Generation Logic

1. Extract content from uploaded documents
2. AI identifies key sentences with important terms
3. For each question:
   - Removes a significant word, replacing with `___`
   - The removed word becomes the correct option
   - Generates plausible but incorrect distractors
4. Difficulty level influences:
   - **Easy**: Common words, obvious distractors
   - **Medium**: Technical terms, related distractors
   - **Hard**: Context-dependent words, subtle distractors
5. Validates exactly one correct option per question
6. Persists FillBlankSet

## Option Generation

Distractors are generated to be:
- Grammatically valid in the sentence
- Semantically related to the topic
- Clearly incorrect upon careful reading

## Storage Interactions
- Create: Insert new FillBlankSet with all questions
- Read: Fetch by ID or list all for user
- Delete: Remove set and all associated questions
