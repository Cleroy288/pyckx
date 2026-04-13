# Rate Limiting

This application implements **per-user rate limiting** to prevent abuse and ensure fair resource usage.

---

## Architecture

### Components

1. **`RateLimitMiddleware`** - Actix-web middleware that wraps all routes
2. **`RateLimiter`** - Thread-safe store for tracking request counts per user/route
3. **`RateLimitConfig`** - Builder pattern for defining limits
4. **`RateLimitError`** - Returns 429 status with retry-after information

### User Identification

Requests are tracked by:
1. **Session ID** (preferred) - from `session_id` cookie
2. **X-Forwarded-For** - for proxied requests
3. **Client IP** - fallback

This ensures each **authenticated user** has their own quota.

---

## Current Limits

### Authentication Routes
- **Strict** (`/api/auth/login`, `/api/auth/register`):
  - 2 per second
  - 10 per minute
  - 50 per hour
  - 200 per day

### AI Generation Routes
- **AI Generation** (all AI creation endpoints):
  - **1 per minute**
  - 30 per hour
  - 100 per day

Routes limited:
- `/api/intello/qcm/generate`
- `/api/intello/open-questions`
- `/api/intello/flashcards`
- `/api/intello/true-false`
- `/api/intello/keywords`
- `/api/intello/order-phrases`
- `/api/intello/fill-blanks`
- `/api/intello/generate-course`

### Standard Routes
- **Standard** (default for all other routes):
  - 10 per second
  - 100 per minute
  - 1000 per hour

### Collection Routes
- **Relaxed** (`/api/collection/dvds`):
  - 30 per second
  - 300 per minute
  - 5000 per hour

---

## Implementation

Rate limits are configured in `main.rs`:

```rust
fn configure_rate_limiter() -> RateLimiter {
    let limiter = RateLimiter::with_default(RateLimitConfig::standard());
    
    // Auth - strict limits
    limiter.configure("/api/auth/login", RateLimitConfig::strict());
    limiter.configure("/api/auth/register", RateLimitConfig::strict());
    
    // AI Generation - 1 per minute
    let ai_config = RateLimitConfig::ai_generation();
    limiter.configure("/api/intello/qcm/generate", ai_config.clone());
    limiter.configure("/api/intello/open-questions", ai_config.clone());
    // ... (8 total AI routes)
    
    limiter
}
```

---

## Response Format

When rate limited, the API returns:
- **Status**: `429 Too Many Requests`
- **Error code**: `RATE_LIMIT_EXCEEDED`
- **Message**: Details about which limit was exceeded and when it resets

Example:
```json
{
  "code": "RATE_LIMIT_EXCEEDED",
  "message": "Rate limit exceeded for this endpoint (1 per minute). Try again in 45 seconds."
}
```

---

## Frontend Handling

The frontend displays user-friendly messages:

```typescript
if (errorMessage.includes("rate limit") || errorMessage.includes("429")) {
  setError("⏱️ Please wait 1 minute before creating another AI resource")
}
```

---

## Adding New Limits

To add limits to a new route:

1. **Choose a preset** in `middleware/rate_limit/config.rs`:
   - `strict()` - Authentication
   - `ai_generation()` - AI creation
   - `standard()` - Normal APIs
   - `relaxed()` - Read-heavy endpoints

2. **Configure in `main.rs`**:
   ```rust
   limiter.configure("/api/your/route", RateLimitConfig::strict());
   ```

3. **Or create custom limits**:
   ```rust
   limiter.configure(
       "/api/custom", 
       RateLimitConfig::new()
           .per_second(5)
           .per_minute(50)
   );
   ```

---

## Testing

Test rate limiting manually:
```bash
# Make multiple requests quickly
for i in {1..3}; do
  curl -X POST http://localhost:8080/api/intello/qcm/generate \
    -H "Cookie: session_id=test-session" \
    -d "{...}"
done
```

2nd request should return 429.
