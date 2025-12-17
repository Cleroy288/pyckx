// == QcmSet Class - Represents a set of QCM questions ==

import { QcmQuestion } from "./QcmQuestion";
import type {
  QcmSetData,
  CreateQcmSetInput,
  UpdateQcmSetInput,
  QcmSetSuccessResponse,
  Level,
} from "./types";

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") return fallback;
  const err = errorBody as { message?: string };
  return err.message || fallback;
}

// == QcmSet Class ==
export class QcmSet {
  readonly id: string;
  readonly userId: string;
  name: string;
  description: string;
  level: Level;
  language: string;
  subjects: string[];
  questions: QcmQuestion[];

  constructor(data: QcmSetData) {
    this.id = data.id;
    this.userId = data.user_id;
    this.name = data.name;
    this.description = data.description;
    this.level = data.level;
    this.language = data.language;
    this.subjects = [...data.subjects];
    this.questions = data.questions.map((q) => QcmQuestion.fromAPI(q));
  }

  // == Instance Methods ==

  toJSON(): QcmSetData {
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

  clone(): QcmSet {
    return new QcmSet(this.toJSON());
  }

  // == Static Factory ==

  static fromAPI(data: QcmSetData): QcmSet {
    return new QcmSet(data);
  }

  // == API Methods ==

  // Note: These methods require endpoint configuration in lib/api/config.ts
  // Add the following endpoints:
  // - /app/intello/qcm (POST: create, GET: list)
  // - /app/intello/qcm/{id} (GET: single, PUT: update, DELETE: delete)

  static async create(input: CreateQcmSetInput): Promise<QcmSet> {
    const res = await fetch("/app/intello/qcm", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify(input),
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to create QCM set (${res.status})`)
      );
    }

    const result: QcmSetSuccessResponse = await res.json();
    if (!result.set) {
      throw new Error("No set returned from API");
    }
    return QcmSet.fromAPI(result.set);
  }

  async save(input: UpdateQcmSetInput): Promise<QcmSet> {
    const res = await fetch(`/app/intello/qcm/${this.id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify(input),
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to update QCM set (${res.status})`)
      );
    }

    const result: QcmSetSuccessResponse = await res.json();
    if (!result.set) {
      throw new Error("No set returned from API");
    }
    return QcmSet.fromAPI(result.set);
  }

  async delete(): Promise<void> {
    const res = await fetch(`/app/intello/qcm/${this.id}`, {
      method: "DELETE",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to delete QCM set (${res.status})`)
      );
    }
  }

  static async getById(id: string): Promise<QcmSet> {
    const res = await fetch(`/app/intello/qcm/${id}`, {
      method: "GET",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to fetch QCM set (${res.status})`)
      );
    }

    const data: QcmSetData = await res.json();
    return QcmSet.fromAPI(data);
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

  /** Get a random question from this set */
  getRandomQuestion(): QcmQuestion | undefined {
    if (this.questions.length === 0) return undefined;
    const index = Math.floor(Math.random() * this.questions.length);
    return this.questions[index];
  }
}
