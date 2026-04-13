# QCM Service

Business logic for QCM (Multiple Choice Questions) management.

## Creation Modes

### Manual Creation
User provides questions directly via JSON payload. The service:
1. Validates all fields against domain rules
2. Ensures answer uniqueness per question
3. Persists to storage

### AI Generation
Documents are processed through the AI pipeline:
1. Extract text content from uploaded files (PDF, DOCX, PPTX, TXT)
2. Send to AI model with generation instructions
3. Parse and validate AI response
4. Sanitize any duplicate JSON keys
5. Persist generated QcmSet

## Processing Steps

### Grading
Each question has exactly one `right_answer`. Comparison is case-insensitive.

### Answer Shuffling
When presenting questions, answers (wrong + right) should be shuffled client-side to prevent position-based memorization.

## Storage Interactions
- Create: Insert new QcmSet with all questions
- Read: Fetch by ID or list all for user
- Update: Replace entire set
- Delete: Remove set and all associated questions
