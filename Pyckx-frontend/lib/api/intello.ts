// == Intello API - Functions for QCM set management // ==

import { endpoints } from "./config";
import type { BackendErrorResponse } from "./types";
import {
  MAX_SUBJECTS,
  MAX_SUBJECT_LENGTH,
  type QcmSetData,
  type CreateQcmSetInput,
  type UpdateQcmSetInput,
  type QcmSetListResponse,
  type QcmSetSuccessResponse,
} from "@/lib/classes/intello";

// == Game Types // ==

/** Game data returned by backend */
export interface GameData {
  id: string;
  name: string;
  description: string;
}

/** Response for available games */
export interface AvailableGamesResponse {
  games: GameData[];
  count: number;
}

// == Helper to extract error message from backend response // ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") {
    return fallback;
  }

  const backendError = errorBody as BackendErrorResponse;
  if (backendError.message) {
    return backendError.message;
  }

  return fallback;
}

// == Subject Validation // ==
export function validateSubjects(subjects: string[] | undefined): void {
  if (!subjects) return;

  if (subjects.length > MAX_SUBJECTS) {
    throw new Error(`Too many subjects: ${subjects.length}. Maximum allowed is ${MAX_SUBJECTS}`);
  }

  for (let i = 0; i < subjects.length; i++) {
    if (subjects[i].length > MAX_SUBJECT_LENGTH) {
      throw new Error(
        `Subject ${i + 1} is too long (${subjects[i].length} chars). Maximum is ${MAX_SUBJECT_LENGTH} characters`
      );
    }
  }
}

// == Get All User QCM Sets // ==
export async function getAllQcmSets(): Promise<QcmSetData[]> {
  const res = await fetch(endpoints.intello.qcm(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch QCM sets (${res.status})`));
  }

  const data: QcmSetListResponse = await res.json();
  return data.sets;
}

// == Add QCM Set // ==
export async function addQcmSet(input: CreateQcmSetInput): Promise<QcmSetData> {
  // Validate subjects before sending
  validateSubjects(input.subjects);

  const res = await fetch(endpoints.intello.qcm(), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify(input),
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create QCM set (${res.status})`));
  }

  const data: QcmSetSuccessResponse = await res.json();
  if (!data.set) {
    throw new Error("No set returned from API");
  }
  return data.set;
}

// == Modify QCM Set // ==
export async function modifyQcmSet(id: string, input: UpdateQcmSetInput): Promise<QcmSetData> {
  // Validate subjects before sending
  validateSubjects(input.subjects);

  const res = await fetch(endpoints.intello.qcmById(id), {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify(input),
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to update QCM set (${res.status})`));
  }

  const data: QcmSetSuccessResponse = await res.json();
  if (!data.set) {
    throw new Error("No set returned from API");
  }
  return data.set;
}

// == Delete QCM Set // ==
export async function deleteQcmSet(id: string): Promise<void> {
  const res = await fetch(endpoints.intello.qcmById(id), {
    method: "DELETE",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to delete QCM set (${res.status})`));
  }
}

// == Get Available Games // ==
export async function getAvailableGames(): Promise<GameData[]> {
  const res = await fetch(endpoints.intello.games(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch available games (${res.status})`));
  }

  const data: AvailableGamesResponse = await res.json();
  return data.games;
}


// == Custom Question Types // ==

/** Difficulty level for questions */
export type Level = "easy" | "medium" | "hard";

/** Valid number of questions options */
export const VALID_NUM_QUESTIONS = [5, 10, 15, 20, 25, 30] as const;
export type NumQuestions = typeof VALID_NUM_QUESTIONS[number];

/** AI model string type */
export type AIModel = string;

/** Response from available models endpoint */
export interface AvailableModelsResponse {
  models: string[];
  default_model: string;
}

// == Get Available AI Models ==
export async function getAvailableModels(): Promise<AvailableModelsResponse> {
  const res = await fetch(endpoints.intello.models(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch available models (${res.status})`));
  }

  return await res.json();
}

/** Input for creating a custom question */
export interface CreateCustomQuestionInput {
  /** Name of the custom question set */
  name: string;
  /** Description of what this generates */
  description: string;
  /** Specific instructions for the AI model */
  instructions: string;
  /** Language for generated questions (e.g., "en", "fr") */
  language: string;
  /** Difficulty level */
  level: Level;
  /** Output game ID (references AVAILABLE_GAMES, e.g., "qcm") */
  output_game: string;
  /** Subjects/topics for the questions (max 3) */
  subjects: string[];
  /** Number of questions to generate (5, 10, 15, 20, 25, or 30) */
  num_questions: NumQuestions;
  /** AI model to use (optional - uses default if not specified) */
  model?: AIModel;
}

