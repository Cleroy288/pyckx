//! Prompt Builder - Generates AI prompts for educational game creation
//!
//! This module creates structured prompts for AI models to generate
//! educational games based on user-provided content and parameters.

use crate::services::intello::custom_question_domain::CustomQuestion;
use crate::services::intello::Level;

/// Game-specific output format instructions
pub struct GameOutputFormat {
    /// Game ID (e.g., "qcm")
    pub game_id: &'static str,
    /// Human-readable game name
    pub game_name: &'static str,
    /// Description of the game format
    pub format_description: &'static str,
    /// JSON schema example for the output
    pub json_schema: &'static str,
}

/// Available game output formats
pub const GAME_FORMATS: &[GameOutputFormat] = &[
    GameOutputFormat {
        game_id: "qcm",
        game_name: "Multiple Choice Questions (QCM)",
        format_description: r#"Create a multiple choice quiz where each question has exactly 4 answer options:
- One question text
- Exactly 3 wrong answers (plausible but incorrect)
- Exactly 1 correct answer
- An explanation of why the correct answer is right

CRITICAL - ANSWER LENGTH RULE:
All 4 answer options (1 right + 3 wrong) MUST have SIMILAR lengths.
- Users should NOT be able to guess the correct answer based on length
- If the correct answer is 5 words, wrong answers should also be around 4-6 words
- If the correct answer is a long explanation, wrong answers must also be long explanations
- AVOID patterns like: short wrong answers + long correct answer (gives it away!)

IMPORTANT: Each question MUST have exactly 4 total answer choices (1 right + 3 wrong)."#,
        json_schema: r#"{
  "questions": [
    {
      "question": "The question text goes here?",
      "wrong_answers": [
        "First incorrect option (similar length to correct answer)",
        "Second incorrect option (similar length to correct answer)",
        "Third incorrect option (similar length to correct answer)"
      ],
      "right_answer": "The correct answer (similar length to wrong answers)",
      "explanation": "Brief explanation of why this answer is correct"
    }
  ]
}

CRITICAL: The "wrong_answers" array MUST contain exactly 3 items. No more, no less.
CRITICAL: All answers MUST have similar lengths - no guessing by length allowed!"#,
    },
    GameOutputFormat {
        game_id: "open_question",
        game_name: "Open-Ended Questions",
        format_description: r#"Create open-ended questions where users write their own answers:
- One clear question text that requires a written response
- An expected answer (the ideal/correct response)
- A helpful hint to guide the user without giving away the answer

The questions should encourage critical thinking and detailed responses."#,
        json_schema: r#"{
  "questions": [
    {
      "question": "The open-ended question text goes here?",
      "expected_answer": "The ideal/correct answer that the user should provide",
      "hint": "A helpful hint to guide the user without revealing the answer"
    }
  ]
}

CRITICAL JSON RULES:
- Each object must have UNIQUE field names - NEVER duplicate any field
- Each question object must have exactly ONE "question", ONE "expected_answer", and ONE "hint" field
- Do NOT repeat the same field twice in any object
- Validate your JSON structure before outputting"#,
    },
    GameOutputFormat {
        game_id: "flashcard",
        game_name: "Flashcards",
        format_description: r#"Create flashcards for memorization and quick recall:
- Front: A term, concept, question, or prompt
- Back: The definition, answer, or explanation

Flashcards should be concise and focused on key facts, definitions, and concepts.
Each card should test ONE specific piece of knowledge."#,
        json_schema: r#"{
  "cards": [
    {
      "front": "The term, concept, or question on the front of the card",
      "back": "The definition, answer, or explanation on the back"
    }
  ]
}

IMPORTANT: Keep cards concise. Front should be a clear prompt, back should be a clear answer."#,
    },
    GameOutputFormat {
        game_id: "true_false",
        game_name: "True or False Statements",
        format_description: r#"Create true or false statements where users must determine if each statement is true or false:
- One clear statement that is either true or false
- The correct answer (true or false)
- An explanation of why the statement is true or false

Statements should be factual and based on the source material."#,
        json_schema: r#"{
  "statements": [
    {
      "statement": "A clear statement that is either true or false",
      "answer": true,
      "explanation": "Explanation of why this statement is true or false"
    }
  ]
}

