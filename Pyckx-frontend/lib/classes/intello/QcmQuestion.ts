// == QcmQuestion Class - Represents a single QCM question ==

import type { QcmQuestionData, CreateQcmQuestionInput } from "./types";

// == QcmQuestion Class ==
export class QcmQuestion {
  readonly id: string;
  question: string;
  wrongAnswers: string[];
  rightAnswer: string;
  explanation: string;

  constructor(data: QcmQuestionData) {
    this.id = data.id;
    this.question = data.question;
    this.wrongAnswers = [...data.wrong_answers];
    this.rightAnswer = data.right_answer;
    this.explanation = data.explanation;
  }

  // == Instance Methods ==

  toJSON(): QcmQuestionData {
    return {
      id: this.id,
      question: this.question,
      wrong_answers: [...this.wrongAnswers],
      right_answer: this.rightAnswer,
      explanation: this.explanation,
    };
  }

  clone(): QcmQuestion {
    return new QcmQuestion(this.toJSON());
  }

  // == Static Factory ==

  static fromAPI(data: QcmQuestionData): QcmQuestion {
    return new QcmQuestion(data);
  }

  static fromInput(input: CreateQcmQuestionInput, id: string): QcmQuestion {
    return new QcmQuestion({
      id,
      question: input.question,
      wrong_answers: input.wrong_answers,
      right_answer: input.right_answer,
      explanation: input.explanation,
    });
  }

  // == Utility ==

  /** Get all answers (wrong + right) shuffled */
  getAllAnswersShuffled(): string[] {
    const answers = [...this.wrongAnswers, this.rightAnswer];
    // Fisher-Yates shuffle
    for (let i = answers.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [answers[i], answers[j]] = [answers[j], answers[i]];
    }
    return answers;
  }

  /** Check if an answer is correct */
  isCorrect(answer: string): boolean {
    return answer === this.rightAnswer;
  }
}
