"use client"

import { createContext, useContext, useState, useCallback, useMemo, useRef, type ReactNode } from "react"
import { getAvailableGames, getAllQcmSets, getAllOpenQuestionSets, getAllFlashcardSets, getAllTrueOrFalseSets, getAllKeywordSets, getAllOrderPhraseSets, type GameData, type OpenQuestionSetData, type FlashcardSetData, type TrueOrFalseSetData, type KeywordSetData, type OrderPhraseSetData } from "@/lib/api/intello"
import type { QcmSetData, QcmQuestionData, Level } from "@/lib/classes/intello"

// == Types ==
export type ViewState =
	| "home"
	| "create-qcm" | "create-ai-qcm" | "create-ai-open" | "create-ai-flashcard" | "create-ai-true-false" | "create-ai-keywords" | "create-ai-order-phrase"
	| "play-qcm" | "play-open" | "play-flashcard" | "play-true-false" | "play-keywords" | "play-order-phrase"
	| "playing-qcm" | "playing-open" | "playing-flashcard" | "playing-true-false" | "playing-keywords" | "playing-order-phrase"
	| "results"

export interface QuizAnswer {
	questionIndex: number
	selectedAnswer: string
	isCorrect: boolean
	correctAnswer: string
	explanation: string
}

// == Context Type ==
interface IntelloContextType {
	// View state
	view: ViewState
	setView: (view: ViewState) => void

	// Loading/error
	loading: boolean
	error: string | null
	setError: (error: string | null) => void

	// Games/Sets data
	games: GameData[]
	qcmSets: QcmSetData[]
	openQuestionSets: OpenQuestionSetData[]
	flashcardSets: FlashcardSetData[]
	trueOrFalseSets: TrueOrFalseSetData[]
	keywordSets: KeywordSetData[]
	orderPhraseSets: OrderPhraseSetData[]

	// Selected sets
	selectedQcmSet: QcmSetData | null
	selectedOpenSet: OpenQuestionSetData | null
	selectedFlashcardSet: FlashcardSetData | null
	selectedTrueOrFalseSet: TrueOrFalseSetData | null
	selectedKeywordSet: KeywordSetData | null
	selectedOrderPhraseSet: OrderPhraseSetData | null

	// QCM quiz state
	currentQuestionIndex: number
	selectedAnswer: string | null
	hasAnswered: boolean
	answers: QuizAnswer[]
	shuffledAnswersRef: React.MutableRefObject<string[]>
	score: { correct: number; total: number; percentage: number }

	// Load functions
	loadGames: () => Promise<void>
	loadQcmSets: () => Promise<void>
	loadOpenQuestionSets: () => Promise<void>
	loadFlashcardSets: () => Promise<void>
	loadTrueOrFalseSets: () => Promise<void>
	loadKeywordSets: () => Promise<void>
	loadOrderPhraseSets: () => Promise<void>

	// Navigation handlers
	handleBackToHome: () => void
	handleNavigateToGame: (gameType: "qcm" | "open" | "flashcard" | "true_false" | "keywords" | "order_phrase") => Promise<void>
	handlePlayQcm: () => Promise<void>
	handlePlayOpen: () => Promise<void>
	handlePlayFlashcard: () => Promise<void>
	handlePlayTrueOrFalse: () => Promise<void>
	handlePlayKeywords: () => Promise<void>
	handlePlayOrderPhrase: () => Promise<void>

	// Selection handlers
	handleSelectQcmSet: (set: QcmSetData) => void
	handleSelectOpenSet: (set: OpenQuestionSetData) => void
	handleSelectFlashcardSet: (set: FlashcardSetData) => void
	handleSelectTrueOrFalseSet: (set: TrueOrFalseSetData) => void
	handleSelectKeywordSet: (set: KeywordSetData) => void
	handleSelectOrderPhraseSet: (set: OrderPhraseSetData) => void

	// QCM game handlers
	handleAnswerSelect: (answer: string) => void
	handleNextQuestion: () => void
	handlePlayAgain: () => void
	shuffleAnswers: (question: QcmQuestionData) => string[]

