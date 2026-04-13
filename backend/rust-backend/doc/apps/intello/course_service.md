# Course Service

Business logic for course management and AI generation.

## 3-Stage Generation Pipeline

The backend uses a sophisticated 3-stage pipeline to generate high-quality courses:

### Stage 0: Decryption & Knowledge Expansion

1. **Intent Decryption**: Analyzing the user's prompt to understand:
   - Core intent and learning objectives
   - Target audience
   - Pedagogical goals

2. **Knowledge Expansion (Ensemble)**: Querying multiple AI models in parallel to:
   - Build a comprehensive knowledge base about the topic
   - Supplement provided documents with broader context

### Stage 1: Core Knowledge Extraction

1. Synthesizing expanded knowledge and user documents
2. Creating a coherent "Core Knowledge" summary
3. Defining key concepts and definitions for accuracy

### Stage 2: Structure Generation

1. **Dynamic Prompt Building**: Using `prompt_builder.rs` as Single Source of Truth to inject strict JSON schemas for embedded games (QCM, Flashcards, etc.)

2. **Course Construction**: Generating the final `GeneratedCourse` structure:
   - `course_metadata`: Title, description, level
   - `modules`: Ordered list containing content blocks (text, schemas, games)

## Resource Management

### Token Counting
- Each resource tracks its `token_count`
- Used for AI context window management
- Prevents exceeding model limits when combining resources

### Resource Fetching
When generating courses with `resource_ids`:
1. Fetch resources from Supabase by ID
2. Combine content respecting token limits
3. Pass to generation pipeline

## JSON Recovery Mechanism

When AI returns malformed JSON that fails to parse:

1. Capture the parsing error message
2. Send malformed JSON + error to AI for repair
3. Re-parse the fixed JSON
4. Only fail if second attempt also fails

This increases course generation reliability by allowing AI self-correction.

## Storage Interactions
- Course CRUD: Create, read, update, delete courses
- Resource management: Upload, list, delete resources per course
- Session management: Create and list study sessions per course
