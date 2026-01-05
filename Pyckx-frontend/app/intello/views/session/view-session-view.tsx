"use client"

import { useState, useEffect, useCallback, useRef } from "react"
import { BookOpen, ArrowLeft, Loader2, GraduationCap, Sparkles, RefreshCw, GitBranch, CheckCircle, XCircle, Plus, X, Target, FileText, Tag, HelpCircle } from "lucide-react"
import { useIntello } from "../../context"
import { generateCourse, getSession } from "@/lib/api/intello"
import ReactMarkdown from "react-markdown"
import dynamic from "next/dynamic"
import type { GeneratedCourse, CourseModule, ContentBlock, QcmSetPayload, FlashcardSetPayload, TrueFalseSetPayload } from "@/lib/types/block-protocol"

const MermaidDiagram = dynamic(
	() => import("@/components/intello/mermaid-diagram").then((mod) => mod.MermaidDiagram),
	{
		ssr: false,
		loading: () => (
			<div className="my-4 p-6 rounded-xl border border-border/50 bg-card/30 flex items-center justify-center min-h-[100px]">
				<div className="flex items-center gap-2 text-muted-foreground animate-pulse">
					<GitBranch className="h-5 w-5" />
					<span className="text-sm">Loading diagram...</span>
				</div>
			</div>
		),
	}
)

// Block Components
function TitleBlock({ content }: { content: string }) {
	return <h2 className="text-2xl font-bold text-foreground mt-8 mb-4">{content}</h2>
}

function SubtitleBlock({ content }: { content: string }) {
	return <h3 className="text-xl font-semibold text-foreground/90 mt-6 mb-3">{content}</h3>
}

function TextBlock({ content }: { content: string }) {
	return (
		<div className="prose prose-lg dark:prose-invert max-w-none text-muted-foreground/90 leading-relaxed mb-4">
			<ReactMarkdown>{content}</ReactMarkdown>
		</div>
	)
}

function SchemaBlock({ content }: { content: string }) {
	return <MermaidDiagram code={content} diagramType="diagram" />
}

// QCM Set Block with interactive quiz
function QcmSetBlock({ data }: { data: QcmSetPayload }) {
	const [currentQuestion, setCurrentQuestion] = useState(0)
	const [selectedAnswer, setSelectedAnswer] = useState<string | null>(null)
	const [showResult, setShowResult] = useState(false)
	const [score, setScore] = useState(0)
	const [completed, setCompleted] = useState(false)

	// Safety check
	if (!data || !data.questions || data.questions.length === 0) {
		return <div className="p-4 bg-red-500/10 rounded-lg text-red-400">QCM data is invalid</div>
	}

	const question = data.questions[currentQuestion]
	const allAnswers = question ? [question.right_answer, ...question.wrong_answers].sort(() => Math.random() - 0.5) : []

	const handleAnswer = (answer: string) => {
		if (showResult) return
		setSelectedAnswer(answer)
		setShowResult(true)
		if (answer === question.right_answer) {
			setScore(s => s + 1)
		}
	}

	const handleNext = () => {
		if (currentQuestion < data.questions.length - 1) {
			setCurrentQuestion(c => c + 1)
			setSelectedAnswer(null)
			setShowResult(false)
		} else {
			setCompleted(true)
		}
	}

	const handleRetry = () => {
		setCurrentQuestion(0)
		setSelectedAnswer(null)
		setShowResult(false)
		setScore(0)
		setCompleted(false)
	}

	if (completed) {
		return (
			<div className="my-6 p-6 rounded-2xl bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20">
				<div className="text-center space-y-4">
					<div className="text-4xl">🎉</div>
					<h3 className="text-xl font-bold">Quiz Complete!</h3>
					<p className="text-lg">
						Score: <span className="font-bold text-primary">{score}</span> / {data.questions.length}
					</p>
					<button onClick={handleRetry} className="cursor-pointer px-4 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90">
						Try Again
					</button>
				</div>
			</div>
		)
	}

	return (
		<div className="my-6 rounded-2xl bg-gradient-to-br from-card to-card/50 border border-border/50 overflow-hidden">
			{/* Header */}
			<div className="px-6 py-4 bg-primary/10 border-b border-border/30">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<HelpCircle className="h-5 w-5 text-primary" />
						<div>
							<h4 className="font-bold text-foreground">{data.name}</h4>
							<p className="text-xs text-muted-foreground">{data.description}</p>
						</div>
					</div>
					<span className="text-sm text-muted-foreground">
						{currentQuestion + 1} / {data.questions.length}
					</span>
				</div>
			</div>

			{/* Question */}
			<div className="p-6 space-y-4">
				<p className="text-lg font-medium text-foreground">{question.question}</p>

				{/* Answers */}
				<div className="space-y-2">
					{allAnswers.map((answer, i) => {
						const isCorrect = answer === question.right_answer
						const isSelected = answer === selectedAnswer
						let className = "w-full text-left p-4 rounded-xl border transition-all "

						if (!showResult) {
							className += "border-border/50 hover:border-primary/50 hover:bg-muted/30"
						} else if (isCorrect) {
							className += "border-green-500 bg-green-500/10 text-green-400"
						} else if (isSelected && !isCorrect) {
							className += "border-red-500 bg-red-500/10 text-red-400"
						} else {
							className += "border-border/30 opacity-50"
						}

						return (
							<button key={i} onClick={() => handleAnswer(answer)} className={className} disabled={showResult}>
								<div className="flex items-center gap-3">
									{showResult && isCorrect && <CheckCircle className="h-5 w-5 text-green-500" />}
									{showResult && isSelected && !isCorrect && <XCircle className="h-5 w-5 text-red-500" />}
									<span>{answer}</span>
								</div>
							</button>
						)
					})}
				</div>

				{/* Explanation */}
				{showResult && (
					<div className="p-4 rounded-xl bg-muted/30 border border-border/30">
						<p className="text-sm text-muted-foreground">
							<span className="font-medium text-foreground">Explanation:</span> {question.explanation}
						</p>
					</div>
				)}

				{/* Next button */}
				{showResult && (
					<button onClick={handleNext} className="cursor-pointer w-full py-3 rounded-xl bg-primary text-primary-foreground font-medium hover:bg-primary/90">
						{currentQuestion < data.questions.length - 1 ? "Next Question" : "See Results"}
					</button>
				)}
			</div>
		</div>
	)
}