IMPORTANT: The "answer" field MUST be a boolean (true or false), not a string."#,
    },
    GameOutputFormat {
        game_id: "keywords",
        game_name: "Keywords Recognition",
        format_description: r#"Create keyword recognition exercises where users identify correct keywords related to a statement:
- One statement/fact from the source material
- A list of 6-8 keyword options
- REQUIRED: Each question MUST have between 2 and 5 correct keywords (never just 1)
- The remaining keywords should be plausible but incorrect distractors
- An explanation of why the correct keywords relate to the statement

Correct keywords should be directly related to the statement's topic.
Incorrect keywords should be plausible but unrelated to the specific statement."#,
        json_schema: r#"{
  "questions": [
    {
      "statement": "A factual statement from the source material",
      "keywords": [
        {"word": "keyword1", "is_correct": true},
        {"word": "keyword2", "is_correct": true},
        {"word": "keyword3", "is_correct": false},
        {"word": "keyword4", "is_correct": true},
        {"word": "keyword5", "is_correct": false},
        {"word": "keyword6", "is_correct": false}
      ],
      "explanation": "Explanation of why the correct keywords relate to the statement"
    }
  ]
}

IMPORTANT: Each question MUST have 6-8 keywords with exactly 2-5 of them being correct (is_correct: true). NEVER have only 1 correct keyword."#,
    },
    GameOutputFormat {
        game_id: "order_phrase",
        game_name: "Order Phrase",
        format_description: r#"Create phrase ordering exercises where users arrange shuffled words into the correct order:
- One complete phrase/sentence from the source material
- The phrase split into individual words
- Each word with its correct position (0-indexed)
- An optional hint to help the user

Phrases should be:
- Meaningful sentences or phrases from the source content
- Between 4-10 words long for optimal difficulty
- Grammatically correct when properly ordered"#,
        json_schema: r#"{
  "questions": [
    {
      "original_phrase": "The complete phrase in correct order",
      "words": [
        {"word": "The", "position": 0},
        {"word": "complete", "position": 1},
        {"word": "phrase", "position": 2},
        {"word": "in", "position": 3},
        {"word": "correct", "position": 4},
        {"word": "order", "position": 5}
      ],
      "hint": "Optional hint to help the user"
    }
  ]
}

IMPORTANT: The "position" field is 0-indexed and indicates the correct position of each word. Phrases should be 4-10 words long."#,
    },
    GameOutputFormat {
        game_id: "fill_blank",
        game_name: "Fill in the Blank",
        format_description: r#"Create fill-in-the-blank exercises where users complete phrases by selecting the correct answer:
- One phrase with a blank to fill (marked with ___ or [blank])
- At least 4 answer options
- Exactly ONE correct answer
- An explanation of why the correct answer is right

Phrases should be:
- Meaningful sentences from the source content
- Clear about what type of answer is expected
- Testing important concepts or vocabulary"#,
        json_schema: r#"{
  "questions": [
    {
      "phrase": "The ___ is the powerhouse of the cell.",
      "options": [
        {"text": "mitochondria", "is_correct": true},
        {"text": "nucleus", "is_correct": false},
        {"text": "ribosome", "is_correct": false},
        {"text": "cytoplasm", "is_correct": false}
      ],
      "explanation": "The mitochondria is known as the powerhouse of the cell because it produces ATP, the cell's energy currency."
    }
  ]
}

IMPORTANT: Each question MUST have at least 4 options with EXACTLY ONE correct answer (is_correct: true). All other options must have is_correct: false."#,
    },
];

/// Get the output format for a specific game
pub fn get_game_format(game_id: &str) -> Option<&'static GameOutputFormat> {
    GAME_FORMATS.iter().find(|f| f.game_id == game_id)
}

