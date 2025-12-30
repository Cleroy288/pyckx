"use client"

import { useState, useEffect } from "react"
import { ArrowLeft, GraduationCap, Plus, X, Loader2, FileText, Sparkles } from "lucide-react"
import { useIntello } from "../../context"
import { startStudySession, getUserResources } from "@/lib/api/intello"

export function CreateSessionView() {
	const { selectedCourse, handleSelectCourse, handleSelectSession } = useIntello()
	const [topic, setTopic] = useState("")
	const [instructions, setInstructions] = useState("")
	const [keywords, setKeywords] = useState<string[]>([])
	const [keywordInput, setKeywordInput] = useState("")
	const [language, setLanguage] = useState<"en" | "fr" | "es" | "de" | "nl">("en")
	const [selectedResources, setSelectedResources] = useState<string[]>([])
	const [isSubmitting, setIsSubmitting] = useState(false)
	const [error, setError] = useState<string | null>(null)

	// Load user resources from API
	const [resources, setResources] = useState<Array<{ id: string, filename: string, token_count: number }>>([])
	const [isLoadingResources, setIsLoadingResources] = useState(true)

	useEffect(() => {
		const fetchResources = async () => {
			try {
				const userResources = await getUserResources()
				setResources(userResources)
			} catch (err) {
				console.error("Failed to fetch resources:", err)
			} finally {
				setIsLoadingResources(false)
			}
		}
		fetchResources()
	}, [])

	if (!selectedCourse) {
		return (
			<div className="text-center py-12">
				<p className="text-muted-foreground">No course selected</p>
			</div>
		)
	}

	const handleBackToCourse = () => {
		handleSelectCourse(selectedCourse)
	}

	const handleAddKeyword = () => {
		const trimmed = keywordInput.trim()
		if (trimmed && !keywords.includes(trimmed) && keywords.length < 5) {
			setKeywords([...keywords, trimmed])
			setKeywordInput("")
		}
	}

	const handleRemoveKeyword = (keyword: string) => {
		setKeywords(keywords.filter(k => k !== keyword))
	}

	const handleToggleResource = (resourceId: string) => {
		setSelectedResources(prev =>
			prev.includes(resourceId)
				? prev.filter(id => id !== resourceId)
				: [...prev, resourceId]
		)
	}

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault()
		setError(null)

		if (!topic.trim()) {
			setError("Topic is required")
			return
		}

		if (selectedResources.length === 0) {
			setError("Please select at least one resource")
			return
		}

		setIsSubmitting(true)

		try {
			const session = await startStudySession(
				selectedCourse.id,
				topic.trim(),
				instructions.trim() || "",
				keywords.length > 0 ? keywords : [],
				language
			)
			handleSelectSession(session)
		} catch (err) {
			setError(err instanceof Error ? err.message : "Failed to create session")
		} finally {
			setIsSubmitting(false)
		}
	}

	return (
		<div className="space-y-6">
			{/* Header */}
			<div className="flex items-center gap-3">
				<button onClick={handleBackToCourse} className="p-2 rounded-lg hover:bg-muted/50 transition-colors">
					<ArrowLeft className="h-5 w-5" />
				</button>
				<div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
					<GraduationCap className="h-5 w-5 text-primary" />
				</div>
				<div>
					<h1 className="text-xl font-bold tracking-tight text-foreground">New Study Session</h1>
					<p className="text-xs text-muted-foreground">{selectedCourse.name}</p>
				</div>
			</div>

			{/* Form */}
			<form onSubmit={handleSubmit} className="space-y-6">
				{/* Topic */}
				<div className="space-y-2">
					<label htmlFor="topic" className="text-sm font-medium text-foreground">
						Topic <span className="text-destructive">*</span>
					</label>
					<input
						id="topic"
						type="text"
						value={topic}
						onChange={(e) => setTopic(e.target.value)}
						placeholder="e.g., Introduction to React Hooks"
						className="w-full px-4 py-2.5 rounded-lg border border-border/50 bg-background/50 text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
						required
					/>
				</div>

				{/* Instructions */}
				<div className="space-y-2">
					<label htmlFor="instructions" className="text-sm font-medium text-foreground">
						Instructions (optional)
					</label>
					<textarea
						id="instructions"
						value={instructions}
						onChange={(e) => setInstructions(e.target.value)}
						placeholder="Any specific focus or learning objectives..."
						className="w-full px-4 py-2.5 rounded-lg border border-border/50 bg-background/50 text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50 min-h-[100px]"
					/>
				</div>

				{/* Keywords */}
				<div className="space-y-2">
					<label className="text-sm font-medium text-foreground">Keywords (max 5)</label>
					<div className="flex gap-2">
						<input
							type="text"
							value={keywordInput}
							onChange={(e) => setKeywordInput(e.target.value)}
							onKeyDown={(e) => e.key === "Enter" && (e.preventDefault(), handleAddKeyword())}
							placeholder="Add keywords..."
							className="flex-1 px-4 py-2.5 rounded-lg border border-border/50 bg-background/50 text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
							disabled={keywords.length >= 5}
						/>
						<button
							type="button"
							onClick={handleAddKeyword}
							disabled={keywords.length >= 5}
							className="px-4 py-2.5 rounded-lg border border-border/50 hover:bg-muted/50 disabled:opacity-50 transition-colors"
						>
							<Plus className="h-4 w-4" />
						</button>
					</div>
					{keywords.length > 0 && (
						<div className="flex flex-wrap gap-2">
							{keywords.map(keyword => (
								<span
									key={keyword}
									className="inline-flex items-center gap-1 px-3 py-1 rounded-full bg-primary/10 text-primary text-sm"
								>
									{keyword}
									<button
										type="button"
										onClick={() => handleRemoveKeyword(keyword)}
										className="hover:text-primary/70 transition-colors"
									>
										<X className="h-3 w-3" />
									</button>
								</span>
							))}
						</div>
					)}
				</div>

				{/* Language */}
				<div className="space-y-2">
					<label htmlFor="language" className="text-sm font-medium text-foreground">Language</label>
					<select
						id="language"
						value={language}
						onChange={(e) => setLanguage(e.target.value as typeof language)}
						className="w-full px-4 py-2.5 rounded-lg border border-border/50 bg-background/50 text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
					>
						<option value="en">English</option>
						<option value="fr">French</option>
						<option value="es">Spanish</option>
						<option value="de">German</option>
						<option value="nl">Dutch</option>
					</select>
				</div>

				{/* Resources Selection */}
				<div className="space-y-2">
					<label className="text-sm font-medium text-foreground">
						Select Resources <span className="text-destructive">*</span>
					</label>
					<div className="space-y-2 p-4 rounded-lg border border-border/50 bg-card/30 max-h-[300px] overflow-y-auto">
						{isLoadingResources ? (
							<div className="flex items-center justify-center py-4">
								<Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
								<span className="ml-2 text-sm text-muted-foreground">Loading resources...</span>
							</div>
						) : resources.length === 0 ? (
							<p className="text-sm text-muted-foreground text-center py-4">
								No resources available. Upload resources first.
							</p>
						) : (
							resources.map(resource => (
								<label
									key={resource.id}
									className="flex items-center gap-3 p-3 rounded-lg hover:bg-muted/30 cursor-pointer transition-colors"
								>
									<input
										type="checkbox"
										checked={selectedResources.includes(resource.id)}
										onChange={() => handleToggleResource(resource.id)}
										className="h-4 w-4 rounded border-border/50 text-primary focus:ring-2 focus:ring-primary/50"
									/>
									<FileText className="h-4 w-4 text-muted-foreground" />
									<span className="text-sm text-foreground">{resource.filename}</span>
								</label>
							))
						)}
					</div>
					<p className="text-xs text-muted-foreground">
						{selectedResources.length} resource{selectedResources.length !== 1 ? "s" : ""} selected
					</p>
				</div>

				{/* Error Display */}
				{error && (
					<div className="p-4 rounded-lg bg-destructive/10 border border-destructive/20 text-destructive text-sm">
						{error}
					</div>
				)}

				{/* Submit Button */}
				<div className="flex gap-3 pt-4">
					<button
						type="button"
						onClick={handleBackToCourse}
						className="flex-1 px-6 py-3 rounded-lg border border-border/50 hover:bg-muted/50 transition-colors"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={isSubmitting || isLoadingResources || resources.length === 0}
						className="flex-1 inline-flex items-center justify-center gap-2 px-6 py-3 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 transition-colors"
					>
						{isSubmitting ? (
							<>
								<Loader2 className="h-4 w-4 animate-spin" />
								Creating...
							</>
						) : (
							<>
								<Sparkles className="h-4 w-4" />
								Start Session
							</>
						)}
					</button>
				</div>
			</form>
		</div>
	)
}