// Flashcard Set Block with flip-card UI
function FlashcardSetBlock({ data }: { data: FlashcardSetPayload }) {
	const [currentCard, setCurrentCard] = useState(0)
	const [isFlipped, setIsFlipped] = useState(false)
	const [completed, setCompleted] = useState(false)

	if (!data || !data.cards || data.cards.length === 0) {
		return <div className="p-4 bg-red-500/10 rounded-lg text-red-400">Flashcard data is invalid</div>
	}

	const card = data.cards[currentCard]

	const handleFlip = () => setIsFlipped(!isFlipped)

	const handleNext = () => {
		if (currentCard < data.cards.length - 1) {
			setCurrentCard(c => c + 1)
			setIsFlipped(false)
		} else {
			setCompleted(true)
		}
	}

	const handlePrev = () => {
		if (currentCard > 0) {
			setCurrentCard(c => c - 1)
			setIsFlipped(false)
		}
	}

	const handleRestart = () => {
		setCurrentCard(0)
		setIsFlipped(false)
		setCompleted(false)
	}

	if (completed) {
		return (
			<div className="my-6 p-6 rounded-2xl bg-gradient-to-br from-blue-500/10 to-blue-500/5 border border-blue-500/20">
				<div className="text-center space-y-4">
					<div className="text-4xl">📚</div>
					<h3 className="text-xl font-bold">Flashcards Complete!</h3>
					<p className="text-muted-foreground">You reviewed all {data.cards.length} cards</p>
					<button onClick={handleRestart} className="cursor-pointer px-4 py-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600">
						Review Again
					</button>
				</div>
			</div>
		)
	}

	return (
		<div className="my-6 rounded-2xl bg-gradient-to-br from-blue-500/5 to-card border border-blue-500/20 overflow-hidden">
			{/* Header */}
			<div className="px-6 py-4 bg-blue-500/10 border-b border-blue-500/20">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<span className="text-2xl">📖</span>
						<div>
							<h4 className="font-bold text-foreground">{data.name}</h4>
							<p className="text-xs text-muted-foreground">{data.description}</p>
						</div>
					</div>
					<span className="text-sm text-muted-foreground">
						{currentCard + 1} / {data.cards.length}
					</span>
				</div>
			</div>

			{/* Card */}
			<div className="p-6">
				<div
					onClick={handleFlip}
					className="cursor-pointer min-h-[200px] rounded-xl border-2 border-dashed border-border/50 flex items-center justify-center p-8 transition-all hover:border-blue-500/50 hover:bg-blue-500/5"
				>
					<div className="text-center">
						<p className="text-xs text-muted-foreground mb-2">
							{isFlipped ? "BACK" : "FRONT"} • Click to flip
						</p>
						<p className={`text-lg font-medium ${isFlipped ? "text-blue-400" : "text-foreground"}`}>
							{isFlipped ? card.back : card.front}
						</p>
					</div>
				</div>

				{/* Navigation */}
				<div className="flex gap-2 mt-4">
					<button
						onClick={handlePrev}
						disabled={currentCard === 0}
						className="cursor-pointer flex-1 py-2 rounded-lg border border-border/50 hover:bg-muted/50 disabled:opacity-30 disabled:cursor-not-allowed"
					>
						← Previous
					</button>
					<button
						onClick={handleNext}
						className="cursor-pointer flex-1 py-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600"
					>
						{currentCard < data.cards.length - 1 ? "Next →" : "Complete ✓"}
					</button>
				</div>
			</div>
		</div>
	)
}

