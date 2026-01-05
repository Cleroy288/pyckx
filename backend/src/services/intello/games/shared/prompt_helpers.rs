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

pub fn get_language_name(code: &str) -> &'static str {
    match code.to_lowercase().as_str() {
        "en" => "English",
        "fr" => "French",
        "es" => "Spanish",
        "de" => "German",
        "it" => "Italian",
        "pt" => "Portuguese",
        "nl" => "Dutch",
        "pl" => "Polish",
        "ru" => "Russian",
        "ja" => "Japanese",
        "zh" => "Chinese (Simplified)",
        _ => "English", // Default to English
    }
}