	// Helpers
	getLevelBadgeClass: (level: Level) => string
	getScoreColor: (percentage: number) => string
}

const IntelloContext = createContext<IntelloContextType | null>(null)

// == Provider ==
export function IntelloProvider({ children }: { children: ReactNode }) {
	// View state
	const [view, setView] = useState<ViewState>("home")
	const [loading, setLoading] = useState(false)
	const [error, setError] = useState<string | null>(null)

	// Data state
	const [games, setGames] = useState<GameData[]>([])
	const [qcmSets, setQcmSets] = useState<QcmSetData[]>([])
	const [openQuestionSets, setOpenQuestionSets] = useState<OpenQuestionSetData[]>([])
	const [flashcardSets, setFlashcardSets] = useState<FlashcardSetData[]>([])
	const [trueOrFalseSets, setTrueOrFalseSets] = useState<TrueOrFalseSetData[]>([])
	const [keywordSets, setKeywordSets] = useState<KeywordSetData[]>([])
	const [orderPhraseSets, setOrderPhraseSets] = useState<OrderPhraseSetData[]>([])

	// Selected sets
	const [selectedQcmSet, setSelectedQcmSet] = useState<QcmSetData | null>(null)
	const [selectedOpenSet, setSelectedOpenSet] = useState<OpenQuestionSetData | null>(null)
	const [selectedFlashcardSet, setSelectedFlashcardSet] = useState<FlashcardSetData | null>(null)
	const [selectedTrueOrFalseSet, setSelectedTrueOrFalseSet] = useState<TrueOrFalseSetData | null>(null)
	const [selectedKeywordSet, setSelectedKeywordSet] = useState<KeywordSetData | null>(null)
	const [selectedOrderPhraseSet, setSelectedOrderPhraseSet] = useState<OrderPhraseSetData | null>(null)

	// QCM state
	const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0)
	const [selectedAnswer, setSelectedAnswer] = useState<string | null>(null)
	const [hasAnswered, setHasAnswered] = useState(false)
	const [answers, setAnswers] = useState<QuizAnswer[]>([])
	const shuffledAnswersRef = useRef<string[]>([])

	// == Load Functions ==
	const loadGames = useCallback(async () => {
		try {
			const data = await getAvailableGames()
			setGames(data)
		} catch (err) {
			console.error("Failed to load games:", err)
		}
	}, [])

	const loadQcmSets = useCallback(async () => {
		setLoading(true)
		setError(null)
		try {
			const data = await getAllQcmSets()
			setQcmSets(data)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to load QCM sets")
		} finally {
			setLoading(false)
		}
	}, [])

	const loadOpenQuestionSets = useCallback(async () => {
		setLoading(true)
		setError(null)
		try {
			const data = await getAllOpenQuestionSets()
			setOpenQuestionSets(data)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to load open question sets")
		} finally {
			setLoading(false)
		}
	}, [])

	const loadFlashcardSets = useCallback(async () => {
		setLoading(true)
		setError(null)
		try {
			const data = await getAllFlashcardSets()
			setFlashcardSets(data)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to load flashcard sets")
		} finally {
			setLoading(false)
		}
	}, [])

	const loadTrueOrFalseSets = useCallback(async () => {
		setLoading(true)
		setError(null)
		try {
			const data = await getAllTrueOrFalseSets()
			setTrueOrFalseSets(data)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to load true/false sets")
		} finally {
			setLoading(false)
		}
	}, [])

	const loadKeywordSets = useCallback(async () => {
		setLoading(true)
		setError(null)
		try {
			const data = await getAllKeywordSets()
			setKeywordSets(data)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to load keyword sets")
		} finally {
			setLoading(false)
		}
	}, [])

	const loadOrderPhraseSets = useCallback(async () => {
		setLoading(true)
		setError(null)
		try {
			const data = await getAllOrderPhraseSets()
			setOrderPhraseSets(data)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to load order phrase sets")
		} finally {
			setLoading(false)
		}
	}, [])

	// == Helpers ==
	const shuffleAnswers = useCallback((question: QcmQuestionData): string[] => {
		const allAnswers = [...question.wrong_answers, question.right_answer]
		return allAnswers.sort(() => Math.random() - 0.5)
	}, [])

	const getLevelBadgeClass = useCallback((level: Level) => {
		switch (level) {
			case "easy": return "bg-green-500/10 text-green-500 border-green-500/20"
			case "medium": return "bg-yellow-500/10 text-yellow-500 border-yellow-500/20"
			case "hard": return "bg-red-500/10 text-red-500 border-red-500/20"
		}
	}, [])

	const getScoreColor = useCallback((percentage: number) => {
		if (percentage >= 80) return "text-green-500"
		if (percentage >= 60) return "text-yellow-500"
		return "text-red-500"
	}, [])

	const score = useMemo(() => {
		if (answers.length === 0) return { correct: 0, total: 0, percentage: 0 }
		const correct = answers.filter(a => a.isCorrect).length
		return { correct, total: answers.length, percentage: Math.round((correct / answers.length) * 100) }
	}, [answers])

	// == Navigation Handlers ==
	const handleBackToHome = useCallback(() => {
		setView("home")
		setSelectedQcmSet(null)
		setSelectedOpenSet(null)
		setSelectedFlashcardSet(null)
		setSelectedTrueOrFalseSet(null)
		setSelectedKeywordSet(null)
		setSelectedOrderPhraseSet(null)
		setAnswers([])
		setError(null)
	}, [])

	const handlePlayQcm = useCallback(async () => {
		await loadQcmSets()
		setView("play-qcm")
	}, [loadQcmSets])

	const handlePlayOpen = useCallback(async () => {
		await loadOpenQuestionSets()
		setView("play-open")
	}, [loadOpenQuestionSets])

	const handlePlayFlashcard = useCallback(async () => {
		await loadFlashcardSets()
		setView("play-flashcard")
	}, [loadFlashcardSets])

	const handlePlayTrueOrFalse = useCallback(async () => {
		await loadTrueOrFalseSets()
		setView("play-true-false")
	}, [loadTrueOrFalseSets])

	const handlePlayKeywords = useCallback(async () => {
		await loadKeywordSets()
		setView("play-keywords")
	}, [loadKeywordSets])

	const handlePlayOrderPhrase = useCallback(async () => {
		await loadOrderPhraseSets()
		setView("play-order-phrase")
	}, [loadOrderPhraseSets])

	const handleNavigateToGame = useCallback(async (gameType: "qcm" | "open" | "flashcard" | "true_false" | "keywords" | "order_phrase") => {
		switch (gameType) {
			case "qcm":
				await loadQcmSets()
				setView("play-qcm")
				break
			case "open":
				await loadOpenQuestionSets()
				setView("play-open")
				break
			case "flashcard":
				await loadFlashcardSets()
				setView("play-flashcard")
				break
			case "true_false":
				await loadTrueOrFalseSets()
				setView("play-true-false")
				break
			case "keywords":
				await loadKeywordSets()
				setView("play-keywords")
				break
			case "order_phrase":
				await loadOrderPhraseSets()
				setView("play-order-phrase")
				break
		}
	}, [loadQcmSets, loadOpenQuestionSets, loadFlashcardSets, loadTrueOrFalseSets, loadKeywordSets, loadOrderPhraseSets])

	// == Selection Handlers ==
	const handleSelectQcmSet = useCallback((set: QcmSetData) => {
		if (set.questions.length === 0) {
			setError("This quiz has no questions yet!")
			return
		}
		setSelectedQcmSet(set)
		setCurrentQuestionIndex(0)
		setSelectedAnswer(null)
		setHasAnswered(false)
		setAnswers([])
		shuffledAnswersRef.current = shuffleAnswers(set.questions[0])
		setView("playing-qcm")
	}, [shuffleAnswers])

	const handleSelectOpenSet = useCallback((set: OpenQuestionSetData) => {
		if (set.questions.length === 0) {
			setError("This set has no questions yet!")
			return
		}
		setSelectedOpenSet(set)
		setView("playing-open")
	}, [])

	const handleSelectFlashcardSet = useCallback((set: FlashcardSetData) => {
		if (set.cards.length === 0) {
			setError("This set has no cards yet!")
			return
		}
		setSelectedFlashcardSet(set)
		setView("playing-flashcard")
	}, [])

	const handleSelectTrueOrFalseSet = useCallback((set: TrueOrFalseSetData) => {
		if (set.statements.length === 0) {
			setError("This set has no statements yet!")
			return
		}
		setSelectedTrueOrFalseSet(set)
		setView("playing-true-false")
	}, [])

	const handleSelectKeywordSet = useCallback((set: KeywordSetData) => {
		if (set.questions.length === 0) {
			setError("This set has no questions yet!")
			return
		}
		setSelectedKeywordSet(set)
		setView("playing-keywords")
	}, [])

	const handleSelectOrderPhraseSet = useCallback((set: OrderPhraseSetData) => {
		if (set.questions.length === 0) {
			setError("This set has no questions yet!")
			return
		}
		setSelectedOrderPhraseSet(set)
		setView("playing-order-phrase")
	}, [])

	// == QCM Game Handlers ==
	const handleAnswerSelect = useCallback((answer: string) => {
		if (hasAnswered || !selectedQcmSet) return
		setSelectedAnswer(answer)
		setHasAnswered(true)
		const currentQuestion = selectedQcmSet.questions[currentQuestionIndex]
		const isCorrect = answer === currentQuestion.right_answer
		setAnswers(prev => [...prev, {
			questionIndex: currentQuestionIndex,
			selectedAnswer: answer,
			isCorrect,
			correctAnswer: currentQuestion.right_answer,
			explanation: currentQuestion.explanation,
		}])
	}, [hasAnswered, selectedQcmSet, currentQuestionIndex])

	const handleNextQuestion = useCallback(() => {
		if (!selectedQcmSet) return
		const nextIndex = currentQuestionIndex + 1
		if (nextIndex >= selectedQcmSet.questions.length) {
			setView("results")
		} else {
			setCurrentQuestionIndex(nextIndex)
			setSelectedAnswer(null)
			setHasAnswered(false)
			shuffledAnswersRef.current = shuffleAnswers(selectedQcmSet.questions[nextIndex])
		}
	}, [selectedQcmSet, currentQuestionIndex, shuffleAnswers])

	const handlePlayAgain = useCallback(() => {
		if (!selectedQcmSet) return
		setCurrentQuestionIndex(0)
		setSelectedAnswer(null)
		setHasAnswered(false)
		setAnswers([])
		shuffledAnswersRef.current = shuffleAnswers(selectedQcmSet.questions[0])
		setView("playing-qcm")
	}, [selectedQcmSet, shuffleAnswers])

	const value: IntelloContextType = {
		view, setView,
		loading, error, setError,
		games, qcmSets, openQuestionSets, flashcardSets, trueOrFalseSets, keywordSets, orderPhraseSets,
		selectedQcmSet, selectedOpenSet, selectedFlashcardSet, selectedTrueOrFalseSet, selectedKeywordSet, selectedOrderPhraseSet,
		currentQuestionIndex, selectedAnswer, hasAnswered, answers, shuffledAnswersRef, score,
		loadGames, loadQcmSets, loadOpenQuestionSets, loadFlashcardSets, loadTrueOrFalseSets, loadKeywordSets, loadOrderPhraseSets,
		handleBackToHome, handleNavigateToGame, handlePlayQcm, handlePlayOpen, handlePlayFlashcard, handlePlayTrueOrFalse, handlePlayKeywords, handlePlayOrderPhrase,
		handleSelectQcmSet, handleSelectOpenSet, handleSelectFlashcardSet, handleSelectTrueOrFalseSet, handleSelectKeywordSet, handleSelectOrderPhraseSet,
		handleAnswerSelect, handleNextQuestion, handlePlayAgain, shuffleAnswers,
		getLevelBadgeClass, getScoreColor,
	}

	return <IntelloContext.Provider value={value}>{children}</IntelloContext.Provider>
}

// == Hook ==
export function useIntello() {
	const context = useContext(IntelloContext)
	if (!context) {
		throw new Error("useIntello must be used within an IntelloProvider")
	}
	return context
}
