// == OpenQuestion Class - Represents a single open-ended question ==

import type { OpenQuestionData, CreateOpenQuestionInput } from "./types";

// == OpenQuestion Class ==
export class OpenQuestion {
  readonly id: string;
  question: string;
  userAnswer: string;
  expectedAnswer: string | null;
  hint: string | null;

  constructor(data: OpenQuestionData) {
    this.id = data.id;
    this.question = data.question;
    this.userAnswer = data.user_answer;
    this.expectedAnswer = data.expected_answer ?? null;
    this.hint = data.hint ?? null;
  }

  // == Instance Methods ==

  toJSON(): OpenQuestionData {
    return {
      id: this.id,
      question: this.question,
      user_answer: this.userAnswer,
      expected_answer: this.expectedAnswer ?? undefined,
      hint: this.hint ?? undefined,
    };
  }

  clone(): OpenQuestion {
    return new OpenQuestion(this.toJSON());
  }

  // == Static Factory ==

  static fromAPI(data: OpenQuestionData): OpenQuestion {
    return new OpenQuestion(data);
  }

  static fromInput(input: CreateOpenQuestionInput, id: string): OpenQuestion {
    return new OpenQuestion({
      id,
      question: input.question,
      user_answer: input.user_answer ?? "",
      expected_answer: input.expected_answer,
      hint: input.hint,
    });
  }

  // == Utility ==

  /** Check if the question has been answered */
  get isAnswered(): boolean {
    return this.userAnswer.trim().length > 0;
  }

  /** Check if an expected answer exists */
  get hasExpectedAnswer(): boolean {
    return this.expectedAnswer !== null && this.expectedAnswer.trim().length > 0;
  }

  /** Check if a hint is available */
  get hasHint(): boolean {
    return this.hint !== null && this.hint.trim().length > 0;
  }

  /** Set the user's answer */
  setAnswer(answer: string): void {
    this.userAnswer = answer;
  }

  /** Clear the user's answer */
  clearAnswer(): void {
    this.userAnswer = "";
  }
}