/** A single QCM question */
export interface QcmQuestion {
  id: string;
  question: string;
  wrong_answers: string[];
  right_answer: string;
  explanation: string;
}

/** Response from custom question creation */
export interface CustomQuestionResponse {
  success: boolean;
  message: string;
  id: string;
  total_token_count: number;
  documents_processed: number;
  questions: QcmQuestion[];
}

// == Create Custom Question // ==
/**
 * Create a custom question configuration with uploaded documents.
 * Sends multipart form data with JSON metadata and file uploads.
 *
 * @param input - The custom question metadata
 * @param files - Array of files to upload (PDF, Word, PowerPoint, TXT)
 * @returns Response with created question info
 */
export async function createCustomQuestion(
  input: CreateCustomQuestionInput,
  files: File[]
): Promise<CustomQuestionResponse> {
  // Validate subjects before sending
  validateSubjects(input.subjects);

  // Build multipart form data
  const formData = new FormData();

  // Add metadata as JSON
  formData.append("metadata", JSON.stringify(input));

  // Add files
  for (const file of files) {
    formData.append("files", file);
  }

  const res = await fetch(endpoints.intello.qcmGenerate(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create custom question (${res.status})`));
  }

  return await res.json();
}


// == Open Question Types ==

/** A single open question */
export interface OpenQuestion {
  id: string;
  question: string;
  user_answer: string;
  expected_answer?: string;
  hint?: string;
}

/** Input for creating open questions */
export interface CreateOpenQuestionInput {
  name: string;
  description: string;
  instructions: string;
  language: string;
  level: Level;
  subjects: string[];
  num_questions: NumQuestions;
  /** AI model to use (optional - uses default if not specified) */
  model?: AIModel;
}

/** Response from open question creation */
export interface CreateOpenQuestionResponse {
  success: boolean;
  message: string;
  id: string;
  total_token_count: number;
  documents_processed: number;
  questions: OpenQuestion[];
}

/** Open question set data */
export interface OpenQuestionSetData {
  id: string;
  user_id: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  questions: OpenQuestion[];
}

/** Response for listing open question sets */
export interface OpenQuestionSetListResponse {
  sets: OpenQuestionSetData[];
  count: number;
}

/** User answer for grading */
export interface UserAnswer {
  question_id: string;
  user_answer: string;
}

/** Request to check/grade answers */
export interface CheckAnswersRequest {
  set_id: string;
  answers: UserAnswer[];
}

/** Grade level for an answer */
export type GradeLevel = "right" | "medium" | "error";

/** Graded answer response */
export interface GradedAnswer {
  question_id: string;
  grade: GradeLevel;
  feedback: string;
}

/** Response from checking answers */
export interface CheckAnswersResponse {
  success: boolean;
  message: string;
  set_id: string;
  grades: GradedAnswer[];
}

// == Create Open Questions ==
/**
 * Create open questions from uploaded documents.
 * Sends multipart form data with JSON metadata and file uploads.
 */
export async function createOpenQuestions(
  input: CreateOpenQuestionInput,
  files: File[]
): Promise<CreateOpenQuestionResponse> {
  validateSubjects(input.subjects);

  const formData = new FormData();
  formData.append("metadata", JSON.stringify(input));

  for (const file of files) {
    formData.append("files", file);
  }

  const res = await fetch(endpoints.intello.openQuestions(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create open questions (${res.status})`));
  }

  return await res.json();
}

// == Get All User Open Question Sets ==
export async function getAllOpenQuestionSets(): Promise<OpenQuestionSetData[]> {
  const res = await fetch(endpoints.intello.openQuestions(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch open question sets (${res.status})`));
  }

  const data: OpenQuestionSetListResponse = await res.json();
  return data.sets;
}

// == Check/Grade Open Question Answers ==
export async function checkOpenQuestionAnswers(
  setId: string,
  answers: UserAnswer[]
): Promise<CheckAnswersResponse> {
  const res = await fetch(endpoints.intello.openQuestionsCheck(), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify({ set_id: setId, answers }),
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to check answers (${res.status})`));
  }

  return await res.json();
}


// == Flashcard Types ==

/** A single flashcard */
export interface Flashcard {
  id: string;
  front: string;
  back: string;
}

/** Input for creating flashcards */
export interface CreateFlashcardInput {
  name: string;
  description: string;
  instructions: string;
  language: string;
  level: Level;
  subjects: string[];
  num_questions: NumQuestions;
  /** AI model to use (optional - uses default if not specified) */
  model?: AIModel;
}

