# AI Usage Service

Business logic for AI usage tracking and logging.

## Logging Mechanism

All AI requests are logged with:
1. User identification
2. Feature being used
3. Model selected
4. Token counts (input/output)
5. Calculated cost
6. Timestamp

## Logging Flow

1. Before AI call: Record request metadata
2. After AI response: Capture token usage from response
3. Calculate cost based on model pricing
4. Persist AiUsageLog entry

## Usage Tracking

Enables:
- Per-user usage monitoring
- Cost attribution by feature
- Model performance comparison
- Usage analytics and reporting

## Storage Interactions
- Create: Log each AI request
- Read: Query usage by user, feature, date range
- Aggregate: Sum costs, token counts for reporting
