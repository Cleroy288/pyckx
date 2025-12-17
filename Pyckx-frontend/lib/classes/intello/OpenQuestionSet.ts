// == OpenQuestionSet Class - Represents a set of open-ended questions ==

import { OpenQuestion } from "./OpenQuestion";
import type {
  OpenQuestionSetData,
  CreateOpenQuestionSetInput,
  UpdateOpenQuestionSetInput,
  OpenQuestionSetSuccessResponse,
  Level,
} from "./types";

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") return fallback;
  const err = errorBody as { message?: string };
  return err.message || fallback;
}

// == OpenQuestionSet Class ==
export class OpenQuestionSet {
  readonly id: string;
  readonly userId: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  questions: OpenQuestion[];

  constructor(data: OpenQuestionSetData) {
    this.id = data.id;
    this.userId = data.user_id;
    this.name = data.name;
    this.description = data.description;
    this.level = data.level;
    this.language = data.language;
    this.subjects = [...data.subjects];
    this.questions = data.questions.map((q) => OpenQuestion.fromAPI(q));
  }

  // == Instance Methods ==

  toJSON(): OpenQuestionSetData {
    return {
      id: this.id,
      user_id: this.userId,
      name: this.name,
      description: this.description,
      level: this.level,
      language: this.language,
      subjects: [...this.subjects],
      questions: this.questions.map((q) => q.toJSON()),
    };
  }

  clone(): OpenQuestionSet {
    return new OpenQuestionSet(this.toJSON());
  }

  // == Static Factory ==

  static fromAPI(data: OpenQuestionSetData): OpenQuestionSet {
    return new OpenQuestionSet(data);
  }

  // == API Methods ==

  static async create(input: CreateOpenQuestionSetInput): Promise<OpenQuestionSet> {
    const res = await fetch("/app/intello/open-question", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify(input),
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to create open question set (${res.status})`)
      );
    }

    const result: OpenQuestionSetSuccessResponse = await res.json();
    if (!result.set) {
      throw new Error("No set returned from API");
    }
    return OpenQuestionSet.fromAPI(result.set);
  }

  async save(input: UpdateOpenQuestionSetInput): Promise<OpenQuestionSet> {
    const res = await fetch(`/app/intello/open-question/${this.id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify(input),
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to update open question set (${res.status})`)
      );
    }

    const result: OpenQuestionSetSuccessResponse = await res.json();
    if (!result.set) {
      throw new Error("No set returned from API");
    }
    return OpenQuestionSet.fromAPI(result.set);
  }

  async delete(): Promise<void> {
    const res = await fetch(`/app/intello/open-question/${this.id}`, {
      method: "DELETE",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to delete open question set (${res.status})`)
      );
    }
  }

  static async getById(id: string): Promise<OpenQuestionSet> {
    const res = await fetch(`/app/intello/open-question/${id}`, {
      method: "GET",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to fetch open question set (${res.status})`)
      );
    }

    const data: OpenQuestionSetData = await res.json();
    return OpenQuestionSet.fromAPI(data);
  }

  // == Utility ==

  /** Get the number of questions in this set */
  get questionCount(): number {
    return this.questions.length;
  }

  /** Check if this set has any questions */
  get hasQuestions(): boolean {
    return this.questions.length > 0;
  }

  /** Get the number of answered questions */
  get answeredCount(): number {
    return this.questions.filter((q) => q.isAnswered).length;
  }

  /** Get the completion percentage (0-100) */
  get completionPercentage(): number {
    if (this.questions.length === 0) return 0;
    return Math.round((this.answeredCount / this.questions.length) * 100);
  }

  /** Check if all questions have been answered */
  get isComplete(): boolean {
    return this.questions.length > 0 && this.answeredCount === this.questions.length;
  }

  /** Get a random unanswered question */
  getRandomUnansweredQuestion(): OpenQuestion | undefined {
    const unanswered = this.questions.filter((q) => !q.isAnswered);
    if (unanswered.length === 0) return undefined;
    const index = Math.floor(Math.random() * unanswered.length);
    return unanswered[index];
  }

  /** Clear all user answers */
  clearAllAnswers(): void {
    this.questions.forEach((q) => q.clearAnswer());
  }
}