/// Convert Level enum to detailed human-readable string with specific guidelines
pub fn level_to_string(level: &Level) -> &'static str {
    match level {
        Level::Easy => {
            r#"EASY - Beginner Level
   - Use simple, everyday vocabulary (avoid jargon and technical terms)
   - Ask about basic facts, definitions, and simple concepts
   - Questions should be straightforward with obvious correct answers
   - Focus on "what", "who", "when" type questions
   - Answers should be short and direct
   - Wrong answers should be clearly distinguishable from correct ones"#
        }
        Level::Medium => {
            r#"MEDIUM - Intermediate Level
   - Use appropriate technical vocabulary with context
   - Ask about relationships, causes, effects, and applications
   - Questions require understanding, not just memorization
   - Include "why", "how", and "explain" type questions
   - Answers may require connecting multiple concepts
   - Wrong answers should be plausible but distinguishable"#
        }
        Level::Hard => {
            r#"HARD - Advanced Level
   - Use precise technical and domain-specific terminology
   - Ask about complex relationships, analysis, and synthesis
   - Questions require deep understanding and critical thinking
   - Include scenario-based, analytical, and evaluation questions
   - Answers require integrating multiple concepts and reasoning
   - Wrong answers should be sophisticated and require careful analysis to eliminate"#
        }
    }
}

// =============================================================================
// =============================================================================
// BUILD PROMPT FUNCTIONS
// =============================================================================

/// Build the complete AI prompt from a CustomQuestion
pub fn build_prompt(custom_question: &CustomQuestion) -> String {
    let game_format = get_game_format(&custom_question.output_game).unwrap_or(&GAME_FORMATS[0]); // Default to QCM if not found

    let subjects_list = if custom_question.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        custom_question.subjects.join(", ")
    };

    // Build the documents content section
    let documents_content = custom_question
        .documents
        .iter()
        .enumerate()
        .map(|(i, doc)| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                doc.filename,
                doc.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL QUIZ GENERATION

You are an expert educational content creator. Your task is to generate high-quality quiz questions that help students learn specific subjects from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test knowledge about {subjects}
- Every question MUST be specifically about {subjects} - no generic or off-topic questions
- Extract the most important concepts, facts, and details about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

You MUST strictly follow these difficulty guidelines. Every question must match this level.

### 3. LANGUAGE
All content MUST be in {language}. This includes questions, answers, and explanations.

---

## Quiz Details

**Name**: {name}
**Description**: {description}
**Number of Questions**: {num_questions} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract information ONLY about {subjects}:

{documents}

---

## Output Format

{game_name}
{format_description}

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Question Generation Rules

1. **Subject Relevance**: Each question MUST directly test knowledge about {subjects}
   - Ask about key concepts, definitions, facts, and relationships within {subjects}
   - Do NOT create generic questions that could apply to any topic
   - Do NOT create questions about topics outside {subjects}

2. **Difficulty Consistency**: Every question must match the {level} level
   - Vocabulary, complexity, and reasoning must all align with this level
   - Do not mix easy and hard questions

3. **Source Accuracy**: All facts must come from the provided documents
   - Do not invent information not present in the source material
   - Quote or paraphrase directly from the documents

4. **Answer Quality**:
   - Correct answer must be unambiguously right based on the source
   - Wrong answers must be plausible but clearly incorrect
   - Explanations should teach why the answer is correct

5. **Coverage**: Distribute questions across different aspects of {subjects}
   - Cover various concepts, not just one narrow topic
   - Ensure comprehensive testing of the subject matter

## Final Check

Before outputting, verify:
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the {level} difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = custom_question.name,
        description = custom_question.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&custom_question.level),
        language = get_language_name(&custom_question.language),
        num_questions = custom_question.num_questions,
        instructions = if custom_question.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            custom_question.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

/// Input parameters for building an open question prompt
pub struct OpenQuestionPromptInput {
    /// Name of the question set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated questions (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the questions
    pub subjects: Vec<String>,
    /// Number of questions to generate
    pub num_questions: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating open-ended questions
///
/// This function creates a structured prompt that instructs the AI to:
/// 1. Analyze the provided source documents
/// 2. Generate the requested number of open-ended questions
/// 3. Include expected answers and helpful hints for each question
/// 4. Return the result in a specific JSON format
pub fn build_open_question_prompt(input: &OpenQuestionPromptInput) -> String {
    let game_format = get_game_format("open_question").unwrap_or(&GAME_FORMATS[1]); // Default to open_question format

    let subjects_list = if input.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        input.subjects.join(", ")
    };

