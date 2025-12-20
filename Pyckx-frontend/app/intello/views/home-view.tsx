"use client"

import { Brain, Play, PenTool, Sparkles, FileText, Layers, CheckCircle2, Tags, ListOrdered, TextCursor, Plus, Gamepad2 } from "lucide-react"
import { useIntello } from "../context"

// Compact card component for consistent styling
interface GameCardProps {
	title: string
	description: string
	icon: React.ReactNode
	onClick: () => void
	variant: "create-manual" | "create-ai" | "play"
	actionLabel: string
}

function GameCard({ title, description, icon, onClick, variant, actionLabel }: GameCardProps) {
	const variantStyles = {
		"create-manual": {
			border: "border-accent/30 hover:border-accent/50",
			bg: "bg-accent/10 hover:bg-accent/20",
			iconBg: "bg-accent/20 border-accent/30",
			actionColor: "text-accent",
		},
		"create-ai": {
			border: "border-primary/30 hover:border-primary/50",
			bg: "bg-primary/10 hover:bg-primary/20",
			iconBg: "bg-primary/20 border-primary/30",
			actionColor: "text-primary",
		},
		"play": {
			border: "border-accent/30 hover:border-accent/50",
			bg: "bg-accent/10 hover:bg-accent/20",
			iconBg: "bg-accent/20 border-accent/30",
			actionColor: "text-accent",
		},
	}

	const styles = variantStyles[variant]

	return (
		<button
			onClick={onClick}
			className={`group rounded-lg border ${styles.border} ${styles.bg} p-3 backdrop-blur-sm transition-all hover:scale-[1.02] text-left cursor-pointer w-full`}
		>
			<div className="flex items-center gap-2.5">
				<div className={`flex h-8 w-8 shrink-0 items-center justify-center rounded-md ${styles.iconBg} border`}>
					{icon}
				</div>
				<div className="min-w-0 flex-1">
					<h3 className="font-medium text-sm text-foreground truncate">{title}</h3>
					<p className="text-xs text-muted-foreground truncate">{description}</p>
				</div>
			</div>
			<div className={`flex items-center gap-1.5 mt-2 ${styles.actionColor}`}>
				{variant === "create-ai" ? <Sparkles className="h-3 w-3" /> : variant === "create-manual" ? <PenTool className="h-3 w-3" /> : <Play className="h-3 w-3" />}
				<span className="text-xs font-medium">{actionLabel}</span>
			</div>
		</button>
	)
}

// Section header component
interface SectionHeaderProps {
	icon: React.ReactNode
	title: string
	subtitle?: string
	iconColor: string
}

function SectionHeader({ icon, title, subtitle, iconColor }: SectionHeaderProps) {
	return (
		<div className="flex items-center gap-2 mb-3">
			<div className={iconColor}>{icon}</div>
			<div>
				<h2 className="text-base font-semibold text-foreground leading-tight">{title}</h2>
				{subtitle && <p className="text-xs text-muted-foreground">{subtitle}</p>}
			</div>
		</div>
	)
}

