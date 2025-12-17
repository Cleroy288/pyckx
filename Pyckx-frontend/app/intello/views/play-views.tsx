"use client"

import { ArrowLeft, Brain, FileText, Layers, CheckCircle2, BookOpen, Play, Loader2, Tags, ListOrdered } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useIntello } from "../context"

// == PLAY QCM VIEW ==
export function PlayQcmView() {
	const {
		loading, error, qcmSets, setView,
		handleBackToHome, handleSelectQcmSet, loadQcmSets, getLevelBadgeClass
	} = useIntello()

	return (
		<div className="space-y-6">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
						<Brain className="h-6 w-6 text-accent" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">QCM Sets</h1>
						<p className="text-sm text-muted-foreground">Select a quiz to play</p>
					</div>
				</div>
			</div>

			{loading && (
				<div className="flex items-center justify-center py-16">
					<Loader2 className="h-8 w-8 animate-spin text-accent" />
				</div>
			)}

			{error && (
				<div className="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
					<p className="text-sm text-destructive">{error}</p>
					<Button variant="link" onClick={loadQcmSets} className="text-destructive p-0 h-auto mt-2">Try again</Button>
				</div>
			)}

			{!loading && !error && qcmSets.length === 0 && (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-16">
					<BookOpen className="h-12 w-12 text-muted-foreground mb-4" />
					<h3 className="text-lg font-semibold text-foreground mb-2">No QCM sets yet</h3>
					<p className="text-sm text-muted-foreground mb-4">Create some QCM sets first!</p>
					<Button onClick={() => setView("create-ai-qcm")}>Create with AI</Button>
				</div>
			)}

			{!loading && !error && qcmSets.length > 0 && (
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{qcmSets.map((set) => (
						<button
							key={set.id}
							onClick={() => handleSelectQcmSet(set)}
							disabled={set.questions.length === 0}
							className="group rounded-xl border border-accent/30 bg-accent/10 p-5 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
						>
							<div className="flex items-start justify-between mb-3">
								<h3 className="font-semibold text-foreground line-clamp-1">{set.name}</h3>
								<div className="flex items-center gap-1.5 shrink-0 ml-2">
									<span className="text-xs px-2 py-0.5 rounded-full border bg-blue-500/10 text-blue-500 border-blue-500/20 uppercase">{set.language}</span>
									<span className={`text-xs px-2 py-0.5 rounded-full border ${getLevelBadgeClass(set.level)}`}>{set.level}</span>
								</div>
							</div>
							<p className="text-sm text-muted-foreground mb-3 line-clamp-2">{set.description}</p>
							{set.subjects.length > 0 && (
								<div className="flex flex-wrap gap-1 mb-3">
									{set.subjects.slice(0, 3).map((subject, i) => (
										<span key={i} className="inline-block px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary border border-primary/20">{subject}</span>
									))}
								</div>
							)}
							<div className="flex items-center justify-between pt-3 border-t border-border/30">
								<div className="flex items-center gap-2 text-sm text-muted-foreground">
									<BookOpen className="h-4 w-4" />
									<span>{set.questions.length} questions</span>
								</div>
								{set.questions.length > 0 && (
									<div className="flex items-center gap-1 text-accent">
										<Play className="h-4 w-4" />
										<span className="text-sm font-medium">Play</span>
									</div>
								)}
							</div>
						</button>
					))}
				</div>
			)}
		</div>
	)
}

