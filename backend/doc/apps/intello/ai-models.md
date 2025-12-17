# AI Models

Available free models for content generation via OpenRouter.

## Get Available Models

```
GET /app/intello/models
```

## Models

| Model ID | Name |
|----------|------|
| `nvidia/nemotron-nano-12b-v2-vl:free` | NVIDIA Nemotron Nano 12B |
| `tngtech/deepseek-r1t-chimera:free` | DeepSeek R1T Chimera |
| `z-ai/glm-4.5-air:free` | GLM 4.5 Air |
| `nex-agi/deepseek-v3.1-nex-n1:free` | DeepSeek v3.1 Nex N1 |
| `amazon/nova-2-lite-v1:free` | Amazon Nova 2 Lite **(default)** |
| `tngtech/deepseek-r1t2-chimera:free` | DeepSeek R1T2 Chimera |

## Usage

Pass `model` in the metadata when creating content:

```json
{
  "name": "My Quiz",
  "model": "nvidia/nemotron-nano-12b-v2-vl:free",
  ...
}
```

If omitted, defaults to `amazon/nova-2-lite-v1:free`.

## Token Limits

- **Max tokens per request**: 800,000
- Tokens are counted from extracted document text

## Supported Documents

| Extension | Type |
|-----------|------|
| `.txt` | Plain text |
| `.pdf` | PDF |
| `.docx` | Microsoft Word |
| `.pptx` | Microsoft PowerPoint |
