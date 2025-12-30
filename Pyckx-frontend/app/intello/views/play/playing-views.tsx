"use client"

import { ArrowLeft, ArrowRight, CheckCircle2, XCircle, Trophy } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { OpenQuestionPlayer } from "@/components/intello/open-question-player"
import { FlashcardPlayer } from "@/components/intello/flashcard-player"
import { TrueOrFalsePlayer } from "@/components/intello/true-false-player"
import { KeywordsPlayer } from "@/components/intello/keywords-player"
import { OrderPhrasePlayer } from "@/components/intello/order-phrase-player"
import { FillBlankPlayer } from "@/components/intello/fill-blank-player"
import { useIntello } from "../../context"

// == PLAYING QCM VIEW ==
export function PlayingQcmView() {
	const {
		selectedQcmSet, currentQuestionIndex, selectedAnswer, hasAnswered, answers,
		shuffledAnswersRef, setView, handleAnswerSelect, handleNextQuestion, getLevelBadgeClass
	} = useIntello()

	if (!selectedQcmSet) return null

	const currentQuestion = selectedQcmSet.questions[currentQuestionIndex]
	const progress = ((currentQuestionIndex + 1) / selectedQcmSet.questions.length) * 100
	const currentAnswer = answers.find(a => a.questionIndex === currentQuestionIndex)

	return (
		<div className="space-y-6 max-w-3xl mx-auto">
			<div className="flex items-center justify-between">
				<Button variant="ghost" size="sm" onClick={() => setView("play-qcm")} className="gap-2">
					<ArrowLeft className="h-4 w-4" />Exit Quiz
				</Button>
				<div className="text-sm text-muted-foreground">Question {currentQuestionIndex + 1} of {selectedQcmSet.questions.length}</div>
			</div>

			<div className="space-y-2">
				<Progress value={progress} className="h-2" />
				<div className="flex justify-between text-xs text-muted-foreground">
					<span>{selectedQcmSet.name}</span>
					<span className={getLevelBadgeClass(selectedQcmSet.level).replace('bg-', 'text-').split(' ')[0]}>{selectedQcmSet.level}</span>
				</div>
			</div>

			<div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-6 space-y-6">
				<h2 className="text-xl font-semibold text-foreground leading-relaxed">{currentQuestion.question}</h2>

				<div className="grid gap-3">
					{shuffledAnswersRef.current.map((answer, index) => {
						const isSelected = selectedAnswer === answer
						const isCorrect = answer === currentQuestion.right_answer
						const showResult = hasAnswered
						let buttonClass = "w-full p-4 rounded-xl border text-left transition-all "
						if (!showResult) {
							buttonClass += isSelected ? "border-accent bg-accent/20 text-foreground cursor-pointer" : "border-border/50 bg-card/40 hover:border-accent/50 hover:bg-accent/10 text-foreground cursor-pointer"
						} else if (isCorrect) {
							buttonClass += "border-green-500 bg-green-500/20 text-green-500"
						} else if (isSelected && !isCorrect) {
							buttonClass += "border-red-500 bg-red-500/20 text-red-500"
						} else {
							buttonClass += "border-border/30 bg-card/20 text-muted-foreground opacity-60"
						}
						return (
							<button key={index} onClick={() => handleAnswerSelect(answer)} disabled={hasAnswered} className={buttonClass}>
								<div className="flex items-center gap-3">
									<span className="flex h-8 w-8 items-center justify-center rounded-lg bg-background/50 text-sm font-medium shrink-0">{String.fromCharCode(65 + index)}</span>
									<span className="flex-1">{answer}</span>
									{showResult && isCorrect && <CheckCircle2 className="h-5 w-5 text-green-500 shrink-0" />}
									{showResult && isSelected && !isCorrect && <XCircle className="h-5 w-5 text-red-500 shrink-0" />}
								</div>
							</button>
						)
					})}
				</div>

				{hasAnswered && (
					<div className={`rounded-xl p-4 ${currentAnswer?.isCorrect ? 'bg-green-500/10 border border-green-500/20' : 'bg-red-500/10 border border-red-500/20'}`}>
						<div className="flex items-center gap-2 mb-2">
							{currentAnswer?.isCorrect ? (<><CheckCircle2 className="h-5 w-5 text-green-500" /><span className="font-semibold text-green-500">Correct!</span></>) : (<><XCircle className="h-5 w-5 text-red-500" /><span className="font-semibold text-red-500">Incorrect</span></>)}
						</div>
						<p className="text-sm text-muted-foreground">{currentQuestion.explanation}</p>
					</div>
				)}

				{hasAnswered && (
					<Button onClick={handleNextQuestion} className="w-full gap-2 bg-accent text-accent-foreground hover:bg-accent/90">
						{currentQuestionIndex + 1 >= selectedQcmSet.questions.length ? (<><Trophy className="h-4 w-4" />See Results</>) : (<>Next Question<ArrowRight className="h-4 w-4" /></>)}
					</Button>
				)}
			</div>
		</div>
	)
}

// == PLAYING OPEN QUESTIONS VIEW ==
export function PlayingOpenView() {
	const { selectedOpenSet, setView } = useIntello()

	if (!selectedOpenSet) return null

	return (
		<OpenQuestionPlayer
			questions={selectedOpenSet.questions}
			title={selectedOpenSet.name}
			setId={selectedOpenSet.id}
			onBack={() => setView("play-open")}
		/>
	)
}

// == PLAYING FLASHCARDS VIEW ==
export function PlayingFlashcardView() {
	const { selectedFlashcardSet, setView } = useIntello()

	if (!selectedFlashcardSet) return null

	return (
		<FlashcardPlayer
			cards={selectedFlashcardSet.cards}
			title={selectedFlashcardSet.name}
			onBack={() => setView("play-flashcard")}
		/>
	)
}

// == PLAYING TRUE OR FALSE VIEW ==
export function PlayingTrueOrFalseView() {
	const { selectedTrueOrFalseSet, setView } = useIntello()

	if (!selectedTrueOrFalseSet) return null

	return (
		<TrueOrFalsePlayer
			statements={selectedTrueOrFalseSet.statements}
			title={selectedTrueOrFalseSet.name}
			onBack={() => setView("play-true-false")}
		/>
	)
}

// == PLAYING KEYWORDS VIEW ==
export function PlayingKeywordsView() {
	const { selectedKeywordSet, setView } = useIntello()

	if (!selectedKeywordSet) return null

	return (
		<KeywordsPlayer
			questions={selectedKeywordSet.questions}
			title={selectedKeywordSet.name}
			onBack={() => setView("play-keywords")}
		/>
	)
}

// == PLAYING ORDER PHRASE VIEW ==
export function PlayingOrderPhraseView() {
	const { selectedOrderPhraseSet, setView } = useIntello()

	if (!selectedOrderPhraseSet) return null

	return (
		<OrderPhrasePlayer
			questions={selectedOrderPhraseSet.questions}
			title={selectedOrderPhraseSet.name}
			onBack={() => setView("play-order-phrase")}
		/>
	)
}

// == PLAYING FILL BLANK VIEW ==
export function PlayingFillBlankView() {
	const { selectedFillBlankSet, setView } = useIntello()

	if (!selectedFillBlankSet) return null

	return (
		<FillBlankPlayer
			questions={selectedFillBlankSet.questions}
			title={selectedFillBlankSet.name}
			onBack={() => setView("play-fill-blank")}
		/>
	)
}
