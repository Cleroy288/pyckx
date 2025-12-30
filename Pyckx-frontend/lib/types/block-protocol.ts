// Block Protocol Types for Course Generation

export interface CourseMetadata {
	title: string
	description: string
	level?: string
}

// QCM Types
export interface QcmQuestionPayload {
	question: string
	right_answer: string
	wrong_answers: string[]
	explanation: string
}

export interface QcmSetPayload {
	name: string
	description: string
	level: string
	subjects: string[]
	questions: QcmQuestionPayload[]
}

// Flashcard Types
export interface FlashcardPayload {
	front: string
	back: string
}

export interface FlashcardSetPayload {
	name: string
	description: string
	level: string
	subjects: string[]
	cards: FlashcardPayload[]
}

// True/False Types
export interface TrueFalseStatementPayload {
	statement: string
	answer: boolean
	explanation: string
}

export interface TrueFalseSetPayload {
	name: string
	description: string
	level: string
	subjects: string[]
	statements: TrueFalseStatementPayload[]
}

// Content Block (7 types)
export type ContentBlock =
	| { type: "title"; content: string }
	| { type: "subtitle"; content: string }
	| { type: "text"; content: string }
	| { type: "schema"; language: string; content: string }
	| { type: "qcm_set"; data: QcmSetPayload }
	| { type: "true_false_set"; data: TrueFalseSetPayload }
	| { type: "flashcard_set"; data: FlashcardSetPayload }

export interface CourseModule {
	title: string
	blocks: ContentBlock[]
}

export interface GeneratedCourse {
	course_metadata: CourseMetadata
	modules: CourseModule[]
	synthesis: CourseModule  // Reuses CourseModule structure
}

export interface GenerateCourseResponse {
	success: boolean
	course: GeneratedCourse
}
