// == Intello Types - Shared types for Intello module ==

// == Validation Constants ==
export const MAX_SUBJECTS = 3;
export const MAX_SUBJECT_LENGTH = 20;

// == Level Enum ==
export type Level = "easy" | "medium" | "hard";

// == QcmQuestion Types ==
export interface QcmQuestionData {
  id: string;
  question: string;
  wrong_answers: string[];
  right_answer: string;
  explanation: string;
}

export interface CreateQcmQuestionInput {
  question: string;
  wrong_answers: string[];
  right_answer: string;
  explanation: string;
}

// == QcmSet Types ==
export interface QcmSetData {
  id: string;
  user_id: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  questions: QcmQuestionData[];
}

export interface CreateQcmSetInput {
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects?: string[];
  questions?: CreateQcmQuestionInput[];
}

export interface UpdateQcmSetInput {
  name?: string;
  description?: string;
  level?: Level;
  language?: string;
  subjects?: string[];
  questions?: CreateQcmQuestionInput[];
}

// == OpenQuestion Types ==
export interface OpenQuestionData {
  id: string;
  question: string;
  user_answer: string;
  expected_answer?: string;
  hint?: string;
}

export interface CreateOpenQuestionInput {
  question: string;
  user_answer?: string;
  expected_answer?: string;
  hint?: string;
}

// == OpenQuestionSet Types ==
export interface OpenQuestionSetData {
  id: string;
  user_id: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  questions: OpenQuestionData[];
}

export interface CreateOpenQuestionSetInput {
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects?: string[];
  questions?: CreateOpenQuestionInput[];
}

export interface UpdateOpenQuestionSetInput {
  name?: string;
  description?: string;
  level?: Level;
  language?: string;
  subjects?: string[];
  questions?: CreateOpenQuestionInput[];
}

// == API Response Types ==
export interface QcmSetSuccessResponse {
  success: boolean;
  message: string;
  set?: QcmSetData;
}

export interface QcmSetListResponse {
  sets: QcmSetData[];
  count: number;
}

export interface OpenQuestionSetSuccessResponse {
  success: boolean;
  message: string;
  set?: OpenQuestionSetData;
}

export interface OpenQuestionSetListResponse {
  sets: OpenQuestionSetData[];
  count: number;
}

export interface AvailableGamesResponse {
  games: string[];
  count: number;
}

// == TrueOrFalseStatement Types ==
export interface TrueOrFalseStatementData {
  id: string;
  statement: string;
  answer: boolean;
  explanation: string;
}

// == TrueOrFalseSet Types ==
export interface TrueOrFalseSetData {
  id: string;
  user_id: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  statements: TrueOrFalseStatementData[];
}

export interface TrueOrFalseSetListResponse {
  sets: TrueOrFalseSetData[];
  count: number;
}

// == Flashcard Types ==
export interface FlashcardData {
  id: string;
  front: string;
  back: string;
}

// == FlashcardSet Types ==
export interface FlashcardSetData {
  id: string;
  user_id: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  cards: FlashcardData[];
}

export interface FlashcardSetListResponse {
  sets: FlashcardSetData[];
  count: number;
}

// == Keyword Types ==
export interface KeywordData {
  id: string;
  word: string;
  is_correct: boolean;
}

// == KeywordQuestion Types ==
export interface KeywordQuestionData {
  id: string;
  statement: string;
  keywords: KeywordData[];
  explanation: string;
}

// == KeywordSet Types ==
export interface KeywordSetData {
  id: string;
  user_id?: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  questions: KeywordQuestionData[];
}

export interface KeywordSetListResponse {
  sets: KeywordSetData[];
  count: number;
}
