// == TrueOrFalseStatement Class - Represents a single true/false statement ==

import type { TrueOrFalseStatementData } from "./types";

export class TrueOrFalseStatement {
	readonly id: string;
	statement: string;
	answer: boolean;
	explanation: string;

	constructor(data: TrueOrFalseStatementData) {
		this.id = data.id;
		this.statement = data.statement;
		this.answer = data.answer;
		this.explanation = data.explanation;
	}

	// == Instance Methods ==

	toJSON(): TrueOrFalseStatementData {
		return {
			id: this.id,
			statement: this.statement,
			answer: this.answer,
			explanation: this.explanation,
		};
	}

	clone(): TrueOrFalseStatement {
		return new TrueOrFalseStatement(this.toJSON());
	}

	// == Static Factory ==

	static fromAPI(data: TrueOrFalseStatementData): TrueOrFalseStatement {
		return new TrueOrFalseStatement(data);
	}

	// == Utility ==

	/** Check if statement is true */
	get isTrue(): boolean {
		return this.answer === true;
	}

	/** Check if statement is false */
	get isFalse(): boolean {
		return this.answer === false;
	}
}
