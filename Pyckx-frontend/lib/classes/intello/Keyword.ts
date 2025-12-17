import { KeywordData } from "./types"

/**
 * Represents a single keyword option
 */
export class Keyword {
	readonly id: string
	word: string
	isCorrect: boolean

	constructor(data: KeywordData) {
		this.id = data.id
		this.word = data.word
		this.isCorrect = data.is_correct
	}

	toData(): KeywordData {
		return {
			id: this.id,
			word: this.word,
			is_correct: this.isCorrect,
		}
	}

	static fromData(data: KeywordData): Keyword {
		return new Keyword(data)
	}
}