/** Response from flashcard creation */
export interface CreateFlashcardResponse {
  success: boolean;
  message: string;
  id: string;
  total_token_count: number;
  documents_processed: number;
  cards: Flashcard[];
}

/** Flashcard set data */
export interface FlashcardSetData {
  id: string;
  user_id: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  cards: Flashcard[];
}

/** Response for listing flashcard sets */
export interface FlashcardSetListResponse {
  sets: FlashcardSetData[];
  count: number;
}

// == Create Flashcards ==
/**
 * Create flashcards from uploaded documents.
 * Sends multipart form data with JSON metadata and file uploads.
 */
export async function createFlashcards(
  input: CreateFlashcardInput,
  files: File[]
): Promise<CreateFlashcardResponse> {
  validateSubjects(input.subjects);

  const formData = new FormData();
  formData.append("metadata", JSON.stringify(input));

  for (const file of files) {
    formData.append("files", file);
  }

  const res = await fetch(endpoints.intello.flashcards(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create flashcards (${res.status})`));
  }

  return await res.json();
}

// == Get All User Flashcard Sets ==
export async function getAllFlashcardSets(): Promise<FlashcardSetData[]> {
  const res = await fetch(endpoints.intello.flashcards(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch flashcard sets (${res.status})`));
  }

  const data: FlashcardSetListResponse = await res.json();
  return data.sets;
}


// == True or False Types ==

/** A single true/false statement */
export interface TrueOrFalseStatement {
  id: string;
  statement: string;
  answer: boolean;
  explanation: string;
}

/** Input for creating true/false statements */
export interface CreateTrueOrFalseInput {
  name: string;
  description: string;
  instructions: string;
  language: string;
  level: Level;
  subjects: string[];
  num_questions: NumQuestions;
  /** AI model to use (optional - uses default if not specified) */
  model?: AIModel;
}

/** Response from true/false creation */
export interface CreateTrueOrFalseResponse {
  success: boolean;
  message: string;
  id: string;
  total_token_count: number;
  documents_processed: number;
  statements: TrueOrFalseStatement[];
}

/** True or False set data (with full statements for playing) */
export interface TrueOrFalseSetData {
  id: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  statements: TrueOrFalseStatement[];
}

/** Response for listing true/false sets */
export interface TrueOrFalseSetListResponse {
  sets: TrueOrFalseSetData[];
  count: number;
}

// == Create True or False ==
/**
 * Create true/false statements from uploaded documents.
 * Sends multipart form data with JSON metadata and file uploads.
 */
export async function createTrueOrFalse(
  input: CreateTrueOrFalseInput,
  files: File[]
): Promise<CreateTrueOrFalseResponse> {
  validateSubjects(input.subjects);

  const formData = new FormData();
  formData.append("metadata", JSON.stringify(input));

  for (const file of files) {
    formData.append("files", file);
  }

  const res = await fetch(endpoints.intello.trueFalse(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create true/false statements (${res.status})`));
  }

  return await res.json();
}

// == Get All User True or False Sets ==
export async function getAllTrueOrFalseSets(): Promise<TrueOrFalseSetData[]> {
  const res = await fetch(endpoints.intello.trueFalse(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch true/false sets (${res.status})`));
  }

  const data: TrueOrFalseSetListResponse = await res.json();
  return data.sets;
}

// == Keyword Types ==
export interface KeywordData {
  id: string;
  word: string;
  is_correct: boolean;
}

export interface KeywordQuestionData {
  id: string;
  statement: string;
  keywords: KeywordData[];
  explanation: string;
}

export interface KeywordSetData {
  id: string;
  user_id?: string;
  name: string;
  description: string;
  level: string;
  language: string;
  subjects: string[];
  questions: KeywordQuestionData[];
}

export interface KeywordSetListResponse {
  sets: KeywordSetData[];
  count: number;
}

// == Create Keywords Input/Response ==
export interface CreateKeywordsInput {
  name: string;
  description: string;
  instructions: string;
  language: string;
  level: string;
  subjects: string[];
  num_questions: number;
  model?: string;
}

export interface CreateKeywordsResponse {
  success: boolean;
  message: string;
  id: string;
  total_token_count: number;
  documents_processed: number;
  questions: KeywordQuestionData[];
}

// == Create Keywords ==
/**
 * Create keyword questions from uploaded documents.
 * Sends multipart form data with JSON metadata and file uploads.
 */