// True/False Set Block with statement selection UI
function TrueFalseSetBlock({ data }: { data: TrueFalseSetPayload }) {
	const [currentStatement, setCurrentStatement] = useState(0)
	const [selectedAnswer, setSelectedAnswer] = useState<boolean | null>(null)
	const [showResult, setShowResult] = useState(false)
	const [score, setScore] = useState(0)
	const [completed, setCompleted] = useState(false)

	if (!data || !data.statements || data.statements.length === 0) {
		return <div className="p-4 bg-red-500/10 rounded-lg text-red-400">True/False data is invalid</div>
	}

	const statement = data.statements[currentStatement]

	const handleAnswer = (answer: boolean) => {
		if (showResult) return
		setSelectedAnswer(answer)
		setShowResult(true)
		if (answer === statement.answer) {
			setScore(s => s + 1)
		}
	}

	const handleNext = () => {
		if (currentStatement < data.statements.length - 1) {
			setCurrentStatement(c => c + 1)
			setSelectedAnswer(null)
			setShowResult(false)
		} else {
			setCompleted(true)
		}
	}

	const handleRetry = () => {
		setCurrentStatement(0)
		setSelectedAnswer(null)
		setShowResult(false)
		setScore(0)
		setCompleted(false)
	}

	if (completed) {
		return (
			<div className="my-6 p-6 rounded-2xl bg-gradient-to-br from-amber-500/10 to-amber-500/5 border border-amber-500/20">
				<div className="text-center space-y-4">
					<div className="text-4xl">✅</div>
					<h3 className="text-xl font-bold">True/False Complete!</h3>
					<p className="text-lg">
						Score: <span className="font-bold text-amber-500">{score}</span> / {data.statements.length}
					</p>
					<button onClick={handleRetry} className="cursor-pointer px-4 py-2 rounded-lg bg-amber-500 text-white hover:bg-amber-600">
						Try Again
					</button>
				</div>
			</div>
		)
	}

	return (
		<div className="my-6 rounded-2xl bg-gradient-to-br from-amber-500/5 to-card border border-amber-500/20 overflow-hidden">
			{/* Header */}
			<div className="px-6 py-4 bg-amber-500/10 border-b border-amber-500/20">
				<div className="flex items-center justify-between">
					<div className="flex items-center gap-3">
						<span className="text-2xl">❓</span>
						<div>
							<h4 className="font-bold text-foreground">{data.name}</h4>
							<p className="text-xs text-muted-foreground">{data.description}</p>
						</div>
					</div>
					<span className="text-sm text-muted-foreground">
						{currentStatement + 1} / {data.statements.length}
					</span>
				</div>
			</div>

			{/* Statement */}
			<div className="p-6 space-y-4">
				<p className="text-lg font-medium text-foreground text-center py-4">
					"{statement.statement}"
				</p>

				{/* True/False buttons */}
				<div className="flex gap-4">
					<button
						onClick={() => handleAnswer(true)}
						disabled={showResult}
						className={`flex-1 py-4 rounded-xl border-2 transition-all flex items-center justify-center gap-2 font-medium ${showResult && statement.answer === true
							? "border-green-500 bg-green-500/10 text-green-400"
							: showResult && selectedAnswer === true && statement.answer !== true
								? "border-red-500 bg-red-500/10 text-red-400"
								: "border-border/50 hover:border-green-500/50 hover:bg-green-500/5"
							}`}
					>
						<CheckCircle className="h-5 w-5" /> TRUE
					</button>
					<button
						onClick={() => handleAnswer(false)}
						disabled={showResult}
						className={`flex-1 py-4 rounded-xl border-2 transition-all flex items-center justify-center gap-2 font-medium ${showResult && statement.answer === false
							? "border-green-500 bg-green-500/10 text-green-400"
							: showResult && selectedAnswer === false && statement.answer !== false
								? "border-red-500 bg-red-500/10 text-red-400"
								: "border-border/50 hover:border-red-500/50 hover:bg-red-500/5"
							}`}
					>
						<XCircle className="h-5 w-5" /> FALSE
					</button>
				</div>

				{/* Explanation */}
				{showResult && (
					<div className="p-4 rounded-xl bg-muted/30 border border-border/30">
						<p className="text-sm text-muted-foreground">
							<span className="font-medium text-foreground">Explanation:</span> {statement.explanation}
						</p>
					</div>
				)}

				{/* Next button */}
				{showResult && (
					<button onClick={handleNext} className="cursor-pointer w-full py-3 rounded-xl bg-amber-500 text-white font-medium hover:bg-amber-600">
						{currentStatement < data.statements.length - 1 ? "Next Statement" : "See Results"}
					</button>
				)}
			</div>
		</div>
	)
}

