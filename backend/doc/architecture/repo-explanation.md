# Repository Explanation - Quick Reference

Where to add code for a new feature.

## Quick Lookup Table

| What | Where | Purpose |
|------|-------|---------|
| **Domain Entity** | `domain/{app}/` | Pure data structs (`MyEntity`) |
| **Repository Trait** | `infrastructure/repository/{feature}/` | Database abstraction |
| **Supabase Impl** | `infrastructure/supabase/{feature}/` | Actual DB calls |
| **Service Methods** | `services/{app}/{feature}_ops.rs` | Business logic |
| **Use Case** | `use_cases/{app}/{feature}.rs` | Input validation + orchestration |
| **Request DTO** | `api/dto/{app}/{feature}/request.rs` | HTTP request parsing |
| **Response DTO** | `api/dto/{app}/{feature}/response.rs` | HTTP response format |
| **Handler** | `api/handlers/{app}/{feature}.rs` | HTTP endpoint |
| **Wiring** | `app.rs` | Dependency injection |
| **Routes** | `api/handlers/{app}/mod.rs` | Register endpoint |
| **Error Codes** | `error/shared/code.rs` + `shared/constants/errors.rs` | Error handling |
| **Tests** | `tests/{app}/` | Unit/property tests |

## Architecture Flow

```
HTTP Request
     │
     ▼
┌──────────────────────────────────────────────────────────────────┐
│  api/handlers/     →  Parse DTO, extract session                 │
└──────────────────────────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────────────────────┐
│  use_cases/        →  Validate input, coordinate services        │
└──────────────────────────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────────────────────┐
│  services/         →  Business rules, call repository            │
└──────────────────────────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────────────────────┐
│  infrastructure/   →  Supabase/external API calls                │
└──────────────────────────────────────────────────────────────────┘
```

## Export Chain (mod.rs files to update)

1. `domain/{app}/mod.rs` → Export entity
2. `infrastructure/repository/mod.rs` → Export trait
3. `infrastructure/supabase/mod.rs` → Export implementation
4. `infrastructure/mod.rs` → Re-export trait + impl
5. `api/dto/{app}/mod.rs` → Export DTOs
6. `use_cases/{app}/mod.rs` → Export use case
7. `api/handlers/{app}/mod.rs` → Register route

## Files to Touch for a New Intello Game

```
1. domain/intello/my_game.rs          ← Entity
2. domain/intello/mod.rs              ← Export
3. infrastructure/repository/my_game/ ← Trait
4. infrastructure/supabase/my_game/   ← Impl
5. services/intello/my_game_ops.rs    ← Business logic
6. use_cases/intello/my_game.rs       ← Use case
7. api/dto/intello/my_game/           ← DTOs
8. api/handlers/intello/my_game.rs    ← Handler
9. api/handlers/intello/mod.rs        ← Register route
10. services/intello/types.rs         ← Add to IntelloRepositories
11. app.rs                            ← Wire repository
12. tests/intello/helpers.rs          ← Add stub repo
```

## Existing Apps

| App | Path Pattern | Description |
|-----|--------------|-------------|
| **Auth** | `*/auth/` | Login, logout |
| **Apps** | `*/apps/` | App management |
| **Collection** | `*/collection/` | DVD collection |
| **Intello** | `*/intello/` | AI-powered games |

## See Also

- [adding-features.md](architecture/adding-features.md) - Full step-by-step guide
- [end-to-end-feature.md](architecture/end-to-end-feature.md) - Complete example
- [overview.md](architecture/overview.md) - Architecture details