export async function createKeywords(
  input: CreateKeywordsInput,
  files: File[]
): Promise<CreateKeywordsResponse> {
  validateSubjects(input.subjects);

  const formData = new FormData();
  formData.append("metadata", JSON.stringify(input));

  for (const file of files) {
    formData.append("files", file);
  }

  const res = await fetch(endpoints.intello.keywords(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create keyword questions (${res.status})`));
  }

  return await res.json();
}

// == Get All User Keyword Sets ==
export async function getAllKeywordSets(): Promise<KeywordSetData[]> {
  const res = await fetch(endpoints.intello.keywords(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch keyword sets (${res.status})`));
  }

  const data: KeywordSetListResponse = await res.json();
  return data.sets;
}

// == Order Phrase Types ==
export interface OrderPhraseWordData {
  id: string;
  word: string;
  position: number;
}

export interface OrderPhraseQuestionData {
  id: string;
  original_phrase: string;
  words: OrderPhraseWordData[];
  hint: string;
}

export interface OrderPhraseSetData {
  id: string;
  user_id?: string;
  name: string;
  description: string;
  level: string;
  language: string;
  subjects: string[];
  questions: OrderPhraseQuestionData[];
}

export interface OrderPhraseSetListResponse {
  sets: OrderPhraseSetData[];
  count: number;
}

// == Create Order Phrase Input/Response ==
export interface CreateOrderPhraseInput {
  name: string;
  description: string;
  instructions: string;
  language: string;
  level: string;
  subjects: string[];
  num_questions: number;
  model?: string;
}

export interface CreateOrderPhraseResponse {
  success: boolean;
  message: string;
  id: string;
  total_token_count: number;
  documents_processed: number;
  questions: OrderPhraseQuestionData[];
}

// == Create Order Phrase ==
/**
 * Create order phrase questions from uploaded documents.
 * Sends multipart form data with JSON metadata and file uploads.
 */
export async function createOrderPhrase(
  input: CreateOrderPhraseInput,
  files: File[]
): Promise<CreateOrderPhraseResponse> {
  validateSubjects(input.subjects);

  const formData = new FormData();
  formData.append("metadata", JSON.stringify(input));

  for (const file of files) {
    formData.append("files", file);
  }

  const res = await fetch(endpoints.intello.orderPhrases(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create order phrase questions (${res.status})`));
  }

  return await res.json();
}

// == Get All User Order Phrase Sets ==
export async function getAllOrderPhraseSets(): Promise<OrderPhraseSetData[]> {
  const res = await fetch(endpoints.intello.orderPhrases(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch order phrase sets (${res.status})`));
  }

  const data: OrderPhraseSetListResponse = await res.json();
  return data.sets;
}

// == Fill Blank Types ==
export interface FillBlankOptionData {
  id: string;
  text: string;
  is_correct: boolean;
}

export interface FillBlankQuestionData {
  id: string;
  phrase: string;
  options: FillBlankOptionData[];
  explanation: string;
}

export interface FillBlankSetData {
  id: string;
  user_id?: string;
  name: string;
  description: string;
  level: string;
  language: string;
  subjects: string[];
  questions: FillBlankQuestionData[];
}

export interface FillBlankSetListResponse {
  sets: FillBlankSetData[];
  count: number;
}

// == Create Fill Blank Input/Response ==
export interface CreateFillBlankInput {
  name: string;
  description: string;
  instructions: string;
  language: string;
  level: string;
  subjects: string[];
  num_questions: number;
  model?: string;
}

export interface CreateFillBlankResponse {
  success: boolean;
  message: string;
  id: string;
  total_token_count: number;
  documents_processed: number;
  questions: FillBlankQuestionData[];
}

// == Create Fill Blank ==
/**
 * Create fill blank questions from uploaded documents.
 * Sends multipart form data with JSON metadata and file uploads.
 */
export async function createFillBlank(
  input: CreateFillBlankInput,
  files: File[]
): Promise<CreateFillBlankResponse> {
  validateSubjects(input.subjects);

  const formData = new FormData();
  formData.append("metadata", JSON.stringify(input));

  for (const file of files) {
    formData.append("files", file);
  }

  const res = await fetch(endpoints.intello.fillBlanks(), {
    method: "POST",
    credentials: "include",
    body: formData,
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to create fill blank questions (${res.status})`));
  }

  return await res.json();
}

// == Get All User Fill Blank Sets ==
export async function getAllFillBlankSets(): Promise<FillBlankSetData[]> {
  const res = await fetch(endpoints.intello.fillBlanks(), {
    method: "GET",
    credentials: "include",
  });

  if (!res.ok) {
    const errorBody = await res.json().catch(() => null);
    throw new Error(extractErrorMessage(errorBody, `Failed to fetch fill blank sets (${res.status})`));
  }

  const data: FillBlankSetListResponse = await res.json();
  return data.sets;
}
