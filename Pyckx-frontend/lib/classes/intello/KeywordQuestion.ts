import { Keyword } from "./Keyword"
import { KeywordQuestionData, KeywordData } from "./types"

/**
 * Represents a keyword question with a statement and keyword options
 */
export class KeywordQuestion {
	readonly id: string
	statement: string
	keywords: Keyword[]
	explanation: string

	constructor(data: KeywordQuestionData) {
		this.id = data.id
		this.statement = data.statement
		this.keywords = data.keywords.map((k: KeywordData) => Keyword.fromData(k))
		this.explanation = data.explanation
	}

	get correctKeywords(): Keyword[] {
		return this.keywords.filter(k => k.isCorrect)
	}

	get incorrectKeywords(): Keyword[] {
		return this.keywords.filter(k => !k.isCorrect)
	}

	get correctCount(): number {
		return this.correctKeywords.length
	}

	toData(): KeywordQuestionData {
		return {
			id: this.id,
			statement: this.statement,
			keywords: this.keywords.map(k => k.toData()),
			explanation: this.explanation,
		}
	}

	static fromData(data: KeywordQuestionData): KeywordQuestion {
		return new KeywordQuestion(data)
	}
}
