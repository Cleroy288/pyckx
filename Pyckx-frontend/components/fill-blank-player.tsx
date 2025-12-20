"use client"

import { useState, useCallback } from "react"
import { CheckCircle, XCircle, ArrowRight, RotateCcw, Trophy, TextCursor } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { FillBlankQuestionData, FillBlankOptionData } from "@/lib/api/intello"

interface FillBlankPlayerProps {
	questions: FillBlankQuestionData[]
	title: string
	onBack: () => void
}

export function FillBlankPlayer({ questions, title, onBack }: FillBlankPlayerProps) {
	const [currentIndex, setCurrentIndex] = useState(0)
	const [selectedOption, setSelectedOption] = useState<string | null>(null)
	const [showResult, setShowResult] = useState(false)
	const [score, setScore] = useState(0)
	const [finished, setFinished] = useState(false)

	const currentQuestion = questions[currentIndex]

	const handleOptionSelect = useCallback((optionId: string) => {
		if (showResult) return
		setSelectedOption(optionId)
	}, [showResult])

	const handleSubmit = useCallback(() => {
		if (selectedOption === null || !currentQuestion) return
		setShowResult(true)

		// Check if selected option is correct
		const selectedOpt = currentQuestion.options.find(o => o.id === selectedOption)
		if (selectedOpt?.is_correct) {
			setScore((prev) => prev + 1)
		}
	}, [selectedOption, currentQuestion])

	const handleNext = useCallback(() => {
		if (currentIndex < questions.length - 1) {
			setCurrentIndex((prev) => prev + 1)
			setSelectedOption(null)
			setShowResult(false)
		} else {
			setFinished(true)
		}
	}, [currentIndex, questions.length])

	const handleRestart = useCallback(() => {
		setCurrentIndex(0)
		setSelectedOption(null)
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

	const correctOption = currentQuestion.options.find(o => o.is_correct)
	const selectedOpt = currentQuestion.options.find(o => o.id === selectedOption)
	const isCorrect = showResult && selectedOpt?.is_correct

	// Replace blank with placeholder
	const phraseWithBlank = currentQuestion.phrase.replace(/___+/g, "______")

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

			{/* Phrase with blank */}
			<div className="p-6 rounded-xl bg-card/60 border border-border/50">
				<p className="text-lg font-medium text-center">
					{phraseWithBlank}
				</p>
			</div>

			{/* Instructions */}
			<div className="text-center text-sm text-muted-foreground">
				<TextCursor className="inline h-4 w-4 mr-1" />
				Select the correct word to fill in the blank
			</div>

			{/* Options Grid */}
			<div className="grid gap-3 sm:grid-cols-2">
				{currentQuestion.options.map((option) => {
					const isSelected = selectedOption === option.id
					const optIsCorrect = option.is_correct

					let buttonClass = "w-full p-4 rounded-xl border-2 transition-all font-medium text-left flex items-center gap-3 "

					if (!showResult) {
						// Before submission
						buttonClass += isSelected
							? "border-primary bg-primary/20 text-primary"
							: "border-border/50 bg-card/40 hover:border-primary/50 text-foreground"
					} else {
						// After submission
						if (optIsCorrect) {
							// Correct option = green
							buttonClass += "border-green-500 bg-green-500/20 text-green-500"
						} else if (isSelected && !optIsCorrect) {
							// Incorrect but selected = red
							buttonClass += "border-red-500 bg-red-500/20 text-red-500"
						} else {
							// Incorrect and not selected = faded
							buttonClass += "border-border/30 bg-card/20 text-muted-foreground opacity-60"
						}
					}

					return (
						<button
							key={option.id}
							onClick={() => handleOptionSelect(option.id)}
							disabled={showResult}
							className={buttonClass}
						>
							<span className="flex-1">{option.text}</span>
							{showResult && optIsCorrect && (
								<CheckCircle className="h-5 w-5 shrink-0" />
							)}
							{showResult && isSelected && !optIsCorrect && (
								<XCircle className="h-5 w-5 shrink-0" />
							)}
						</button>
					)
				})}
			</div>

			{/* Result indicator */}
			{showResult && (
				<div className={`p-4 rounded-xl text-center ${isCorrect ? "bg-green-500/10 border border-green-500/20" : "bg-red-500/10 border border-red-500/20"}`}>
					<p className={`font-semibold ${isCorrect ? "text-green-500" : "text-red-500"}`}>
						{isCorrect ? "Correct!" : `Incorrect! The answer was: ${correctOption?.text}`}
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
					<Button onClick={handleSubmit} disabled={selectedOption === null} className="flex-1">
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