// == PLAY OPEN QUESTIONS VIEW ==
export function PlayOpenView() {
	const {
		loading, error, openQuestionSets, setView,
		handleBackToHome, handleSelectOpenSet, loadOpenQuestionSets, getLevelBadgeClass
	} = useIntello()

	return (
		<div className="space-y-6">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
						<FileText className="h-6 w-6 text-accent" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">Open Question Sets</h1>
						<p className="text-sm text-muted-foreground">Select a set to play</p>
					</div>
				</div>
			</div>

			{loading && (
				<div className="flex items-center justify-center py-16">
					<Loader2 className="h-8 w-8 animate-spin text-accent" />
				</div>
			)}

			{error && (
				<div className="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
					<p className="text-sm text-destructive">{error}</p>
					<Button variant="link" onClick={loadOpenQuestionSets} className="text-destructive p-0 h-auto mt-2">Try again</Button>
				</div>
			)}

			{!loading && !error && openQuestionSets.length === 0 && (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-16">
					<FileText className="h-12 w-12 text-muted-foreground mb-4" />
					<h3 className="text-lg font-semibold text-foreground mb-2">No open question sets yet</h3>
					<p className="text-sm text-muted-foreground mb-4">Create some open question sets first!</p>
					<Button onClick={() => setView("create-ai-open")}>Create with AI</Button>
				</div>
			)}

			{!loading && !error && openQuestionSets.length > 0 && (
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{openQuestionSets.map((set) => (
						<button
							key={set.id}
							onClick={() => handleSelectOpenSet(set)}
							disabled={set.questions.length === 0}
							className="group rounded-xl border border-accent/30 bg-accent/10 p-5 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
						>
							<div className="flex items-start justify-between mb-3">
								<h3 className="font-semibold text-foreground line-clamp-1">{set.name}</h3>
								<div className="flex items-center gap-1.5 shrink-0 ml-2">
									<span className="text-xs px-2 py-0.5 rounded-full border bg-blue-500/10 text-blue-500 border-blue-500/20 uppercase">{set.language}</span>
									<span className={`text-xs px-2 py-0.5 rounded-full border ${getLevelBadgeClass(set.level)}`}>{set.level}</span>
								</div>
							</div>
							<p className="text-sm text-muted-foreground mb-3 line-clamp-2">{set.description}</p>
							{set.subjects.length > 0 && (
								<div className="flex flex-wrap gap-1 mb-3">
									{set.subjects.slice(0, 3).map((subject, i) => (
										<span key={i} className="inline-block px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary border border-primary/20">{subject}</span>
									))}
								</div>
							)}
							<div className="flex items-center justify-between pt-3 border-t border-border/30">
								<div className="flex items-center gap-2 text-sm text-muted-foreground">
									<FileText className="h-4 w-4" />
									<span>{set.questions.length} questions</span>
								</div>
								{set.questions.length > 0 && (
									<div className="flex items-center gap-1 text-accent">
										<Play className="h-4 w-4" />
										<span className="text-sm font-medium">Play</span>
									</div>
								)}
							</div>
						</button>
					))}
				</div>
			)}
		</div>
	)
}

