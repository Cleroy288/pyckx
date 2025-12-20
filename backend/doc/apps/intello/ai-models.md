# AI Models

Available models for content generation via OpenRouter.

## Get Available Models

```
GET /app/intello/models
```

## Models

### Free Models

| Model ID | Context Limit |
|----------|---------------|
| `google/gemini-2.0-flash-exp:free` | 1.05M tokens **(default)** |
| `kwaipilot/kat-coder-pro:free` | 256K tokens |
| `mistralai/devstral-2512:free` | 262K tokens |
| `tngtech/deepseek-r1t2-chimera:free` | 164K tokens |

### Paid Models

| Model ID | Context Limit |
|----------|---------------|
| `google/gemini-3-flash-preview` | 1.05M tokens |
| `google/gemini-3-pro-preview` | 1.05M tokens |
| `openai/gpt-5.2` | 400K tokens |
| `amazon/nova-2-lite-v1` | 1M tokens |
| `x-ai/grok-4.1-fast` | 2M tokens |

## Usage

Pass `model` in the metadata when creating content:

```json
{
  "name": "My Quiz",
  "model": "google/gemini-2.0-flash-exp:free",
  ...
}
```

If omitted, defaults to `google/gemini-2.0-flash-exp:free`.

## Token Limits

- Each model has its own context limit (see table above)
- A 10K token safety buffer is subtracted from each limit
- Tokens are counted from extracted document text
- Requests exceeding the limit return a validation error

## Google API Key (Optional)

For higher rate limits with Google/Gemini models, add `GOOGLE_AI_KEY` to your `.env`:

```
GOOGLE_AI_KEY=your_google_ai_studio_api_key
```

Get a free key from: https://aistudio.google.com/apikey

## Supported Documents

| Extension | Type |
|-----------|------|
| `.txt` | Plain text |
| `.pdf` | PDF |
| `.docx` | Microsoft Word |
| `.pptx` | Microsoft PowerPoint |
