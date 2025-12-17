"use client"

import { useState, useCallback } from "react"
import { CheckCircle, XCircle, ArrowRight, RotateCcw, Trophy, Tags } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { KeywordQuestionData, KeywordData } from "@/lib/api/intello"

interface KeywordsPlayerProps {
	questions: KeywordQuestionData[]
	title: string
	onBack: () => void
}

export function KeywordsPlayer({ questions, title, onBack }: KeywordsPlayerProps) {
	const [currentIndex, setCurrentIndex] = useState(0)
	const [selectedKeywords, setSelectedKeywords] = useState<Set<string>>(new Set())
	const [showResult, setShowResult] = useState(false)
	const [score, setScore] = useState(0)
	const [finished, setFinished] = useState(false)

	const currentQuestion = questions[currentIndex]

	const handleKeywordToggle = useCallback((keywordId: string) => {
		if (showResult) return
		setSelectedKeywords((prev) => {
			const next = new Set(prev)
			if (next.has(keywordId)) {
				next.delete(keywordId)
			} else {
				next.add(keywordId)
			}
			return next
		})
	}, [showResult])

	const handleSubmit = useCallback(() => {
		if (selectedKeywords.size === 0 || !currentQuestion) return
		setShowResult(true)

		// Calculate if all correct keywords were selected and no incorrect ones
		const correctKeywords = currentQuestion.keywords.filter(k => k.is_correct)
		const selectedCorrect = correctKeywords.filter(k => selectedKeywords.has(k.id)).length
		const selectedIncorrect = currentQuestion.keywords.filter(k => !k.is_correct && selectedKeywords.has(k.id)).length

		// Full score if all correct selected and no incorrect selected
		if (selectedCorrect === correctKeywords.length && selectedIncorrect === 0) {
			setScore((prev) => prev + 1)
		}
	}, [selectedKeywords, currentQuestion])

	const handleNext = useCallback(() => {
		if (currentIndex < questions.length - 1) {
			setCurrentIndex((prev) => prev + 1)
			setSelectedKeywords(new Set())
			setShowResult(false)
		} else {
			setFinished(true)
		}
	}, [currentIndex, questions.length])

	const handleRestart = useCallback(() => {
		setCurrentIndex(0)
		setSelectedKeywords(new Set())
		setShowResult(false)
		setScore(0)
		setFinished(false)
	}, [])

	// Finished screen
	if (finished) {
		const percentage = Math.round((score / questions.length) * 100)
		return (
			<div className="flex flex-col items-center justify-center py-12 space-y-6">
				<Trophy className="h-16 w-16 text-yellow-500" />
				<h2 className="text-2xl font-bold">Game Complete!</h2>
				<div className="text-center">
					<p className="text-4xl font-bold text-primary">{score}/{questions.length}</p>
					<p className="text-muted-foreground mt-1">{percentage}% correct</p>
				</div>
				<div className="flex gap-3 pt-4">
					<Button variant="outline" onClick={onBack}>
						Back to Sets
					</Button>
					<Button onClick={handleRestart} className="gap-2">
						<RotateCcw className="h-4 w-4" />
						Play Again
					</Button>
				</div>
			</div>
		)
	}

	if (!currentQuestion) {
		return <div>No questions available</div>
	}

	const correctKeywords = currentQuestion.keywords.filter(k => k.is_correct)
	const selectedCorrect = correctKeywords.filter(k => selectedKeywords.has(k.id)).length
	const selectedIncorrect = currentQuestion.keywords.filter(k => !k.is_correct && selectedKeywords.has(k.id)).length
	const isPerfect = showResult && selectedCorrect === correctKeywords.length && selectedIncorrect === 0

	return (
		<div className="space-y-6">
			{/* Header */}
			<div className="flex items-center justify-between">
				<h3 className="font-semibold">{title}</h3>
				<span className="text-sm text-muted-foreground">
					Question {currentIndex + 1} of {questions.length}
				</span>
			</div>

			{/* Progress bar */}
			<div className="h-2 bg-border/30 rounded-full overflow-hidden">
				<div
					className="h-full bg-primary transition-all duration-300"
					style={{ width: `${((currentIndex + 1) / questions.length) * 100}%` }}
				/>
			</div>

			{/* Statement */}
			<div className="p-6 rounded-xl bg-card/60 border border-border/50">
				<p className="text-lg font-medium text-center">{currentQuestion.statement}</p>
			</div>

			{/* Instructions */}
			<div className="text-center text-sm text-muted-foreground">
				<Tags className="inline h-4 w-4 mr-1" />
				Select all keywords that correctly relate to the statement ({correctKeywords.length} correct)
			</div>

			{/* Keywords Grid */}
			<div className="flex flex-wrap gap-3 justify-center">
				{currentQuestion.keywords.map((keyword) => {
					const isSelected = selectedKeywords.has(keyword.id)
					const isCorrect = keyword.is_correct

					let buttonClass = "px-4 py-2 rounded-lg border-2 transition-all font-medium "

					if (!showResult) {
						// Before submission
						buttonClass += isSelected
							? "border-primary bg-primary/20 text-primary"
							: "border-border/50 bg-card/40 hover:border-primary/50 text-foreground"
					} else {
						// After submission
						if (isCorrect && isSelected) {
							// Correct and selected = green
							buttonClass += "border-green-500 bg-green-500/20 text-green-500"
						} else if (isCorrect && !isSelected) {
							// Correct but not selected = show as missed (yellow)
							buttonClass += "border-yellow-500 bg-yellow-500/10 text-yellow-500"
						} else if (!isCorrect && isSelected) {
							// Incorrect but selected = red
							buttonClass += "border-red-500 bg-red-500/20 text-red-500"
						} else {
							// Incorrect and not selected = faded
							buttonClass += "border-border/30 bg-card/20 text-muted-foreground opacity-60"
						}
					}

					return (
						<button
							key={keyword.id}
							onClick={() => handleKeywordToggle(keyword.id)}
							disabled={showResult}
							className={buttonClass}
						>
							{keyword.word}
							{showResult && isCorrect && isSelected && (
								<CheckCircle className="inline ml-2 h-4 w-4" />
							)}
							{showResult && !isCorrect && isSelected && (
								<XCircle className="inline ml-2 h-4 w-4" />
							)}
						</button>
					)
				})}
			</div>

			{/* Result indicator */}
			{showResult && (
				<div className={`p-4 rounded-xl text-center ${isPerfect ? "bg-green-500/10 border border-green-500/20" : "bg-red-500/10 border border-red-500/20"}`}>
					<p className={`font-semibold ${isPerfect ? "text-green-500" : "text-red-500"}`}>
						{isPerfect ? "Perfect! All correct keywords selected!" : `${selectedCorrect}/${correctKeywords.length} correct keywords (${selectedIncorrect} wrong selections)`}
					</p>
				</div>
			)}

			{/* Explanation */}
			{showResult && (
				<div className="p-4 rounded-xl bg-accent/10 border border-accent/20">
					<p className="text-sm font-medium text-accent mb-1">Explanation</p>
					<p className="text-sm text-muted-foreground">{currentQuestion.explanation}</p>
				</div>
			)}

			{/* Actions */}
			<div className="flex gap-3 pt-2">
				<Button variant="outline" onClick={onBack}>
					Exit Game
				</Button>
				{!showResult ? (
					<Button onClick={handleSubmit} disabled={selectedKeywords.size === 0} className="flex-1">
						Check Answer
					</Button>
				) : (
					<Button onClick={handleNext} className="flex-1 gap-2">
						{currentIndex < questions.length - 1 ? (
							<>
								Next Question
								<ArrowRight className="h-4 w-4" />
							</>
						) : (
							<>
								See Results
								<Trophy className="h-4 w-4" />
							</>
						)}
					</Button>
				)}
			</div>

			{/* Score */}
			<div className="text-center text-sm text-muted-foreground">
				Current score: {score}/{currentIndex + (showResult ? 1 : 0)}
			</div>
		</div>
	)
}