// == PLAY FLASHCARD VIEW ==
export function PlayFlashcardView() {
	const {
		loading, error, flashcardSets, setView,
		handleBackToHome, handleSelectFlashcardSet, loadFlashcardSets, getLevelBadgeClass
	} = useIntello()

	return (
		<div className="space-y-6">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
						<Layers className="h-6 w-6 text-accent" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">Flashcard Sets</h1>
						<p className="text-sm text-muted-foreground">Select a set to study</p>
					</div>
				</div>
			</div>

			{loading && (
				<div className="flex items-center justify-center py-16">
					<Loader2 className="h-8 w-8 animate-spin text-accent" />
				</div>
			)}

			{error && (
				<div className="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
					<p className="text-sm text-destructive">{error}</p>
					<Button variant="link" onClick={loadFlashcardSets} className="text-destructive p-0 h-auto mt-2">Try again</Button>
				</div>
			)}

			{!loading && !error && flashcardSets.length === 0 && (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-16">
					<Layers className="h-12 w-12 text-muted-foreground mb-4" />
					<h3 className="text-lg font-semibold text-foreground mb-2">No flashcard sets yet</h3>
					<p className="text-sm text-muted-foreground mb-4">Create some flashcard sets first!</p>
					<Button onClick={() => setView("create-ai-flashcard")}>Create with AI</Button>
				</div>
			)}

			{!loading && !error && flashcardSets.length > 0 && (
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{flashcardSets.map((set) => (
						<button
							key={set.id}
							onClick={() => handleSelectFlashcardSet(set)}
							disabled={set.cards.length === 0}
							className="group rounded-xl border border-accent/30 bg-accent/10 p-5 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
						>
							<div className="flex items-start justify-between mb-3">
								<h3 className="font-semibold text-foreground line-clamp-1">{set.name}</h3>
								<div className="flex items-center gap-1.5 shrink-0 ml-2">
									<span className="text-xs px-2 py-0.5 rounded-full border bg-blue-500/10 text-blue-500 border-blue-500/20 uppercase">{set.language}</span>
									<span className={`text-xs px-2 py-0.5 rounded-full border ${getLevelBadgeClass(set.level)}`}>{set.level}</span>
								</div>
							</div>
							<p className="text-sm text-muted-foreground mb-3 line-clamp-2">{set.description}</p>
							{set.subjects.length > 0 && (
								<div className="flex flex-wrap gap-1 mb-3">
									{set.subjects.slice(0, 3).map((subject, i) => (
										<span key={i} className="inline-block px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary border border-primary/20">{subject}</span>
									))}
								</div>
							)}
							<div className="flex items-center justify-between pt-3 border-t border-border/30">
								<div className="flex items-center gap-2 text-sm text-muted-foreground">
									<Layers className="h-4 w-4" />
									<span>{set.cards.length} cards</span>
								</div>
								{set.cards.length > 0 && (
									<div className="flex items-center gap-1 text-accent">
										<Play className="h-4 w-4" />
										<span className="text-sm font-medium">Study</span>
									</div>
								)}
							</div>
						</button>
					))}
				</div>
			)}
		</div>
	)
}

// == PLAY TRUE OR FALSE VIEW ==
export function PlayTrueOrFalseView() {
	const {
		loading, error, trueOrFalseSets, setView,
		handleBackToHome, handleSelectTrueOrFalseSet, loadTrueOrFalseSets, getLevelBadgeClass
	} = useIntello()

	return (
		<div className="space-y-6">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
						<CheckCircle2 className="h-6 w-6 text-accent" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">True or False Sets</h1>
						<p className="text-sm text-muted-foreground">Select a set to play</p>
					</div>
				</div>
			</div>

			{loading && (
				<div className="flex items-center justify-center py-16">
					<Loader2 className="h-8 w-8 animate-spin text-accent" />
				</div>
			)}

			{error && (
				<div className="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
					<p className="text-sm text-destructive">{error}</p>
					<Button variant="link" onClick={loadTrueOrFalseSets} className="text-destructive p-0 h-auto mt-2">Try again</Button>
				</div>
			)}

			{!loading && !error && trueOrFalseSets.length === 0 && (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-16">
					<CheckCircle2 className="h-12 w-12 text-muted-foreground mb-4" />
					<h3 className="text-lg font-semibold text-foreground mb-2">No true/false sets yet</h3>
					<p className="text-sm text-muted-foreground mb-4">Create some true/false sets first!</p>
					<Button onClick={() => setView("create-ai-true-false")}>Create with AI</Button>
				</div>
			)}

			{!loading && !error && trueOrFalseSets.length > 0 && (
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{trueOrFalseSets.map((set) => (
						<button
							key={set.id}
							onClick={() => handleSelectTrueOrFalseSet(set)}
							disabled={set.statements.length === 0}
							className="group rounded-xl border border-accent/30 bg-accent/10 p-5 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
						>
							<div className="flex items-start justify-between mb-3">
								<h3 className="font-semibold text-foreground line-clamp-1">{set.name}</h3>
								<div className="flex items-center gap-1.5 shrink-0 ml-2">
									<span className="text-xs px-2 py-0.5 rounded-full border bg-blue-500/10 text-blue-500 border-blue-500/20 uppercase">{set.language}</span>
									<span className={`text-xs px-2 py-0.5 rounded-full border ${getLevelBadgeClass(set.level)}`}>{set.level}</span>
								</div>
							</div>
							<p className="text-sm text-muted-foreground mb-3 line-clamp-2">{set.description}</p>
							{set.subjects.length > 0 && (
								<div className="flex flex-wrap gap-1 mb-3">
									{set.subjects.slice(0, 3).map((subject, i) => (
										<span key={i} className="inline-block px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary border border-primary/20">{subject}</span>
									))}
								</div>
							)}
							<div className="flex items-center justify-between pt-3 border-t border-border/30">
								<div className="flex items-center gap-2 text-sm text-muted-foreground">
									<CheckCircle2 className="h-4 w-4" />
									<span>{set.statements.length} statements</span>
								</div>
								{set.statements.length > 0 && (
									<div className="flex items-center gap-1 text-accent">
										<Play className="h-4 w-4" />
										<span className="text-sm font-medium">Play</span>
									</div>
								)}
							</div>
						</button>
					))}
				</div>
			)}
		</div>
	)
}

