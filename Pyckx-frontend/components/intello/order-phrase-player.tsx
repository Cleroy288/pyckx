"use client"

import { useState, useCallback, useMemo, useEffect } from "react"
import { ArrowLeft, ArrowRight, CheckCircle2, XCircle, Trophy, Lightbulb, RefreshCw } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import type { OrderPhraseQuestionData, OrderPhraseWordData } from "@/lib/api/intello"

interface OrderPhrasePlayerProps {
	questions: OrderPhraseQuestionData[]
	title: string
	onBack: () => void
}

export function OrderPhrasePlayer({ questions, title, onBack }: OrderPhrasePlayerProps) {
	const [currentIndex, setCurrentIndex] = useState(0)
	const [userOrder, setUserOrder] = useState<OrderPhraseWordData[]>([])
	const [availableWords, setAvailableWords] = useState<OrderPhraseWordData[]>([])
	const [hasChecked, setHasChecked] = useState(false)
	const [isCorrect, setIsCorrect] = useState(false)
	const [showHint, setShowHint] = useState(false)
	const [results, setResults] = useState<boolean[]>([])
	const [isComplete, setIsComplete] = useState(false)

	// Initialize shuffled words for current question
	const currentQuestion = questions[currentIndex]

	// Shuffle words
	const shuffleWords = useCallback((words: OrderPhraseWordData[]) => {
		const shuffled = [...words].sort(() => Math.random() - 0.5)
		return shuffled
	}, [])

	// Initialize question on mount and when currentIndex changes
	useEffect(() => {
		if (currentQuestion && currentQuestion.words) {
			setAvailableWords(shuffleWords(currentQuestion.words))
			setUserOrder([])
			setHasChecked(false)
			setIsCorrect(false)
			setShowHint(false)
		}
	}, [currentIndex, shuffleWords]) // eslint-disable-line react-hooks/exhaustive-deps

	// Add word to user order
	const handleAddWord = (word: OrderPhraseWordData) => {
		if (hasChecked) return
		setAvailableWords(prev => prev.filter(w => w.id !== word.id))
		setUserOrder(prev => [...prev, word])
	}

	// Remove word from user order
	const handleRemoveWord = (word: OrderPhraseWordData) => {
		if (hasChecked) return
		setUserOrder(prev => prev.filter(w => w.id !== word.id))
		setAvailableWords(prev => [...prev, word])
	}

	// Check answer
	const handleCheck = () => {
		const correct = userOrder.every((word, idx) => word.position === idx)
		setIsCorrect(correct)
		setHasChecked(true)
		setResults(prev => [...prev, correct])
	}

	// Next question
	const handleNext = () => {
		if (currentIndex + 1 >= questions.length) {
			setIsComplete(true)
		} else {
			// Just update index - useEffect will handle initialization
			setCurrentIndex(prev => prev + 1)
		}
	}

	// Reset current question
	const handleReset = () => {
		if (currentQuestion && currentQuestion.words) {
			setAvailableWords(shuffleWords(currentQuestion.words))
			setUserOrder([])
			setHasChecked(false)
			setIsCorrect(false)
			setShowHint(false)
		}
	}

	const progress = ((currentIndex + 1) / questions.length) * 100
	const score = useMemo(() => {
		const correct = results.filter(Boolean).length
		return {
			correct,
			total: results.length,
			percentage: results.length > 0 ? Math.round((correct / results.length) * 100) : 0
		}
	}, [results])

	// Results screen
	if (isComplete) {
		return (
			<div className="space-y-6 max-w-2xl mx-auto">
				<div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-8 text-center">
					<div className="flex justify-center mb-4">
						<div className="flex h-20 w-20 items-center justify-center rounded-full bg-accent/20 border border-accent/30">
							<Trophy className="h-10 w-10 text-accent" />
						</div>
					</div>
					<h2 className="text-2xl font-bold text-foreground mb-2">Quiz Complete!</h2>
					<p className="text-muted-foreground mb-6">{title}</p>

					<div className="flex justify-center gap-8 mb-6">
						<div className="text-center">
							<div className={`text-4xl font-bold ${score.percentage >= 70 ? 'text-green-500' : score.percentage >= 50 ? 'text-yellow-500' : 'text-red-500'}`}>
								{score.percentage}%
							</div>
							<div className="text-sm text-muted-foreground">Score</div>
						</div>
						<div className="text-center">
							<div className="text-4xl font-bold text-foreground">{score.correct}/{score.total}</div>
							<div className="text-sm text-muted-foreground">Correct</div>
						</div>
					</div>

					<Button onClick={onBack} className="w-full max-w-xs">
						Back to Sets
					</Button>
				</div>
			</div>
		)
	}

	return (
		<div className="space-y-6 max-w-3xl mx-auto">
			{/* Header */}
			<div className="flex items-center justify-between">
				<Button variant="ghost" size="sm" onClick={onBack} className="gap-2">
					<ArrowLeft className="h-4 w-4" />Exit
				</Button>
				<div className="text-sm text-muted-foreground">
					Question {currentIndex + 1} of {questions.length}
				</div>
			</div>

			{/* Progress */}
			<div className="space-y-2">
				<Progress value={progress} className="h-2" />
				<div className="flex justify-between text-xs text-muted-foreground">
					<span>{title}</span>
					<span>{score.correct} correct so far</span>
				</div>
			</div>

			{/* Question Card */}
			<div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-6 space-y-6">
				{/* Hint button */}
				<div className="flex justify-between items-center">
					<h2 className="text-lg font-semibold text-foreground">Arrange the words in order:</h2>
					<Button
						variant="ghost"
						size="sm"
						onClick={() => setShowHint(!showHint)}
						className="gap-2 text-muted-foreground hover:text-primary"
					>
						<Lightbulb className="h-4 w-4" />
						Hint
					</Button>
				</div>

				{/* Hint display */}
				{showHint && currentQuestion.hint && (
					<div className="rounded-lg bg-primary/10 border border-primary/20 p-3">
						<p className="text-sm text-primary">{currentQuestion.hint}</p>
					</div>
				)}

				{/* Drop zone - User's ordered words */}
				<div className="min-h-[80px] rounded-xl border-2 border-dashed border-accent/30 bg-accent/5 p-4 flex flex-wrap gap-2">
					{userOrder.length === 0 && (
						<span className="text-muted-foreground text-sm italic">Click words below to build the phrase...</span>
					)}
					{userOrder.map((word, idx) => (
						<button
							key={word.id}
							onClick={() => handleRemoveWord(word)}
							disabled={hasChecked}
							className={`px-4 py-2 rounded-lg font-medium transition-all cursor-pointer
								${hasChecked
									? word.position === idx
										? 'bg-green-500/20 border border-green-500/40 text-green-500'
										: 'bg-red-500/20 border border-red-500/40 text-red-500'
									: 'bg-accent/20 border border-accent/30 text-foreground hover:bg-accent/30'
								}
								disabled:cursor-not-allowed`}
						>
							{word.word}
						</button>
					))}
				</div>

				{/* Available words */}
				<div className="flex flex-wrap gap-2 justify-center">
					{availableWords.map((word) => (
						<button
							key={word.id}
							onClick={() => handleAddWord(word)}
							disabled={hasChecked}
							className="px-4 py-2 rounded-lg bg-card border border-border text-foreground font-medium 
								hover:border-accent/50 hover:bg-accent/10 transition-all cursor-pointer
								disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{word.word}
						</button>
					))}
				</div>

				{/* Actions */}
				<div className="flex gap-3">
					{!hasChecked && (
						<>
							<Button
								variant="outline"
								onClick={handleReset}
								className="gap-2"
							>
								<RefreshCw className="h-4 w-4" />
								Reset
							</Button>
							<Button
								onClick={handleCheck}
								disabled={userOrder.length !== currentQuestion?.words.length}
								className="flex-1 gap-2 bg-accent text-accent-foreground hover:bg-accent/90"
							>
								Check Answer
							</Button>
						</>
					)}
				</div>

				{/* Feedback */}
				{hasChecked && (
					<div className={`rounded-xl p-4 ${isCorrect ? 'bg-green-500/10 border border-green-500/20' : 'bg-red-500/10 border border-red-500/20'}`}>
						<div className="flex items-center gap-2 mb-2">
							{isCorrect ? (
								<><CheckCircle2 className="h-5 w-5 text-green-500" /><span className="font-semibold text-green-500">Correct!</span></>
							) : (
								<><XCircle className="h-5 w-5 text-red-500" /><span className="font-semibold text-red-500">Incorrect</span></>
							)}
						</div>
						<p className="text-sm text-muted-foreground">
							<span className="font-medium">Correct order: </span>
							{currentQuestion.original_phrase}
						</p>
					</div>
				)}

				{/* Next button */}
				{hasChecked && (
					<Button onClick={handleNext} className="w-full gap-2 bg-accent text-accent-foreground hover:bg-accent/90">
						{currentIndex + 1 >= questions.length ? (
							<><Trophy className="h-4 w-4" />See Results</>
						) : (
							<>Next Question<ArrowRight className="h-4 w-4" /></>
						)}
					</Button>
				)}
			</div>
		</div>
	)
}
