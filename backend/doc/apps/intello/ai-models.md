# AI Model

All AI content generation uses **Gemini 3 Flash** (`google/gemini-3-flash-preview`).

> [!NOTE]
> Model selection has been removed. All game generation (QCM, flashcards, open questions, etc.) automatically uses the default model.

## Model Details

| Property | Value |
|----------|-------|
| Model ID | `google/gemini-3-flash-preview` |
| Display Name | Gemini 3 Flash |
| Context Limit | 1,048,576 tokens |
| Input Cost | $0.50 per million tokens |
| Output Cost | $3.00 per million tokens |

## Token Limits

- A 10K token safety buffer is subtracted from the context limit
- Tokens are counted from extracted document text
- Requests exceeding the limit return a validation error

## AI Cost Tracking

> [!IMPORTANT]
> All AI requests are logged to the `intello_ai_usage_log` table with:
> - Model used
> - Token counts (prompt, completion, total)
> - Calculated cost based on model pricing
> - User ID and feature type

### Feature Types

| Feature Type | Description |
|-------------|-------------|
| `qcm` | QCM (multiple choice) generation |
| `flashcard` | Flashcard generation |
| `open_question` | Open question generation |
| `true_false` | True/False generation |
| `keywords` | Keywords game generation |
| `order_phrase` | Order phrase generation |
| `fill_blank` | Fill blank generation |
| `course_decrypt` | Course: demand decryption |
| `course_expand` | Course: knowledge expansion (multi-model) |
| `course_extract` | Course: knowledge extraction |
| `course_content` | Course: educational content generation |
| `course_generate` | Course: main generation |
| `course_repair` | Course: JSON repair (if needed) |

This enables cost analysis and usage monitoring per user.

## Supported Documents

| Extension | Type |
|-----------|------|
| `.txt` | Plain text |
| `.pdf` | PDF |
| `.docx` | Microsoft Word |
| `.pptx` | Microsoft PowerPoint |
