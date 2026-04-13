# Course Domain

Course organization and resource management data structures.

## Data Model

```rust
Course {
    id: Uuid,
    user_id: Uuid,
    name: String,
    description: String,
    created_at: DateTime,
    updated_at: DateTime
}

CourseResource {
    id: Uuid,
    course_id: Uuid,
    filename: String,
    content: String,
    token_count: i32,
    created_at: DateTime
}

UserResource {
    id: Uuid,
    user_id: Uuid,
    filename: String,
    content: String,
    token_count: i32,
    created_at: DateTime
}
```

## Validation Rules

### Course
- `name`: required, non-empty
- `description`: optional context for the course

### CourseResource
- `filename`: required, original file name
- `content`: extracted text content
- `token_count`: number of tokens for AI context management

### UserResource
- Standalone resources not tied to a specific course
- Same validation as CourseResource
