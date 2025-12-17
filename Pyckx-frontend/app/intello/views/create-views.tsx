"use client"

import { ArrowLeft, Brain, Sparkles } from "lucide-react"
import { Button } from "@/components/ui/button"
import { CustomQuestionForm } from "@/components/custom-question-form"
import { useIntello } from "../context"

// == MANUAL QCM VIEW ==
export function CreateQcmView() {
	const { handleBackToHome } = useIntello()

	return (
		<div className="space-y-6 max-w-2xl mx-auto">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
						<Brain className="h-6 w-6 text-accent" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">Manual QCM</h1>
						<p className="text-sm text-muted-foreground">Create questions manually</p>
					</div>
				</div>
			</div>
			<div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-6">
				<p className="text-muted-foreground text-center py-8">Manual QCM creation coming soon...</p>
				<Button onClick={handleBackToHome} className="w-full">Back to Home</Button>
			</div>
		</div>
	)
}

// == AI QCM VIEW ==
export function CreateAiQcmView() {
	const { games, handleBackToHome, handleNavigateToGame } = useIntello()

	return (
		<div className="space-y-6 max-w-2xl mx-auto">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<Sparkles className="h-6 w-6 text-primary" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">AI QCM</h1>
						<p className="text-sm text-muted-foreground">Generate QCM from your documents</p>
					</div>
				</div>
			</div>
			<div className="rounded-2xl border border-primary/30 bg-card/60 backdrop-blur-sm p-6">
				<CustomQuestionForm games={games} onBack={handleBackToHome} defaultOutputGame="qcm" onNavigateToGame={handleNavigateToGame} />
			</div>
		</div>
	)
}

// == AI OPEN QUESTIONS VIEW ==
export function CreateAiOpenView() {
	const { games, handleBackToHome, handleNavigateToGame } = useIntello()

	return (
		<div className="space-y-6 max-w-2xl mx-auto">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<Sparkles className="h-6 w-6 text-primary" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">AI Open Questions</h1>
						<p className="text-sm text-muted-foreground">Generate open-ended questions from your documents</p>
					</div>
				</div>
			</div>
			<div className="rounded-2xl border border-primary/30 bg-card/60 backdrop-blur-sm p-6">
				<CustomQuestionForm games={games} onBack={handleBackToHome} defaultOutputGame="open_question" onNavigateToGame={handleNavigateToGame} />
			</div>
		</div>
	)
}

// == AI FLASHCARDS VIEW ==
export function CreateAiFlashcardView() {
	const { games, handleBackToHome, handleNavigateToGame } = useIntello()

	return (
		<div className="space-y-6 max-w-2xl mx-auto">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<Sparkles className="h-6 w-6 text-primary" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">AI Flashcards</h1>
						<p className="text-sm text-muted-foreground">Generate flashcards from your documents</p>
					</div>
				</div>
			</div>
			<div className="rounded-2xl border border-primary/30 bg-card/60 backdrop-blur-sm p-6">
				<CustomQuestionForm games={games} onBack={handleBackToHome} defaultOutputGame="flashcard" onNavigateToGame={handleNavigateToGame} />
			</div>
		</div>
	)
}

// == AI TRUE OR FALSE VIEW ==
export function CreateAiTrueOrFalseView() {
	const { games, handleBackToHome, handleNavigateToGame } = useIntello()

	return (
		<div className="space-y-6 max-w-2xl mx-auto">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<Sparkles className="h-6 w-6 text-primary" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">AI True or False</h1>
						<p className="text-sm text-muted-foreground">Generate true/false statements from your documents</p>
					</div>
				</div>
			</div>
			<div className="rounded-2xl border border-primary/30 bg-card/60 backdrop-blur-sm p-6">
				<CustomQuestionForm games={games} onBack={handleBackToHome} defaultOutputGame="true_false" onNavigateToGame={handleNavigateToGame} />
			</div>
		</div>
	)
}

// == AI KEYWORDS VIEW ==
export function CreateAiKeywordsView() {
	const { games, handleBackToHome, handleNavigateToGame } = useIntello()

	return (
		<div className="space-y-6 max-w-2xl mx-auto">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<Sparkles className="h-6 w-6 text-primary" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">AI Keywords</h1>
						<p className="text-sm text-muted-foreground">Generate keyword recognition exercises from your documents</p>
					</div>
				</div>
			</div>
			<div className="rounded-2xl border border-primary/30 bg-card/60 backdrop-blur-sm p-6">
				<CustomQuestionForm games={games} onBack={handleBackToHome} defaultOutputGame="keywords" onNavigateToGame={handleNavigateToGame} />
			</div>
		</div>
	)
}

// == AI ORDER PHRASE VIEW ==
export function CreateAiOrderPhraseView() {
	const { games, handleBackToHome, handleNavigateToGame } = useIntello()

	return (
		<div className="space-y-6 max-w-2xl mx-auto">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<Sparkles className="h-6 w-6 text-primary" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">AI Order Phrase</h1>
						<p className="text-sm text-muted-foreground">Generate phrase ordering exercises from your documents</p>
					</div>
				</div>
			</div>
			<div className="rounded-2xl border border-primary/30 bg-card/60 backdrop-blur-sm p-6">
				<CustomQuestionForm games={games} onBack={handleBackToHome} defaultOutputGame="order_phrase" onNavigateToGame={handleNavigateToGame} />
			</div>
		</div>
	)
}

