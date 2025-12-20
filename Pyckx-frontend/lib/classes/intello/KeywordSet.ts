import { KeywordQuestion } from "./KeywordQuestion"
import { KeywordSetData, KeywordQuestionData, Level } from "./types"
import { endpoints } from "@/lib/api/config"

/**
 * Represents a set of keyword questions
 */
export class KeywordSet {
	readonly id: string
	readonly userId?: string
	name: string
	description: string
	level: Level
	language: string
	subjects: string[]
	questions: KeywordQuestion[]

	constructor(data: KeywordSetData) {
		this.id = data.id
		this.userId = data.user_id
		this.name = data.name
		this.description = data.description
		this.level = data.level as Level
		this.language = data.language
		this.subjects = data.subjects || []
		this.questions = data.questions.map((q: KeywordQuestionData) => KeywordQuestion.fromData(q))
	}

	get questionCount(): number {
		return this.questions.length
	}

	get isEmpty(): boolean {
		return this.questions.length === 0
	}

	get levelDisplay(): string {
		return this.level.charAt(0).toUpperCase() + this.level.slice(1).toLowerCase()
	}

	toData(): KeywordSetData {
		return {
			id: this.id,
			user_id: this.userId,
			name: this.name,
			description: this.description,
			level: this.level,
			language: this.language,
			subjects: this.subjects,
			questions: this.questions.map(q => q.toData()),
		}
	}

	static fromData(data: KeywordSetData): KeywordSet {
		return new KeywordSet(data)
	}

	/**
	 * Fetch all keyword sets for the current user
	 */
	static async getAll(): Promise<KeywordSet[]> {
		const response = await fetch(endpoints.intello.keywords, {
			method: "GET",
			credentials: "include",
		})

		if (!response.ok) {
			throw new Error("Failed to fetch keyword sets")
		}

		const data = await response.json()
		return data.sets.map((set: KeywordSetData) => KeywordSet.fromData(set))
	}
}