// == PLAY KEYWORDS VIEW ==
export function PlayKeywordsView() {
	const {
		loading, error, keywordSets, setView,
		handleBackToHome, handleSelectKeywordSet, loadKeywordSets, getLevelBadgeClass
	} = useIntello()

	return (
		<div className="space-y-6">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
						<Tags className="h-6 w-6 text-accent" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">Keyword Sets</h1>
						<p className="text-sm text-muted-foreground">Select a set to practice</p>
					</div>
				</div>
			</div>

			{loading && (
				<div className="flex items-center justify-center py-16">
					<Loader2 className="h-8 w-8 animate-spin text-accent" />
				</div>
			)}

			{error && (
				<div className="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
					<p className="text-sm text-destructive">{error}</p>
					<Button variant="link" onClick={loadKeywordSets} className="text-destructive p-0 h-auto mt-2">Try again</Button>
				</div>
			)}

			{!loading && !error && keywordSets.length === 0 && (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-16">
					<Tags className="h-12 w-12 text-muted-foreground mb-4" />
					<h3 className="text-lg font-semibold text-foreground mb-2">No keyword sets yet</h3>
					<p className="text-sm text-muted-foreground mb-4">Create some keyword sets first!</p>
					<Button onClick={() => setView("create-ai-keywords")}>Create with AI</Button>
				</div>
			)}

			{!loading && !error && keywordSets.length > 0 && (
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{keywordSets.map((set) => (
						<button
							key={set.id}
							onClick={() => handleSelectKeywordSet(set)}
							disabled={set.questions.length === 0}
							className="group rounded-xl border border-accent/30 bg-accent/10 p-5 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
						>
							<div className="flex items-start justify-between mb-3">
								<h3 className="font-semibold text-foreground line-clamp-1">{set.name}</h3>
								<div className="flex items-center gap-1.5 shrink-0 ml-2">
									<span className="text-xs px-2 py-0.5 rounded-full border bg-blue-500/10 text-blue-500 border-blue-500/20 uppercase">{set.language}</span>
									<span className={`text-xs px-2 py-0.5 rounded-full border ${getLevelBadgeClass(set.level as "easy" | "medium" | "hard")}`}>{set.level}</span>
								</div>
							</div>
							<p className="text-sm text-muted-foreground mb-3 line-clamp-2">{set.description}</p>
							{set.subjects?.length > 0 && (
								<div className="flex flex-wrap gap-1 mb-3">
									{set.subjects.slice(0, 3).map((subject: string, i: number) => (
										<span key={i} className="inline-block px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary border border-primary/20">{subject}</span>
									))}
								</div>
							)}
							<div className="flex items-center justify-between pt-3 border-t border-border/30">
								<div className="flex items-center gap-2 text-sm text-muted-foreground">
									<Tags className="h-4 w-4" />
									<span>{set.questions.length} questions</span>
								</div>
								{set.questions.length > 0 && (
									<div className="flex items-center gap-1 text-accent">
										<Play className="h-4 w-4" />
										<span className="text-sm font-medium">Play</span>
									</div>
								)}
							</div>
						</button>
					))}
				</div>
			)}
		</div>
	)
}

