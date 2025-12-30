# True/False Service

Business logic for true/false statement generation.

## Generation Logic

1. Extract content from uploaded documents
2. AI generates factual statements from the content
3. Mix of true and false statements created
4. Each statement includes an explanation
5. Difficulty level influences statement complexity:
   - **Easy**: Direct facts from the text
   - **Medium**: Inferences and relationships
   - **Hard**: Subtle distinctions and edge cases
6. Validates statement content
7. Persists TrueOrFalseSet

## Storage Interactions
- Create: Insert new TrueOrFalseSet with all statements
- Read: Fetch by ID or list all for user
- Delete: Remove set and all associated statements
