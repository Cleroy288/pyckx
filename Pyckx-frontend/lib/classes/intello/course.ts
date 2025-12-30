import type { QcmSetData } from "./types"

// =============================================================================
// BLOCK PROTOCOL TYPES (matches backend)
// =============================================================================

export interface CourseMetadata {
	id: string
	title: string
	description: string
	generated_at: string
}

export type ContentBlock =
	| { type: "title"; content: string }
	| { type: "subtitle"; content: string }
	| { type: "text"; content: string }
	| { type: "schema"; language: string; content: string }

export interface CourseModule {
	id: string
	title: string
	blocks: ContentBlock[]
}

export interface GeneratedCourse {
	course_metadata: CourseMetadata
	modules: CourseModule[]
}

// =============================================================================
// ENTITY TYPES (database models)
// =============================================================================

export interface Course {
	id: string
	user_id: string
	name: string
	description: string
	resources?: CourseResource[]
	created_at: string
	updated_at: string
}

export interface CourseResource {
	id: string
	filename: string
	token_count: number
	created_at: string
}

export interface StudySession {
	id: string
	course_id: string
	topic: string
	instructions: string
	keywords: string[]
	language: string
	status: "in_progress" | "completed"
	created_at: string
	completed_at?: string
}

export interface SessionContent {
	id: string
	session_id: string
	content_type: "course" | "diagram" | "exercise" | "qcm"
	title: string
	content: string
	mermaid_code?: string
	order_index: number
	created_at: string
}

export interface SessionGame {
	id: string
	game_type: string
	game_set_id: string
	errors_count: number
	total_questions: number
	completed: boolean
	created_at: string
}

export interface SessionQuestion {
	id: string
	section_context: string
	question: string
	answer: string
	created_at: string
}

export interface SessionNote {
	id: string
	game_type?: string
	game_set_id?: string
	note: string
	created_at: string
}

export interface SessionSynthesis {
	id: string
	version: number
	content: string
	errors_summary: Record<string, number>
	notes_addressed: number
	combined_from: string[]
	created_at: string
}

// =============================================================================
// REQUEST/RESPONSE TYPES
// =============================================================================

export interface CreateCourseInput {
	name: string
	description?: string
}

export interface AddResourceInput {
	filename: string
	content: string
	token_count?: number
}

export interface StartSessionInput {
	topic: string
	instructions?: string
	keywords?: string[]
	language?: string
	resource_ids?: string[]
}

export interface AddNoteInput {
	game_type?: string
	game_set_id?: string
	note: string
}

export interface CourseListResponse {
	courses: Course[]
}

export interface ResourcesResponse {
	resources: CourseResource[]
}

export interface SessionListResponse {
	sessions: StudySession[]
}

export interface SessionGamesResponse {
	games: SessionGame[]
}

export interface SessionQuestionsResponse {
	questions: SessionQuestion[]
}

export interface SessionNotesResponse {
	notes: SessionNote[]
}
