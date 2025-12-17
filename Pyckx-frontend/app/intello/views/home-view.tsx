"use client"

import { Brain, Play, PenTool, Sparkles, FileText, Layers, CheckCircle2, Tags, ListOrdered } from "lucide-react"
import { useIntello } from "../context"

export function HomeView() {
	const {
		setView,
		handlePlayQcm,
		handlePlayOpen,
		handlePlayFlashcard,
		handlePlayTrueOrFalse,
		handlePlayKeywords,
		handlePlayOrderPhrase
	} = useIntello()

	return (
		<div className="space-y-8">
			{/* Header */}
			<div className="flex items-center gap-3">
				<div className="flex h-12 w-12 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
					<Brain className="h-6 w-6 text-accent" />
				</div>
				<div>
					<h1 className="text-2xl font-bold tracking-tight text-foreground">Intello</h1>
					<p className="text-sm text-muted-foreground">Create quizzes or play existing ones</p>
				</div>
			</div>

			{/* CREATE SECTION */}
			<section className="space-y-4">
				<div className="flex items-center gap-2">
					<PenTool className="h-5 w-5 text-primary" />
					<h2 className="text-lg font-semibold text-foreground">Create</h2>
				</div>
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{/* Manual QCM */}
					<button
						onClick={() => setView("create-qcm")}
						className="group rounded-xl border border-accent/30 bg-accent/10 p-6 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-accent/20 border border-accent/30">
								<Brain className="h-5 w-5 text-accent" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">Manual QCM</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Create multiple choice questions manually</p>
						<div className="flex items-center gap-2 text-accent">
							<PenTool className="h-4 w-4" />
							<span className="text-sm font-medium">Create</span>
						</div>
					</button>

					{/* AI QCM */}
					<button
						onClick={() => setView("create-ai-qcm")}
						className="group rounded-xl border border-primary/30 bg-primary/10 p-6 backdrop-blur-sm transition-all hover:border-primary/50 hover:bg-primary/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/20 border border-primary/30">
								<Sparkles className="h-5 w-5 text-primary" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">AI QCM</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Generate QCM from your documents using AI</p>
						<div className="flex items-center gap-2 text-primary">
							<Sparkles className="h-4 w-4" />
							<span className="text-sm font-medium">Generate</span>
						</div>
					</button>

					{/* AI Open Questions */}
					<button
						onClick={() => setView("create-ai-open")}
						className="group rounded-xl border border-primary/30 bg-primary/10 p-6 backdrop-blur-sm transition-all hover:border-primary/50 hover:bg-primary/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/20 border border-primary/30">
								<Sparkles className="h-5 w-5 text-primary" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">AI Open Questions</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Generate open-ended questions using AI</p>
						<div className="flex items-center gap-2 text-primary">
							<Sparkles className="h-4 w-4" />
							<span className="text-sm font-medium">Generate</span>
						</div>
					</button>

					{/* AI Flashcards */}
					<button
						onClick={() => setView("create-ai-flashcard")}
						className="group rounded-xl border border-primary/30 bg-primary/10 p-6 backdrop-blur-sm transition-all hover:border-primary/50 hover:bg-primary/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/20 border border-primary/30">
								<Sparkles className="h-5 w-5 text-primary" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">AI Flashcards</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Generate flashcards for memorization using AI</p>
						<div className="flex items-center gap-2 text-primary">
							<Sparkles className="h-4 w-4" />
							<span className="text-sm font-medium">Generate</span>
						</div>
					</button>

					<button
						onClick={() => setView("create-ai-true-false")}
						className="group rounded-xl border border-primary/30 bg-primary/10 p-6 backdrop-blur-sm transition-all hover:border-primary/50 hover:bg-primary/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/20 border border-primary/30">
								<Sparkles className="h-5 w-5 text-primary" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">AI True or False</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Generate true/false statements using AI</p>
						<div className="flex items-center gap-2 text-primary">
							<Sparkles className="h-4 w-4" />
							<span className="text-sm font-medium">Generate</span>
						</div>
					</button>

					{/* AI Keywords */}
					<button
						onClick={() => setView("create-ai-keywords")}
						className="group rounded-xl border border-primary/30 bg-primary/10 p-6 backdrop-blur-sm transition-all hover:border-primary/50 hover:bg-primary/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/20 border border-primary/30">
								<Sparkles className="h-5 w-5 text-primary" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">AI Keywords</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Generate keyword recognition exercises using AI</p>
						<div className="flex items-center gap-2 text-primary">
							<Sparkles className="h-4 w-4" />
							<span className="text-sm font-medium">Generate</span>
						</div>
					</button>

					{/* AI Order Phrase */}
					<button
						onClick={() => setView("create-ai-order-phrase")}
						className="group rounded-xl border border-primary/30 bg-primary/10 p-6 backdrop-blur-sm transition-all hover:border-primary/50 hover:bg-primary/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/20 border border-primary/30">
								<Sparkles className="h-5 w-5 text-primary" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">AI Order Phrase</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Generate phrase ordering exercises using AI</p>
						<div className="flex items-center gap-2 text-primary">
							<Sparkles className="h-4 w-4" />
							<span className="text-sm font-medium">Generate</span>
						</div>
					</button>
				</div>
			</section>

			{/* PLAY SECTION */}
			<section className="space-y-4">
				<div className="flex items-center gap-2">
					<Play className="h-5 w-5 text-accent" />
					<h2 className="text-lg font-semibold text-foreground">Play</h2>
				</div>
				<div className="grid gap-4 sm:grid-cols-2">
					{/* Play QCM */}
					<button
						onClick={handlePlayQcm}
						className="group rounded-xl border border-accent/30 bg-accent/10 p-6 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-accent/20 border border-accent/30">
								<Brain className="h-5 w-5 text-accent" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">QCM Sets</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Play your multiple choice quizzes</p>
						<div className="flex items-center gap-2 text-accent">
							<Play className="h-4 w-4" />
							<span className="text-sm font-medium">Browse & Play</span>
						</div>
					</button>

					{/* Play Open Questions */}
					<button
						onClick={handlePlayOpen}
						className="group rounded-xl border border-accent/30 bg-accent/10 p-6 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-accent/20 border border-accent/30">
								<FileText className="h-5 w-5 text-accent" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">Open Question Sets</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Play your open-ended question sets</p>
						<div className="flex items-center gap-2 text-accent">
							<Play className="h-4 w-4" />
							<span className="text-sm font-medium">Browse & Play</span>
						</div>
					</button>

					{/* Play Flashcards */}
					<button
						onClick={handlePlayFlashcard}
						className="group rounded-xl border border-accent/30 bg-accent/10 p-6 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-accent/20 border border-accent/30">
								<Layers className="h-5 w-5 text-accent" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">Flashcard Sets</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Study your flashcard sets</p>
						<div className="flex items-center gap-2 text-accent">
							<Play className="h-4 w-4" />
							<span className="text-sm font-medium">Browse & Study</span>
						</div>
					</button>

					{/* Play True or False */}
					<button
						onClick={handlePlayTrueOrFalse}
						className="group rounded-xl border border-accent/30 bg-accent/10 p-6 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-accent/20 border border-accent/30">
								<CheckCircle2 className="h-5 w-5 text-accent" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">True or False Sets</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Play your true/false games</p>
						<div className="flex items-center gap-2 text-accent">
							<Play className="h-4 w-4" />
							<span className="text-sm font-medium">Browse & Play</span>
						</div>
					</button>

					{/* Play Keywords */}
					<button
						onClick={handlePlayKeywords}
						className="group rounded-xl border border-accent/30 bg-accent/10 p-6 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-accent/20 border border-accent/30">
								<Tags className="h-5 w-5 text-accent" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">Keyword Sets</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Practice keyword recognition</p>
						<div className="flex items-center gap-2 text-accent">
							<Play className="h-4 w-4" />
							<span className="text-sm font-medium">Browse & Play</span>
						</div>
					</button>

					{/* Play Order Phrase */}
					<button
						onClick={handlePlayOrderPhrase}
						className="group rounded-xl border border-accent/30 bg-accent/10 p-6 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20 hover:scale-[1.02] text-left cursor-pointer"
					>
						<div className="flex items-center gap-3 mb-3">
							<div className="flex h-10 w-10 items-center justify-center rounded-lg bg-accent/20 border border-accent/30">
								<ListOrdered className="h-5 w-5 text-accent" />
							</div>
							<h3 className="font-semibold text-lg text-foreground">Order Phrase Sets</h3>
						</div>
						<p className="text-sm text-muted-foreground mb-4">Arrange words in correct order</p>
						<div className="flex items-center gap-2 text-accent">
							<Play className="h-4 w-4" />
							<span className="text-sm font-medium">Browse & Play</span>
						</div>
					</button>
				</div>
			</section>
		</div>
	)
}