// Render a block
function RenderBlock({ block }: { block: ContentBlock }) {
	switch (block.type) {
		case "title":
			return <TitleBlock content={block.content} />
		case "subtitle":
			return <SubtitleBlock content={block.content} />
		case "text":
			return <TextBlock content={block.content} />
		case "schema":
			return <SchemaBlock content={block.content} />
		case "qcm_set":
			return <QcmSetBlock data={block.data} />
		case "true_false_set":
			return <TrueFalseSetBlock data={block.data} />
		case "flashcard_set":
			return <FlashcardSetBlock data={block.data} />
		default:
			return null
	}
}

// Render a module
function RenderModule({ module, index }: { module: CourseModule, index: number }) {
	return (
		<section className="space-y-4 mb-12">
			<div className="flex items-center gap-4">
				<div className="flex items-center justify-center w-10 h-10 rounded-xl bg-primary/10 text-xl font-bold text-primary">
					{index + 1}
				</div>
				<h2 className="text-2xl font-bold text-foreground">{module.title}</h2>
			</div>
			<div className="pl-14 border-l-2 border-border/30 space-y-2">
				{module.blocks.map((block, i) => (
					<RenderBlock key={i} block={block} />
				))}
			</div>
		</section>
	)
}

export function ViewSessionView() {
	const { selectedCourse, selectedSession, handleSelectCourse } = useIntello()
	const [isLoading, setIsLoading] = useState(false)
	const [error, setError] = useState<string | null>(null)
	const [generatedCourse, setGeneratedCourse] = useState<GeneratedCourse | null>(null)
	const [isFetchingSession, setIsFetchingSession] = useState(true) // Start true to prevent race

	// Ref to prevent duplicate generation requests
	const hasRequestedGeneration = useRef(false)
	const currentSessionId = useRef<string | null>(null)

	// Check for existing content AND auto-generate if needed - ALL IN ONE EFFECT
	useEffect(() => {
		// Reset guard when session changes
		if (selectedSession?.id !== currentSessionId.current) {
			hasRequestedGeneration.current = false
			currentSessionId.current = selectedSession?.id ?? null
		}

		if (!selectedSession || !selectedCourse) return;

		// Capture values for type safety in async function
		const session = selectedSession;
		const course = selectedCourse;

		let isMounted = true;

		async function loadAndGenerate() {
			setIsFetchingSession(true);

			try {
				// Step 1: Check for existing content
				const sessionDetail = await getSession(course.id, session.id);

				if (!isMounted) return;

				if (sessionDetail.generated_content) {
					// Already has content - use it
					setGeneratedCourse(sessionDetail.generated_content as unknown as GeneratedCourse);
					setIsFetchingSession(false);
					return;
				}

				// Step 2: No content - generate (but only once!)
				if (hasRequestedGeneration.current) {
					setIsFetchingSession(false);
					return; // Already requested, don't request again
				}

				hasRequestedGeneration.current = true;
				setIsFetchingSession(false);
				setIsLoading(true);
				setError(null);

				const result = await generateCourse({
					topic: session.topic,
					keywords: session.keywords?.length > 0 ? session.keywords : undefined,
					instructions: session.instructions || undefined,
					session_id: session.id,
					text_length: "medium",
					exercise_depth: "medium",
				});

				if (isMounted) {
					setGeneratedCourse(result.course as unknown as GeneratedCourse);
				}
			} catch (err) {
				if (isMounted) {
					console.error("Failed:", err);
					setError(err instanceof Error ? err.message : "Failed to generate course");
				}
			} finally {
				if (isMounted) {
					setIsLoading(false);
					setIsFetchingSession(false);
				}
			}
		}

		loadAndGenerate();

		return () => { isMounted = false; };
	}, [selectedSession?.id, selectedCourse?.id]);

	if (!selectedSession || !selectedCourse) {
		return (
			<div className="text-center py-12">
				<p className="text-muted-foreground">No session selected</p>
			</div>
		)
	}

	const handleBackToCourse = () => {
		handleSelectCourse(selectedCourse)
	}

	// Manual regeneration
	const handleGenerateCourse = async () => {
		if (!selectedSession) return;

		hasRequestedGeneration.current = true; // Allow regeneration
		setIsLoading(true)
		setError(null)
		try {
			const result = await generateCourse({
				topic: selectedSession.topic,
				keywords: selectedSession.keywords?.length > 0 ? selectedSession.keywords : undefined,
				instructions: selectedSession.instructions || undefined,
				session_id: selectedSession.id,
				text_length: "medium",
				exercise_depth: "medium",
			})
			setGeneratedCourse(result.course as unknown as GeneratedCourse)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to generate course")
		} finally {
			setIsLoading(false)
		}
	}

	return (
		<div className="space-y-6">
			{/* Header */}
			<div className="flex items-center justify-between">
				<div className="flex items-center gap-3">
					<button onClick={handleBackToCourse} className="p-2 rounded-lg hover:bg-muted/50 transition-colors">
						<ArrowLeft className="h-5 w-5" />
					</button>
					<div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<GraduationCap className="h-5 w-5 text-primary" />
					</div>
					<div>
						<h1 className="text-xl font-bold tracking-tight text-foreground">{selectedSession.topic}</h1>
						<p className="text-xs text-muted-foreground">{selectedCourse.name}</p>
					</div>
				</div>
				<div className="flex items-center gap-2">
					{selectedSession.keywords?.map((kw, i) => (
						<span key={i} className="px-2 py-1 rounded-full bg-primary/10 text-primary text-xs">{kw}</span>
					))}
				</div>
			</div>

			{error && (
				<div className="p-4 rounded-lg bg-destructive/10 border border-destructive/20 text-destructive text-sm">
					{error}
				</div>
			)}

			<div className="min-h-[400px]">
				{isFetchingSession ? (
					<div className="flex flex-col items-center justify-center py-16 space-y-4">
						<Loader2 className="h-8 w-8 animate-spin text-primary" />
						<p className="text-muted-foreground">Checking existing content...</p>
					</div>
				) : isLoading ? (
					<div className="flex flex-col items-center justify-center py-16 space-y-4">
						<Loader2 className="h-8 w-8 animate-spin text-primary" />
						<p className="text-muted-foreground">Generating course with quizzes...</p>
					</div>
				) : generatedCourse ? (
					<div className="bg-card/50 rounded-2xl border border-border/50 p-8 shadow-sm">
						<div className="space-y-4 pb-8 border-b border-border/40 mb-8">
							<h1 className="text-4xl font-bold text-foreground">{generatedCourse.course_metadata.title}</h1>
							<p className="text-lg text-muted-foreground">{generatedCourse.course_metadata.description}</p>
							<button
								onClick={handleGenerateCourse}
								disabled={isLoading}
								className="flex items-center gap-2 px-3 py-1.5 text-sm rounded-lg border border-border/50 hover:bg-muted/50"
							>
								<RefreshCw className="h-3 w-3" />
								Regenerate
							</button>
						</div>

						{generatedCourse.modules.map((module, i) => (
							<RenderModule key={i} module={module} index={i} />
						))}

						{/* Synthesis Module - rendered as regular module */}
						{generatedCourse.synthesis && (
							<div className="mt-16">
								<RenderModule
									key="synthesis"
									module={generatedCourse.synthesis}
									index={generatedCourse.modules.length}
								/>
							</div>
						)}
					</div>
				) : (
					<div className="flex flex-col items-center justify-center py-16 space-y-4">
						<p className="text-muted-foreground">No content available</p>
					</div>
				)}
			</div>
		</div>
	)
}