// == PLAY ORDER PHRASE VIEW ==
export function PlayOrderPhraseView() {
	const {
		loading, error, orderPhraseSets, setView,
		handleBackToHome, handleSelectOrderPhraseSet, loadOrderPhraseSets, getLevelBadgeClass
	} = useIntello()

	return (
		<div className="space-y-6">
			<div className="flex items-center gap-4">
				<Button variant="ghost" size="icon" onClick={handleBackToHome} className="shrink-0">
					<ArrowLeft className="h-5 w-5" />
				</Button>
				<div className="flex items-center gap-3">
					<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
						<ListOrdered className="h-6 w-6 text-accent" />
					</div>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">Order Phrase Sets</h1>
						<p className="text-sm text-muted-foreground">Select a set to play</p>
					</div>
				</div>
			</div>

			{loading && (
				<div className="flex items-center justify-center py-16">
					<Loader2 className="h-8 w-8 animate-spin text-accent" />
				</div>
			)}

			{error && (
				<div className="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
					<p className="text-sm text-destructive">{error}</p>
					<Button variant="link" onClick={loadOrderPhraseSets} className="text-destructive p-0 h-auto mt-2">Try again</Button>
				</div>
			)}

			{!loading && !error && orderPhraseSets.length === 0 && (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-16">
					<ListOrdered className="h-12 w-12 text-muted-foreground mb-4" />
					<h3 className="text-lg font-semibold text-foreground mb-2">No order phrase sets yet</h3>
					<p className="text-sm text-muted-foreground mb-4">Create some order phrase sets first!</p>
					<Button onClick={() => setView("create-ai-order-phrase")}>Create with AI</Button>
				</div>
			)}

			{!loading && !error && orderPhraseSets.length > 0 && (
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{orderPhraseSets.map((set) => (
						<button
							key={set.id}
							onClick={() => handleSelectOrderPhraseSet(set)}
							disabled={set.questions.length === 0}
							className="group rounded-xl border border-accent/30 bg-accent/10 p-5 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
						>
							<div className="flex items-start justify-between mb-3">
								<h3 className="font-semibold text-foreground line-clamp-1">{set.name}</h3>
								<div className="flex items-center gap-1.5 shrink-0 ml-2">
									<span className="text-xs px-2 py-0.5 rounded-full border bg-blue-500/10 text-blue-500 border-blue-500/20 uppercase">{set.language}</span>
									<span className={`text-xs px-2 py-0.5 rounded-full border ${getLevelBadgeClass(set.level as "easy" | "medium" | "hard")}`}>{set.level}</span>
								</div>
							</div>
							<p className="text-sm text-muted-foreground mb-3 line-clamp-2">{set.description}</p>
							{set.subjects?.length > 0 && (
								<div className="flex flex-wrap gap-1 mb-3">
									{set.subjects.slice(0, 3).map((subject: string, i: number) => (
										<span key={i} className="inline-block px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary border border-primary/20">{subject}</span>
									))}
								</div>
							)}
							<div className="flex items-center justify-between pt-3 border-t border-border/30">
								<div className="flex items-center gap-2 text-sm text-muted-foreground">
									<ListOrdered className="h-4 w-4" />
									<span>{set.questions.length} phrases</span>
								</div>
								{set.questions.length > 0 && (
									<div className="flex items-center gap-1 text-accent">
										<Play className="h-4 w-4" />
										<span className="text-sm font-medium">Play</span>
									</div>
								)}
							</div>
						</button>
					))}
				</div>
			)}
		</div>
	)
}