    // Build the documents content section
    let documents_content = input
        .documents
        .iter()
        .enumerate()
        .map(|(i, (filename, content))| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                filename,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL OPEN-ENDED QUESTION GENERATION

You are an expert educational content creator. Your task is to generate high-quality open-ended questions that help students deeply understand specific subjects from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test deep understanding of {subjects}
- Every question MUST be specifically about {subjects} - no generic or off-topic questions
- Extract the most important concepts, theories, and details about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}
- Questions should require the student to demonstrate real understanding of {subjects}

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

You MUST strictly follow these difficulty guidelines:
- Question complexity must match this level
- Expected answer depth must match this level
- Vocabulary and terminology must match this level

### 3. LANGUAGE
All content MUST be in {language}. This includes questions, expected answers, and hints.

---

## Question Set Details

**Name**: {name}
**Description**: {description}
**Number of Questions**: {num_questions} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract information ONLY about {subjects}:

{documents}

---

## Output Format

{game_name}
{format_description}

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Question Generation Rules

1. **Subject Relevance**: Each question MUST directly test understanding of {subjects}
   - Ask about key concepts, theories, processes, and relationships within {subjects}
   - Questions should require explaining, analyzing, or applying knowledge of {subjects}
   - Do NOT create generic questions that could apply to any topic
   - Do NOT create questions about topics outside {subjects}

2. **Question Types for Open-Ended Format**:
   - "Explain..." - Ask students to explain concepts in their own words
   - "Compare and contrast..." - Ask about similarities and differences
   - "Why does..." - Ask about causes and reasoning
   - "How would you..." - Ask about application of knowledge
   - "What is the significance of..." - Ask about importance and implications
   - "Describe the process of..." - Ask about procedures and sequences

3. **Difficulty Consistency**: Every question must match the difficulty level
   - Easy: Simple explanations, basic definitions, straightforward facts
   - Medium: Connections between concepts, cause-effect relationships, applications
   - Hard: Analysis, synthesis, evaluation, complex reasoning

4. **Expected Answers**:
   - Must be accurate based on the source documents
   - Length and depth should match the difficulty level
   - Should demonstrate the understanding you're testing for

5. **Hints**:
   - Should guide thinking without giving away the answer
   - Reference relevant concepts or sections from the source
   - Help students recall what they learned about {subjects}

6. **Coverage**: Distribute questions across different aspects of {subjects}
   - Cover various concepts, not just one narrow topic
   - Ensure comprehensive testing of the subject matter

## Final Check

Before outputting, verify:
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ Expected answers are accurate and appropriately detailed
✓ Hints are helpful but don't reveal answers
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_questions = input.num_questions,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

/// Convert language code to full language name
fn get_language_name(code: &str) -> &'static str {
    match code.to_lowercase().as_str() {
        "en" => "English",
        "fr" => "French",
        "es" => "Spanish",
        "de" => "German",
        "it" => "Italian",
        "pt" => "Portuguese",
        "nl" => "Dutch",
        "ru" => "Russian",
        "zh" => "Chinese",
        "ja" => "Japanese",
        "ko" => "Korean",
        "ar" => "Arabic",
        _ => "English", // Default fallback
    }
}

// == FLASHCARD PROMPT ==

/// Input parameters for building a flashcard prompt
pub struct FlashcardPromptInput {
    /// Name of the flashcard set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated flashcards (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the flashcards
    pub subjects: Vec<String>,
    /// Number of flashcards to generate
    pub num_cards: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating flashcards
pub fn build_flashcard_prompt(input: &FlashcardPromptInput) -> String {
    let game_format = get_game_format("flashcard").unwrap_or(&GAME_FORMATS[2]);

    let subjects_list = if input.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        input.subjects.join(", ")
    };

