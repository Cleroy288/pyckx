// == FlashcardSet Class - Represents a set of flashcards ==

import { Flashcard } from "./Flashcard";
import { endpoints } from "@/lib/api/config";
import type {
	FlashcardSetData,
	FlashcardSetListResponse,
	Level,
} from "./types";

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
	if (!errorBody || typeof errorBody !== "object") return fallback;
	const err = errorBody as { message?: string };
	return err.message || fallback;
}

// == FlashcardSet Class ==
export class FlashcardSet {
	readonly id: string;
	readonly userId: string;
	name: string;
	description: string;
	level: Level;
	language: string;
	subjects: string[];
	cards: Flashcard[];

	constructor(data: FlashcardSetData) {
		this.id = data.id;
		this.userId = data.user_id;
		this.name = data.name;
		this.description = data.description;
		this.level = data.level;
		this.language = data.language;
		this.subjects = [...data.subjects];
		this.cards = data.cards.map((c) => Flashcard.fromAPI(c));
	}

	// == Instance Methods ==

	toJSON(): FlashcardSetData {
		return {
			id: this.id,
			user_id: this.userId,
			name: this.name,
			description: this.description,
			level: this.level,
			language: this.language,
			subjects: [...this.subjects],
			cards: this.cards.map((c) => c.toJSON()),
		};
	}

	clone(): FlashcardSet {
		return new FlashcardSet(this.toJSON());
	}

	// == Static Factory ==

	static fromAPI(data: FlashcardSetData): FlashcardSet {
		return new FlashcardSet(data);
	}

	// == API Methods ==

	/** Get all flashcard sets for the current user */
	static async getAll(): Promise<FlashcardSet[]> {
		const res = await fetch(endpoints.intello.flashcardList(), {
			method: "GET",
			credentials: "include",
		});

		if (!res.ok) {
			const errorBody = await res.json().catch(() => null);
			throw new Error(
				extractErrorMessage(errorBody, `Failed to fetch flashcard sets (${res.status})`)
			);
		}

		const data: FlashcardSetListResponse = await res.json();
		return data.sets.map((set) => FlashcardSet.fromAPI(set));
	}

	// == Utility ==

	/** Get the number of cards in this set */
	get cardCount(): number {
		return this.cards.length;
	}

	/** Check if this set has any cards */
	get hasCards(): boolean {
		return this.cards.length > 0;
	}

	/** Get a random card from this set */
	getRandomCard(): Flashcard | undefined {
		if (this.cards.length === 0) return undefined;
		const index = Math.floor(Math.random() * this.cards.length);
		return this.cards[index];
	}
}