export function HomeView() {
	const {
		setView,
		handlePlayQcm,
		handlePlayOpen,
		handlePlayFlashcard,
		handlePlayTrueOrFalse,
		handlePlayKeywords,
		handlePlayOrderPhrase,
		handlePlayFillBlank
	} = useIntello()

	// Create game types data
	const createGames = [
		{ id: "manual-qcm", title: "Manual QCM", description: "Create questions manually", icon: <Brain className="h-4 w-4 text-accent" />, onClick: () => setView("create-qcm"), variant: "create-manual" as const, actionLabel: "Create" },
		{ id: "ai-qcm", title: "AI QCM", description: "Generate from documents", icon: <Sparkles className="h-4 w-4 text-primary" />, onClick: () => setView("create-ai-qcm"), variant: "create-ai" as const, actionLabel: "Generate" },
		{ id: "ai-open", title: "AI Open Questions", description: "AI-graded answers", icon: <Sparkles className="h-4 w-4 text-primary" />, onClick: () => setView("create-ai-open"), variant: "create-ai" as const, actionLabel: "Generate" },
		{ id: "ai-flashcard", title: "AI Flashcards", description: "Study cards", icon: <Sparkles className="h-4 w-4 text-primary" />, onClick: () => setView("create-ai-flashcard"), variant: "create-ai" as const, actionLabel: "Generate" },
		{ id: "ai-true-false", title: "AI True or False", description: "Statement verification", icon: <Sparkles className="h-4 w-4 text-primary" />, onClick: () => setView("create-ai-true-false"), variant: "create-ai" as const, actionLabel: "Generate" },
		{ id: "ai-keywords", title: "AI Keywords", description: "Keyword recognition", icon: <Sparkles className="h-4 w-4 text-primary" />, onClick: () => setView("create-ai-keywords"), variant: "create-ai" as const, actionLabel: "Generate" },
		{ id: "ai-order-phrase", title: "AI Order Phrase", description: "Word ordering", icon: <Sparkles className="h-4 w-4 text-primary" />, onClick: () => setView("create-ai-order-phrase"), variant: "create-ai" as const, actionLabel: "Generate" },
		{ id: "ai-fill-blank", title: "AI Fill Blank", description: "Complete phrases", icon: <Sparkles className="h-4 w-4 text-primary" />, onClick: () => setView("create-ai-fill-blank"), variant: "create-ai" as const, actionLabel: "Generate" },
	]

	// Play game types data
	const playGames = [
		{ id: "play-qcm", title: "QCM Sets", description: "Multiple choice quizzes", icon: <Brain className="h-4 w-4 text-accent" />, onClick: handlePlayQcm, actionLabel: "Play" },
		{ id: "play-open", title: "Open Questions", description: "Written answers", icon: <FileText className="h-4 w-4 text-accent" />, onClick: handlePlayOpen, actionLabel: "Play" },
		{ id: "play-flashcard", title: "Flashcards", description: "Study cards", icon: <Layers className="h-4 w-4 text-accent" />, onClick: handlePlayFlashcard, actionLabel: "Study" },
		{ id: "play-true-false", title: "True or False", description: "Statement games", icon: <CheckCircle2 className="h-4 w-4 text-accent" />, onClick: handlePlayTrueOrFalse, actionLabel: "Play" },
		{ id: "play-keywords", title: "Keywords", description: "Keyword recognition", icon: <Tags className="h-4 w-4 text-accent" />, onClick: handlePlayKeywords, actionLabel: "Play" },
		{ id: "play-order-phrase", title: "Order Phrase", description: "Arrange words", icon: <ListOrdered className="h-4 w-4 text-accent" />, onClick: handlePlayOrderPhrase, actionLabel: "Play" },
		{ id: "play-fill-blank", title: "Fill Blank", description: "Complete phrases", icon: <TextCursor className="h-4 w-4 text-accent" />, onClick: handlePlayFillBlank, actionLabel: "Play" },
	]

	return (
		<div className="space-y-6">
			{/* Header */}
			<div className="flex items-center gap-3">
				<div className="flex h-10 w-10 items-center justify-center rounded-xl bg-accent/10 border border-accent/20">
					<Brain className="h-5 w-5 text-accent" />
				</div>
				<div>
					<h1 className="text-xl font-bold tracking-tight text-foreground">Intello</h1>
					<p className="text-xs text-muted-foreground">Create quizzes or play existing ones</p>
				</div>
			</div>

			{/* Two Column Layout for Create & Play */}
			<div className="grid gap-6 lg:grid-cols-2">
				{/* CREATE SECTION */}
				<section className="rounded-xl border border-border/50 bg-card/30 backdrop-blur-sm p-4">
					<SectionHeader
						icon={<Plus className="h-4 w-4" />}
						title="Create"
						subtitle="Generate new learning content"
						iconColor="text-primary"
					/>
					<div className="grid gap-2 sm:grid-cols-2">
						{createGames.map((game) => (
							<GameCard
								key={game.id}
								title={game.title}
								description={game.description}
								icon={game.icon}
								onClick={game.onClick}
								variant={game.variant}
								actionLabel={game.actionLabel}
							/>
						))}
					</div>
				</section>

				{/* PLAY SECTION */}
				<section className="rounded-xl border border-border/50 bg-card/30 backdrop-blur-sm p-4">
					<SectionHeader
						icon={<Gamepad2 className="h-4 w-4" />}
						title="Play"
						subtitle="Practice with your saved games"
						iconColor="text-accent"
					/>
					<div className="grid gap-2 sm:grid-cols-2">
						{playGames.map((game) => (
							<GameCard
								key={game.id}
								title={game.title}
								description={game.description}
								icon={game.icon}
								onClick={game.onClick}
								variant="play"
								actionLabel={game.actionLabel}
							/>
						))}
					</div>
				</section>
			</div>
		</div>
	)
}