    let documents_content = input
        .documents
        .iter()
        .enumerate()
        .map(|(i, (filename, content))| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                filename,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL FLASHCARD GENERATION

You are an expert educational content creator. Your task is to generate high-quality flashcards that help students memorize and recall key concepts from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate flashcards that DIRECTLY cover key facts, terms, and concepts about {subjects}
- Every flashcard MUST be specifically about {subjects}
- Extract the most important definitions, facts, and relationships about {subjects}
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust flashcard complexity based on this level:
- Easy: Basic definitions, simple facts, fundamental concepts
- Medium: More detailed explanations, relationships between concepts
- Hard: Complex concepts, nuanced distinctions, advanced terminology

### 3. LANGUAGE
All content MUST be in {language}. This includes both front and back of cards.

---

## Flashcard Set Details

**Name**: {name}
**Description**: {description}
**Number of Cards**: {num_cards} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract information ONLY about {subjects}:

{documents}

---

## Output Format

{game_name}
{format_description}

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Flashcard Generation Rules

1. **Subject Relevance**: Each flashcard MUST cover {subjects}
   - Focus on key terms, definitions, facts, and concepts
   - Do NOT create generic flashcards unrelated to {subjects}

2. **Card Quality**:
   - Front: Clear, concise prompt (term, question, or concept)
   - Back: Accurate, complete answer or definition
   - One concept per card - don't overload

3. **Variety**: Cover different aspects of {subjects}
   - Definitions and terminology
   - Key facts and dates
   - Relationships and processes
   - Important figures or examples

4. **Source Accuracy**: All facts must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_cards} flashcards are specifically about {subjects}
✓ All cards match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_cards = input.num_cards,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

// == ANSWER VERIFICATION PROMPT ==

/// A single answer to be graded
pub struct AnswerToGrade {
    /// Question ID
    pub question_id: String,
    /// The question text
    pub question: String,
    /// The hint that was provided
    pub hint: String,
    /// The expected/correct answer
    pub expected_answer: String,
    /// The user's submitted answer
    pub user_answer: String,
}

/// Input for building a verification prompt
pub struct VerificationPromptInput {
    /// Language for the feedback
    pub language: String,
    /// The source document content (for context)
    pub source_content: String,
    /// List of answers to grade
    pub answers: Vec<AnswerToGrade>,
}

/// Build an AI prompt for grading open question answers
///
/// This function creates a structured prompt that instructs the AI to:
/// 1. Review the source material for context
/// 2. Compare each user answer against the expected answer
/// 3. Grade each answer as "right", "medium", or "error"
/// 4. Provide feedback explaining the grade
pub fn build_verification_prompt(input: &VerificationPromptInput) -> String {
    let answers_section = input
        .answers
        .iter()
        .enumerate()
        .map(|(i, a)| {
            format!(
                r#"### Answer {} (ID: {})
**Question:** {}
**Hint provided:** {}
**Expected answer:** {}
**User's answer:** {}"#,
                i + 1,
                a.question_id,
                a.question,
                a.hint,
                a.expected_answer,
                a.user_answer
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"**Situation**
You are an educational assessment AI tasked with grading student answers to open-ended questions. You have access to the source material the questions were based on, and you must evaluate each answer fairly and constructively.

**Task**
Grade each user answer by comparing it to the expected answer and the source material. Provide a grade and constructive feedback for each answer.

**Grading Scale**
- **right**: The answer is correct or substantially correct. The user demonstrates clear understanding of the concept.
- **medium**: The answer is partially correct. The user shows some understanding but is missing key elements or has minor errors.
- **error**: The answer is incorrect or shows fundamental misunderstanding of the concept.

---

## Source Material

The following is the source content the questions were based on. Use this to verify factual accuracy:

```
{source_content}
```

---

## Answers to Grade

{answers_section}

---

## Output Format

You MUST respond with valid JSON in the following exact format:

```json
{{
  "grades": [
    {{
      "question_id": "the-question-id",
      "grade": "right" | "medium" | "error",
      "feedback": "Constructive feedback explaining why this grade was given. Be encouraging but honest."
    }}
  ]
}}
```

---

## Grading Guidelines

1. **Be fair and consistent** - Apply the same standards to all answers
2. **Consider partial credit** - Use "medium" for answers that show understanding but are incomplete
3. **Focus on understanding** - Grade based on conceptual understanding, not exact wording
4. **Be constructive** - Feedback should help the student learn, not just criticize
5. **Reference the source** - Base your grading on the provided source material
6. **Use {language}** - All feedback must be written in {language}

## Quality Standards for Feedback

- Keep feedback concise but helpful (2-3 sentences)
- Explain what was correct (if anything)
- Explain what was missing or incorrect
- Suggest how to improve (for medium/error grades)
- Be encouraging and educational

**IMPORTANT**: Respond ONLY with the JSON output. Do not include any additional text, explanations, or markdown formatting outside the JSON structure."#,
        source_content = input.source_content,
        answers_section = answers_section,
        language = get_language_name(&input.language),
    )
}

// == TRUE OR FALSE PROMPT ==

/// Input parameters for building a true/false prompt
pub struct TrueOrFalsePromptInput {
    /// Name of the true/false set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated statements (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the statements
    pub subjects: Vec<String>,
    /// Number of statements to generate
    pub num_statements: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating true/false statements
pub fn build_true_false_prompt(input: &TrueOrFalsePromptInput) -> String {
    let game_format = get_game_format("true_false").unwrap_or(&GAME_FORMATS[3]);

    let subjects_list = if input.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        input.subjects.join(", ")
    };

    let documents_content = input
        .documents
        .iter()
        .enumerate()
        .map(|(i, (filename, content))| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                filename,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL TRUE OR FALSE STATEMENT GENERATION

You are an expert educational content creator. Your task is to generate high-quality true or false statements that help students test their understanding of key concepts from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate statements that DIRECTLY test knowledge about {subjects}
- Every statement MUST be specifically about {subjects}
- Extract the most important facts and concepts about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}
- Create a mix of TRUE and FALSE statements (approximately 50/50)

### 2. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust statement complexity based on this level:
- Easy: Basic facts, simple definitions, fundamental concepts
- Medium: More nuanced facts, relationships between concepts, require understanding
- Hard: Complex concepts, subtle distinctions, require careful analysis

### 3. LANGUAGE
All content MUST be in {language}. This includes statements and explanations.

---

## Statement Set Details

**Name**: {name}
**Description**: {description}
**Number of Statements**: {num_statements} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract information ONLY about {subjects}:

{documents}

---

## Output Format

{game_name}
{format_description}

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Statement Generation Rules

1. **Subject Relevance**: Each statement MUST test knowledge about {subjects}
   - Focus on key facts, definitions, and concepts
   - Do NOT create generic statements unrelated to {subjects}

2. **Balance True and False**: Create roughly equal numbers of true and false statements
   - Do not make all statements one type
   - False statements should be plausible but incorrect

3. **Statement Quality**:
   - Statements should be clear and unambiguous
   - Avoid trick questions or misleading wording
   - Each statement should test ONE concept

4. **Explanations**:
   - Explain WHY the statement is true or false
   - Reference the source material when appropriate
   - Be educational and helpful

5. **Source Accuracy**: All facts must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_statements} statements are specifically about {subjects}
✓ All statements match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ There is a good mix of true and false statements
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_statements = input.num_statements,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

// == KEYWORDS PROMPT ==

/// Input parameters for building a keywords prompt
pub struct KeywordsPromptInput {
    /// Name of the keywords set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated content (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the questions
    pub subjects: Vec<String>,
    /// Number of questions to generate
    pub num_questions: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating keywords questions
pub fn build_keywords_prompt(input: &KeywordsPromptInput) -> String {
    let game_format = get_game_format("keywords").unwrap_or(&GAME_FORMATS[4]);

    let subjects_list = if input.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        input.subjects.join(", ")
    };

    let documents_content = input
        .documents
        .iter()
        .enumerate()
        .map(|(i, (filename, content))| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                filename,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL KEYWORDS RECOGNITION GENERATION

You are an expert educational content creator. Your task is to generate high-quality keyword recognition exercises that help students identify key concepts and vocabulary from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test knowledge about {subjects}
- Every statement MUST be specifically about {subjects}
- Extract the most important concepts and vocabulary about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. Output Format: {game_name}

{format_description}

### 3. Key Parameters
- **Set Name**: {name}
- **Description**: {description}
- **Difficulty Level**: {level}
- **Language**: Generate ALL content in {language}
- **Number of Questions**: Generate exactly {num_questions} keyword questions
- **Additional Instructions**: {instructions}

---

## SOURCE DOCUMENTS

{documents}

---

## OUTPUT FORMAT (JSON ONLY)

You MUST output VALID JSON matching this exact structure:

```json
{json_schema}
```

---

## Keyword Generation Rules

1. **Subject Relevance**: Each statement MUST test knowledge about {subjects}
   - Focus on key facts, definitions, and concepts
   - Do NOT create generic questions unrelated to {subjects}

2. **Keyword Balance**: Each question should have 6-8 keywords
   - About 60% correct (related to the statement)
   - About 40% incorrect (plausible but unrelated distractors)

3. **Keyword Quality**:
   - Correct keywords should be directly related to the statement's topic
   - Incorrect keywords should be plausible terms from the same domain but unrelated to this specific statement
   - Avoid obviously wrong keywords that are easy to eliminate

4. **Explanations**:
   - Explain WHY the correct keywords relate to the statement
   - Reference the source material when appropriate
   - Be educational and helpful

5. **Source Accuracy**: All statements and keywords must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All facts are from the source documents
✓ Each question has 6-8 keywords with a good mix of correct and incorrect
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_questions = input.num_questions,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

// == ORDER PHRASE PROMPT ==

/// Input parameters for building an order phrase prompt
pub struct OrderPhrasePromptInput {
    /// Name of the order phrase set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated content (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the questions
    pub subjects: Vec<String>,
    /// Number of questions to generate
    pub num_questions: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating order phrase questions
pub fn build_order_phrase_prompt(input: &OrderPhrasePromptInput) -> String {
    let game_format = get_game_format("order_phrase").unwrap_or(&GAME_FORMATS[5]);

    let subjects_list = if input.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        input.subjects.join(", ")
    };

    let documents_content = input
        .documents
        .iter()
        .enumerate()
        .map(|(i, (filename, content))| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                filename,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL ORDER PHRASE GENERATION

You are an expert educational content creator. Your task is to generate high-quality phrase ordering exercises that help students learn key concepts and improve language comprehension from their study materials.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate phrases that DIRECTLY relate to {subjects}
- Every phrase MUST be specifically about {subjects}
- Extract meaningful sentences and key phrases about {subjects} from the documents
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. Output Format: {game_name}

{format_description}

### 3. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust phrase complexity based on this level:
- Easy: Short phrases (4-6 words), simple vocabulary, basic sentence structure
- Medium: Medium phrases (5-8 words), moderate vocabulary, standard sentences
- Hard: Longer phrases (7-10 words), complex vocabulary, sophisticated structures

### 4. LANGUAGE
All content MUST be in {language}. This includes phrases and hints.

---

## Phrase Set Details

**Name**: {name}
**Description**: {description}
**Number of Phrases**: {num_questions} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract phrases ONLY about {subjects}:

{documents}

---

## Output Format

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Phrase Generation Rules

1. **Subject Relevance**: Each phrase MUST be about {subjects}
   - Use key definitions, facts, and concepts from the source
   - Do NOT create generic phrases unrelated to {subjects}

2. **Phrase Quality**:
   - Phrases should be grammatically correct
   - Each phrase should be meaningful on its own
   - Avoid fragments or incomplete thoughts
   - Prefer impactful or memorable phrases from the source

3. **Word Splitting**:
   - Split the phrase into individual words
   - Each word gets a position (0-indexed)
   - Include punctuation attached to words (e.g., "world!" not "world" + "!")

4. **Hints**:
   - Provide a helpful hint that guides without giving away the order
   - Reference the topic or first word if helpful

5. **Source Accuracy**: All phrases must come from or be based on the provided documents

## Final Check

Before outputting, verify:
✓ All {num_questions} phrases are specifically about {subjects}
✓ All phrases match the difficulty level (word count, complexity)
✓ All content is in {language}
✓ All phrases are from the source documents
✓ Positions are correctly 0-indexed
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_questions = input.num_questions,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}

// == FILL BLANK PROMPT ==

/// Input parameters for building a fill blank prompt
pub struct FillBlankPromptInput {
    /// Name of the fill blank set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated content (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the questions
    pub subjects: Vec<String>,
    /// Number of questions to generate
    pub num_questions: u8,
    /// Document contents (filename, content pairs)
    pub documents: Vec<(String, String)>,
}

/// Build an AI prompt for generating fill in the blank questions
pub fn build_fill_blank_prompt(input: &FillBlankPromptInput) -> String {
    let game_format = get_game_format("fill_blank").unwrap_or(&GAME_FORMATS[6]);

    let subjects_list = if input.subjects.is_empty() {
        "General topics from the provided content".to_string()
    } else {
        input.subjects.join(", ")
    };

    let documents_content = input
        .documents
        .iter()
        .enumerate()
        .map(|(i, (filename, content))| {
            format!(
                "### Document {} - {}\n```\n{}\n```",
                i + 1,
                filename,
                content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!(
        r#"# EDUCATIONAL FILL IN THE BLANK GENERATION

You are an expert educational content creator. Your task is to generate high-quality fill-in-the-blank exercises that help students test their knowledge by completing phrases with the correct answer from multiple options.

## CRITICAL REQUIREMENTS

### 1. SUBJECT FOCUS (MOST IMPORTANT)
The user wants to learn about: **{subjects}**

You MUST:
- Generate questions that DIRECTLY test knowledge about {subjects}
- Every phrase MUST be specifically about {subjects}
- Extract the most important concepts, facts, and terminology about {subjects}
- If the documents contain information about multiple topics, ONLY use content related to {subjects}

### 2. Output Format: {game_name}

{format_description}

### 3. DIFFICULTY LEVEL (STRICTLY ENFORCE)
{level}

Adjust complexity based on this level:
- Easy: Simple vocabulary, basic concepts, obvious correct answers
- Medium: Technical terms, concepts requiring understanding, plausible distractors
- Hard: Advanced vocabulary, nuanced concepts, very plausible distractors

### 4. LANGUAGE
All content MUST be in {language}. This includes phrases, options, and explanations.

---

## Question Set Details

**Name**: {name}
**Description**: {description}
**Number of Questions**: {num_questions} (generate EXACTLY this many)

**User Instructions**: {instructions}

---

## Source Documents

Study these documents and extract content ONLY about {subjects}:

{documents}

---

## Output Format

Respond with ONLY valid JSON:

```json
{json_schema}
```

---

## Question Generation Rules

1. **Subject Relevance**: Each phrase MUST test knowledge about {subjects}
   - Use key definitions, facts, concepts, and vocabulary
   - Do NOT create generic phrases unrelated to {subjects}

2. **Phrase Quality**:
   - Each phrase should have ONE clear blank (marked with ___)
   - The blank should be for an important term or concept
   - The phrase should be grammatically correct when completed

3. **Answer Options**:
   - Provide at least 4 options per question
   - EXACTLY ONE option must be correct (is_correct: true)
   - Wrong options should be plausible distractors from the same domain
   - Avoid obviously wrong answers

4. **Explanations**:
   - Explain WHY the correct answer is right
   - Reference the source material when appropriate
   - Be educational and helpful

5. **Source Accuracy**: All phrases and answers must come from the provided documents

## Final Check

Before outputting, verify:
✓ All {num_questions} questions are specifically about {subjects}
✓ All questions match the difficulty level
✓ All content is in {language}
✓ All phrases are from the source documents
✓ Each question has at least 4 options with EXACTLY ONE correct
✓ JSON format is valid

**OUTPUT ONLY THE JSON. No other text.**"#,
        name = input.name,
        description = input.description,
        game_name = game_format.game_name,
        format_description = game_format.format_description,
        subjects = subjects_list,
        level = level_to_string(&input.level),
        language = get_language_name(&input.language),
        num_questions = input.num_questions,
        instructions = if input.instructions.is_empty() {
            "No additional instructions provided.".to_string()
        } else {
            input.instructions.clone()
        },
        documents = documents_content,
        json_schema = game_format.json_schema,
    )
}
