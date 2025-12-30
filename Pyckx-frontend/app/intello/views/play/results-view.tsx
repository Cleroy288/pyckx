"use client"

import { ArrowLeft, CheckCircle2, XCircle, Trophy, RotateCcw } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useIntello } from "../../context"

export function ResultsView() {
	const {
		selectedQcmSet, answers, score, setView, handlePlayAgain, getScoreColor
	} = useIntello()

	if (!selectedQcmSet) return null
	const scoreColor = getScoreColor(score.percentage)

	return (
		<div className="space-y-6 max-w-3xl mx-auto">
			<div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-8 text-center space-y-4">
				<div className="flex justify-center">
					<div className={`flex h-20 w-20 items-center justify-center rounded-full ${score.percentage >= 60 ? 'bg-green-500/20 border-2 border-green-500/30' : 'bg-red-500/20 border-2 border-red-500/30'}`}>
						<Trophy className={`h-10 w-10 ${score.percentage >= 60 ? 'text-green-500' : 'text-red-500'}`} />
					</div>
				</div>
				<div>
					<h2 className="text-2xl font-bold text-foreground mb-1">Quiz Complete!</h2>
					<p className="text-muted-foreground">{selectedQcmSet.name}</p>
				</div>
				<div className={`text-5xl font-bold ${scoreColor}`}>{score.percentage}%</div>
				<p className="text-lg text-muted-foreground">
					You got <span className="font-semibold text-foreground">{score.correct}</span> out of <span className="font-semibold text-foreground">{score.total}</span> questions correct
				</p>
				<div className="flex gap-3 justify-center pt-4">
					<Button variant="outline" onClick={() => setView("play-qcm")} className="gap-2"><ArrowLeft className="h-4 w-4" />Back to Quizzes</Button>
					<Button onClick={handlePlayAgain} className="gap-2 bg-accent text-accent-foreground hover:bg-accent/90"><RotateCcw className="h-4 w-4" />Play Again</Button>
				</div>
			</div>

			<div className="space-y-4">
				<h3 className="text-lg font-semibold text-foreground">Review Your Answers</h3>
				{answers.map((answer, index) => {
					const question = selectedQcmSet.questions[answer.questionIndex]
					return (
						<div key={index} className={`rounded-xl border p-4 ${answer.isCorrect ? 'border-green-500/30 bg-green-500/5' : 'border-red-500/30 bg-red-500/5'}`}>
							<div className="flex items-start gap-3 mb-3">
								<div className={`flex h-6 w-6 items-center justify-center rounded-full shrink-0 ${answer.isCorrect ? 'bg-green-500/20' : 'bg-red-500/20'}`}>
									{answer.isCorrect ? <CheckCircle2 className="h-4 w-4 text-green-500" /> : <XCircle className="h-4 w-4 text-red-500" />}
								</div>
								<div className="flex-1">
									<p className="font-medium text-foreground mb-2">Q{index + 1}: {question.question}</p>
									<div className="space-y-1 text-sm">
										<p className="text-muted-foreground">Your answer: <span className={answer.isCorrect ? 'text-green-500' : 'text-red-500'}>{answer.selectedAnswer}</span></p>
										{!answer.isCorrect && <p className="text-muted-foreground">Correct answer: <span className="text-green-500">{answer.correctAnswer}</span></p>}
									</div>
								</div>
							</div>
							<div className="pl-9">
								<p className="text-sm text-muted-foreground bg-background/30 rounded-lg p-3"><span className="font-medium text-foreground/80">Explanation:</span> {answer.explanation}</p>
							</div>
						</div>
					)
				})}
			</div>
		</div>
	)
}
