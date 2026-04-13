# Open Question Service

Business logic for open-ended questions with AI grading.

## AI Grading Logic

When a user submits an answer, the service:
1. Retrieves the question's `expected_answer`
2. Sends both user answer and expected answer to AI
3. AI evaluates semantic similarity and correctness
4. Returns a grade with explanation

### Grade Values

| Grade | Meaning |
|-------|---------|
| `right` | Correct answer - matches expected meaning |
| `medium` | Partially correct - some key points missing |
| `error` | Incorrect - does not match expected answer |

## Caching Mechanism

`OpenQuestionCache` stores:
- Recently graded answers to avoid redundant AI calls
- Question context for faster subsequent grading
- Cache invalidation on question update

## Generation Flow
1. Extract content from uploaded documents
2. AI generates questions with expected answers
3. Optionally generates hints based on difficulty level
4. Validates and persists OpenQuestionSet
