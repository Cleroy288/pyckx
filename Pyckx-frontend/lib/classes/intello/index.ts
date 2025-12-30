// == Intello Module - Main entry point ==

// Main app class
export { IntelloApp } from "./IntelloApp";

// Entity classes
export { QcmSet } from "./QcmSet";
export { QcmQuestion } from "./QcmQuestion";
export { OpenQuestionSet } from "./OpenQuestionSet";
export { OpenQuestion } from "./OpenQuestion";
export { TrueOrFalseSet } from "./TrueOrFalseSet";
export { TrueOrFalseStatement } from "./TrueOrFalseStatement";
export { FlashcardSet } from "./FlashcardSet";
export { Flashcard } from "./Flashcard";
export { KeywordSet } from "./KeywordSet";
export { KeywordQuestion } from "./KeywordQuestion";
export { Keyword } from "./Keyword";

// List class
export { QcmSetList } from "./QcmSetList";
export type { QcmSetFilterField, SortOrder } from "./QcmSetList";

// Types
export type {
  Level,
  // QCM types
  QcmQuestionData,
  CreateQcmQuestionInput,
  QcmSetData,
  CreateQcmSetInput,
  UpdateQcmSetInput,
  QcmSetSuccessResponse,
  QcmSetListResponse,
  // Open Question types
  OpenQuestionData,
  CreateOpenQuestionInput,
  OpenQuestionSetData,
  CreateOpenQuestionSetInput,
  UpdateOpenQuestionSetInput,
  OpenQuestionSetSuccessResponse,
  OpenQuestionSetListResponse,
  // True or False types
  TrueOrFalseStatementData,
  TrueOrFalseSetData,
  TrueOrFalseSetListResponse,
  // Flashcard types
  FlashcardData,
  FlashcardSetData,
  FlashcardSetListResponse,
  // Keyword types
  KeywordData,
  KeywordQuestionData,
  KeywordSetData,
  KeywordSetListResponse,
  // Other
  AvailableGamesResponse,
} from "./types";

// Validation constants
export { MAX_SUBJECTS, MAX_SUBJECT_LENGTH } from "./types";

// Course types
export type { StartSessionInput } from "./course";
