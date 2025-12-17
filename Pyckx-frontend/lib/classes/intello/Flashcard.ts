// == Flashcard Class - Represents a single flashcard ==

import type { FlashcardData } from "./types";

export class Flashcard {
	readonly id: string;
	front: string;
	back: string;

	constructor(data: FlashcardData) {
		this.id = data.id;
		this.front = data.front;
		this.back = data.back;
	}

	// == Instance Methods ==

	toJSON(): FlashcardData {
		return {
			id: this.id,
			front: this.front,
			back: this.back,
		};
	}

	clone(): Flashcard {
		return new Flashcard(this.toJSON());
	}

	// == Static Factory ==

	static fromAPI(data: FlashcardData): Flashcard {
		return new Flashcard(data);
	}
}
