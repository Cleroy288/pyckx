// == TrueOrFalseSet Class - Represents a set of true/false statements ==

import { TrueOrFalseStatement } from "./TrueOrFalseStatement";
import { endpoints } from "@/lib/api/config";
import type {
	TrueOrFalseSetData,
	TrueOrFalseSetListResponse,
	Level,
} from "./types";

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
	if (!errorBody || typeof errorBody !== "object") return fallback;
	const err = errorBody as { message?: string };
	return err.message || fallback;
}

// == TrueOrFalseSet Class ==
export class TrueOrFalseSet {
	readonly id: string;
	readonly userId: string;
	name: string;
	description: string;
	level: Level;
	language: string;
	subjects: string[];
	statements: TrueOrFalseStatement[];

	constructor(data: TrueOrFalseSetData) {
		this.id = data.id;
		this.userId = data.user_id;
		this.name = data.name;
		this.description = data.description;
		this.level = data.level;
		this.language = data.language;
		this.subjects = [...data.subjects];
		this.statements = data.statements.map((s) => TrueOrFalseStatement.fromAPI(s));
	}

	// == Instance Methods ==

	toJSON(): TrueOrFalseSetData {
		return {
			id: this.id,
			user_id: this.userId,
			name: this.name,
			description: this.description,
			level: this.level,
			language: this.language,
			subjects: [...this.subjects],
			statements: this.statements.map((s) => s.toJSON()),
		};
	}

	clone(): TrueOrFalseSet {
		return new TrueOrFalseSet(this.toJSON());
	}

	// == Static Factory ==

	static fromAPI(data: TrueOrFalseSetData): TrueOrFalseSet {
		return new TrueOrFalseSet(data);
	}

	// == API Methods ==

	/** Get all true/false sets for the current user */
	static async getAll(): Promise<TrueOrFalseSet[]> {
		const res = await fetch(endpoints.intello.trueFalse, {
			method: "GET",
			credentials: "include",
		});

		if (!res.ok) {
			const errorBody = await res.json().catch(() => null);
			throw new Error(
				extractErrorMessage(errorBody, `Failed to fetch true/false sets (${res.status})`)
			);
		}

		const data: TrueOrFalseSetListResponse = await res.json();
		return data.sets.map((set) => TrueOrFalseSet.fromAPI(set));
	}

	// == Utility ==

	/** Get the number of statements in this set */
	get statementCount(): number {
		return this.statements.length;
	}

	/** Check if this set has any statements */
	get hasStatements(): boolean {
		return this.statements.length > 0;
	}

	/** Get the count of true statements */
	get trueCount(): number {
		return this.statements.filter((s) => s.isTrue).length;
	}

	/** Get the count of false statements */
	get falseCount(): number {
		return this.statements.filter((s) => s.isFalse).length;
	}

	/** Get a random statement from this set */
	getRandomStatement(): TrueOrFalseStatement | undefined {
		if (this.statements.length === 0) return undefined;
		const index = Math.floor(Math.random() * this.statements.length);
		return this.statements[index];
	}
}
