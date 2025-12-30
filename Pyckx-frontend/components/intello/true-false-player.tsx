"use client"

import { useState, useCallback } from "react"
import { CheckCircle, XCircle, ArrowRight, RotateCcw, Trophy, ThumbsUp, ThumbsDown } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { TrueOrFalseStatement } from "@/lib/api/intello"

interface TrueOrFalsePlayerProps {
	statements: TrueOrFalseStatement[]
	title: string
	onBack: () => void
}

export function TrueOrFalsePlayer({ statements, title, onBack }: TrueOrFalsePlayerProps) {
	const [currentIndex, setCurrentIndex] = useState(0)
	const [selectedAnswer, setSelectedAnswer] = useState<boolean | null>(null)
	const [showResult, setShowResult] = useState(false)
	const [score, setScore] = useState(0)
	const [finished, setFinished] = useState(false)

	const currentStatement = statements[currentIndex]

	const handleAnswerSelect = useCallback((answer: boolean) => {
		if (showResult) return
		setSelectedAnswer(answer)
	}, [showResult])

	const handleSubmit = useCallback(() => {
		if (selectedAnswer === null || !currentStatement) return
		setShowResult(true)
		if (selectedAnswer === currentStatement.answer) {
			setScore((prev) => prev + 1)
		}
	}, [selectedAnswer, currentStatement])

	const handleNext = useCallback(() => {
		if (currentIndex < statements.length - 1) {
			setCurrentIndex((prev) => prev + 1)
			setSelectedAnswer(null)
			setShowResult(false)
		} else {
			setFinished(true)
		}
	}, [currentIndex, statements.length])

	const handleRestart = useCallback(() => {
		setCurrentIndex(0)
		setSelectedAnswer(null)
		setShowResult(false)
		setScore(0)
		setFinished(false)
	}, [])

	const isCorrect = showResult && selectedAnswer === currentStatement?.answer

	// Finished screen
	if (finished) {
		const percentage = Math.round((score / statements.length) * 100)
		return (
			<div className="flex flex-col items-center justify-center py-12 space-y-6">
				<Trophy className="h-16 w-16 text-yellow-500" />
				<h2 className="text-2xl font-bold">Game Complete!</h2>
				<div className="text-center">
					<p className="text-4xl font-bold text-primary">{score}/{statements.length}</p>
					<p className="text-muted-foreground mt-1">{percentage}% correct</p>
				</div>
				<div className="flex gap-3 pt-4">
					<Button variant="outline" onClick={onBack}>
						Back to Form
					</Button>
					<Button onClick={handleRestart} className="gap-2">
						<RotateCcw className="h-4 w-4" />
						Play Again
					</Button>
				</div>
			</div>
		)
	}

	if (!currentStatement) {
		return <div>No statements available</div>
	}

	return (
		<div className="space-y-6">
			{/* Header */}
			<div className="flex items-center justify-between">
				<h3 className="font-semibold">{title}</h3>
				<span className="text-sm text-muted-foreground">
					Statement {currentIndex + 1} of {statements.length}
				</span>
			</div>

			{/* Progress bar */}
			<div className="h-2 bg-border/30 rounded-full overflow-hidden">
				<div
					className="h-full bg-primary transition-all duration-300"
					style={{ width: `${((currentIndex + 1) / statements.length) * 100}%` }}
				/>
			</div>

			{/* Statement */}
			<div className="p-6 rounded-xl bg-card/60 border border-border/50">
				<p className="text-lg font-medium text-center">{currentStatement.statement}</p>
			</div>

			{/* True/False Buttons */}
			<div className="flex gap-4 justify-center">
				<button
					onClick={() => handleAnswerSelect(true)}
					disabled={showResult}
					className={`flex-1 max-w-[200px] p-6 rounded-xl border-2 transition-all flex flex-col items-center gap-3
            ${!showResult && selectedAnswer === true ? "border-green-500 bg-green-500/10" : ""}
            ${!showResult && selectedAnswer !== true ? "border-border/50 hover:border-green-500/50" : ""}
            ${showResult && currentStatement.answer === true ? "border-green-500 bg-green-500/10" : ""}
            ${showResult && selectedAnswer === true && currentStatement.answer !== true ? "border-red-500 bg-red-500/10" : ""}
            ${showResult && selectedAnswer !== true && currentStatement.answer !== true ? "border-border/30 opacity-50" : ""}
          `}
				>
					<ThumbsUp className={`h-8 w-8 ${selectedAnswer === true ? "text-green-500" : "text-muted-foreground"}`} />
					<span className="font-semibold">TRUE</span>
					{showResult && currentStatement.answer === true && (
						<CheckCircle className="h-5 w-5 text-green-500" />
					)}
					{showResult && selectedAnswer === true && currentStatement.answer !== true && (
						<XCircle className="h-5 w-5 text-red-500" />
					)}
				</button>

				<button
					onClick={() => handleAnswerSelect(false)}
					disabled={showResult}
					className={`flex-1 max-w-[200px] p-6 rounded-xl border-2 transition-all flex flex-col items-center gap-3
            ${!showResult && selectedAnswer === false ? "border-red-500 bg-red-500/10" : ""}
            ${!showResult && selectedAnswer !== false ? "border-border/50 hover:border-red-500/50" : ""}
            ${showResult && currentStatement.answer === false ? "border-green-500 bg-green-500/10" : ""}
            ${showResult && selectedAnswer === false && currentStatement.answer !== false ? "border-red-500 bg-red-500/10" : ""}
            ${showResult && selectedAnswer !== false && currentStatement.answer !== false ? "border-border/30 opacity-50" : ""}
          `}
				>
					<ThumbsDown className={`h-8 w-8 ${selectedAnswer === false ? "text-red-500" : "text-muted-foreground"}`} />
					<span className="font-semibold">FALSE</span>
					{showResult && currentStatement.answer === false && (
						<CheckCircle className="h-5 w-5 text-green-500" />
					)}
					{showResult && selectedAnswer === false && currentStatement.answer !== false && (
						<XCircle className="h-5 w-5 text-red-500" />
					)}
				</button>
			</div>

			{/* Result indicator */}
			{showResult && (
				<div className={`p-4 rounded-xl text-center ${isCorrect ? "bg-green-500/10 border border-green-500/20" : "bg-red-500/10 border border-red-500/20"}`}>
					<p className={`font-semibold ${isCorrect ? "text-green-500" : "text-red-500"}`}>
						{isCorrect ? "Correct!" : "Incorrect!"}
					</p>
				</div>
			)}

			{/* Explanation */}
			{showResult && (
				<div className="p-4 rounded-xl bg-accent/10 border border-accent/20">
					<p className="text-sm font-medium text-accent mb-1">Explanation</p>
					<p className="text-sm text-muted-foreground">{currentStatement.explanation}</p>
				</div>
			)}

			{/* Actions */}
			<div className="flex gap-3 pt-2">
				<Button variant="outline" onClick={onBack}>
					Exit Game
				</Button>
				{!showResult ? (
					<Button onClick={handleSubmit} disabled={selectedAnswer === null} className="flex-1">
						Check Answer
					</Button>
				) : (
					<Button onClick={handleNext} className="flex-1 gap-2">
						{currentIndex < statements.length - 1 ? (
							<>
								Next Statement
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
