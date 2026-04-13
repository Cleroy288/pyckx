# Collection App

Manage personal DVD collections with Supabase storage.

## API Endpoints

Base URL: `/app/collection`

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/dvds` | Add DVD |
| `GET` | `/dvds` | List user's DVDs |
| `GET` | `/dvds/{id}` | Get DVD by ID |
| `PUT` | `/dvds/{id}` | Update DVD |
| `DELETE` | `/dvds/{id}` | Delete DVD |

## Data Model

```rust
Dvd {
    id: String,           // UUID
    name: String,         // Unique per user
    description: String,
    release_date: String, // YYYY-MM-DD
    actors: Vec<String>,
    directors: Vec<String>,
    created_at: i64,      // Unix timestamp
    updated_at: i64,
}
```

## Request Examples

### Add DVD
```json
POST /app/collection/dvds
{
  "name": "Inception",
  "description": "A mind-bending thriller",
  "release_date": "2010-07-16",
  "actors": ["Leonardo DiCaprio", "Ellen Page"],
  "directors": ["Christopher Nolan"]
}
```

### Response
```json
{
  "message": "DVD added successfully",
  "dvd": { "id": "uuid", "name": "Inception", ... }
}
```

## Business Rules

1. **Unique names** - DVD names must be unique per user (case-insensitive)
2. **User isolation** - Users only see their own DVDs
3. **Timestamps** - `created_at` set on add, `updated_at` on every update

## Error Codes

| Code | HTTP | Description |
|------|------|-------------|
| `COLLECTION_DVD_NOT_FOUND` | 404 | DVD doesn't exist |
| `COLLECTION_DVD_DUPLICATE` | 409 | Name already exists |
| `SESSION_NOT_FOUND` | 401 | Not authenticated |

## File Structure

src/
├── services/collection/              # Service Layer
│   ├── dvd_domain.rs                 # DVD Entity
│   ├── dvd_service.rs                # DVD Business Logic
│   ├── collection_domain.rs          # Collection Entity
│   ├── collection_service.rs         # Collection Business Logic
│   └── error_domain.rs               # Error types
├── infra/
│   ├── database/dvd/trait.rs         # Repository Trait
│   └── supabase/dvd/                 # Supabase Implementation
└── http_api/handlers/collection/     # HTTP handlers
