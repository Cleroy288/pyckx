# Intello App

AI-powered learning games generated from your documents.

## Documentation Index

### QCM (Multiple Choice)
- [qcm_domain.md](qcm_domain.md) - Data structures, validation rules
- [qcm_service.md](qcm_service.md) - Business logic (manual vs AI creation)

### Open Questions
- [open_question_domain.md](open_question_domain.md) - Data structures
- [open_question_service.md](open_question_service.md) - AI grading logic, caching

### Flashcards
- [flashcard_domain.md](flashcard_domain.md) - Data structures
- [flashcard_service.md](flashcard_service.md) - Generation logic

### True/False
- [true_false_domain.md](true_false_domain.md) - Data structures
- [true_false_service.md](true_false_service.md) - Generation logic

### Keywords
- [keywords_domain.md](keywords_domain.md) - Data structures
- [keywords_service.md](keywords_service.md) - Generation logic

### Order Phrase
- [order_phrase_domain.md](order_phrase_domain.md) - Data structures
- [order_phrase_service.md](order_phrase_service.md) - Game logic (shuffling, positions)

### Fill Blank
- [fill_blank_domain.md](fill_blank_domain.md) - Data structures
- [fill_blank_service.md](fill_blank_service.md) - Game logic (word removal, options)

### Courses
- [course_domain.md](course_domain.md) - Course, CourseResource, UserResource
- [course_service.md](course_service.md) - 3-Stage Generation Pipeline, JSON Recovery

### AI Usage
- [ai_usage_domain.md](ai_usage_domain.md) - Feature enums, cost calculation
- [ai_usage_service.md](ai_usage_service.md) - Logging mechanism

---

## API Endpoints

HTTP endpoint documentation (routes, methods, JSON schemas) is located in the handlers layer:
- `src/http_api/handlers/intello/`

## Related Documentation

- [AI Models](ai-models.md) - Available AI models
- [OpenRouter Service](openrouter.md) - AI generation architecture

