# AI Usage Domain

AI usage tracking and cost calculation data structures.

## Data Model

```rust
AiUsageLog {
    id: Uuid,
    user_id: Uuid,
    feature: AiFeature,
    model: String,
    input_tokens: i32,
    output_tokens: i32,
    cost: f64,
    created_at: DateTime
}

enum AiFeature {
    QcmGeneration,
    OpenQuestionGeneration,
    OpenQuestionGrading,
    FlashcardGeneration,
    TrueFalseGeneration,
    KeywordsGeneration,
    OrderPhraseGeneration,
    FillBlankGeneration,
    CourseGeneration
}
```

## Cost Calculation Rules

Cost is calculated based on:
- Model pricing (varies by provider)
- Input token count
- Output token count

Formula: `cost = (input_tokens * input_price) + (output_tokens * output_price)`

Prices are model-specific and configured per AI provider.
