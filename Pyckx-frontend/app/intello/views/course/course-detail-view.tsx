"use client"

import { useEffect, useState, useRef } from "react"
import { BookOpen, Plus, ArrowLeft, Loader2, Clock, FileText, GraduationCap, Sparkles, Trash2 } from "lucide-react"
import { useIntello } from "../../context"
import { listSessions, uploadResource, deleteCourse, deleteSession, type SessionData } from "@/lib/api/intello"

export function CourseDetailView() {
	const { selectedCourse, loading, error, handleBackToHome, handleCreateSession, handleSelectSession, handleNavigateToCourses, handleSelectCourse } = useIntello()
	const [sessions, setSessions] = useState<SessionData[]>([])
	const [loadingSessions, setLoadingSessions] = useState(false)

	const fileInputRef = useRef<HTMLInputElement>(null)
	const [isUploading, setIsUploading] = useState(false)
	const [isDeleting, setIsDeleting] = useState(false)
	const [deletingSessionId, setDeletingSessionId] = useState<string | null>(null)

	useEffect(() => {
		if (selectedCourse) {
			setLoadingSessions(true)
			listSessions(selectedCourse.id)
				.then(setSessions)
				.catch(console.error)
				.finally(() => setLoadingSessions(false))
		}
	}, [selectedCourse])

	const handleDeleteCourse = async () => {
		if (!selectedCourse || !confirm("Are you sure you want to delete this course? This will remove all associated sessions and resource links.")) return

		setIsDeleting(true)
		try {
			await deleteCourse(selectedCourse.id)
			handleNavigateToCourses()
		} catch (err) {
			console.error("Failed to delete course:", err)
			alert("Failed to delete course.")
			setIsDeleting(false)
		}
	}

	const handleResourceUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
		const files = e.target.files
		if (!files || files.length === 0 || !selectedCourse) return

		setIsUploading(true)
		try {
			// Backend handles extraction (PDF, DOCX, TXT, PPTX)
			await uploadResource(selectedCourse.id, Array.from(files))

			// Refresh course (imports new resources)
			handleSelectCourse(selectedCourse)
		} catch (err) {
			console.error("Failed to upload resource:", err)
			alert("Failed to upload resource. Please ensure all files are supported (PDF, DOCX, TXT, PPTX).")
		} finally {
			setIsUploading(false)
			if (fileInputRef.current) fileInputRef.current.value = ""
		}
	}

	const handleDeleteSession = async (e: React.MouseEvent, sessionId: string) => {
		e.stopPropagation()
		if (!selectedCourse || !confirm("Are you sure you want to delete this session?")) return

		setDeletingSessionId(sessionId)
		try {
			await deleteSession(selectedCourse.id, sessionId)
			// Refresh sessions list
			const updated = await listSessions(selectedCourse.id)
			setSessions(updated)
		} catch (err) {
			console.error("Failed to delete session:", err)
			alert("Failed to delete session.")
		} finally {
			setDeletingSessionId(null)
		}
	}

	if (!selectedCourse) {
		return (
			<div className="text-center py-12">
				<p className="text-muted-foreground">No course selected</p>
			</div>
		)
	}

	return (
		<div className="space-y-6">
			{/* Header */}
			{/* Header */}
			<div className="flex items-start justify-between">
				<div className="flex items-start gap-4">
					<button
						onClick={handleNavigateToCourses}
						className="mt-1 p-2 rounded-lg hover:bg-muted/50 transition-colors"
						title="Back to courses"
					>
						<ArrowLeft className="h-5 w-5" />
					</button>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">{selectedCourse.name}</h1>
						{selectedCourse.description && (
							<p className="text-muted-foreground mt-1">{selectedCourse.description}</p>
						)}
					</div>
				</div>
				<div className="flex items-center gap-2">
					<button
						onClick={handleDeleteCourse}
						disabled={isDeleting}
						className="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-red-500/10 text-red-500 hover:bg-red-500/20 border border-red-500/20 transition-colors"
					>
						{isDeleting ? <Loader2 className="h-4 w-4 animate-spin" /> : <Trash2 className="h-4 w-4" />}
						Delete
					</button>
					<button
						onClick={handleCreateSession}
						className="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
					>
						<Sparkles className="h-4 w-4" />
						New Session
					</button>
				</div>
			</div>

			{/* Resources */}
			<div className="p-4 rounded-xl border border-border/50 bg-card/30">
				<div className="flex items-center justify-between mb-3">
					<h2 className="font-semibold text-foreground">
						Resources ({selectedCourse.resources?.length || 0})
					</h2>
					<div>
						<input
							type="file"
							multiple
							ref={fileInputRef}
							className="hidden"
							onChange={handleResourceUpload}
							accept=".txt,.pdf,.docx,.doc,.pptx,.ppt"
						/>
						<button
							onClick={() => fileInputRef.current?.click()}
							disabled={isUploading}
							className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-secondary/80 hover:bg-secondary text-secondary-foreground text-xs font-medium transition-colors"
						>
							{isUploading ? (
								<Loader2 className="h-3 w-3 animate-spin" />
							) : (
								<Plus className="h-3 w-3" />
							)}
							Add Resource
						</button>
					</div>
				</div>

				{selectedCourse.resources && selectedCourse.resources.length > 0 ? (
					<div className="flex flex-wrap gap-2">
						{selectedCourse.resources.map((r) => (
							<span key={r.id} className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-muted/50 text-sm border border-border/50">
								<FileText className="h-3 w-3 text-muted-foreground" />
								{r.filename}
							</span>
						))}
					</div>
				) : (
					<p className="text-sm text-muted-foreground italic">No resources yet. Add text files to improve AI context.</p>
				)}
			</div>

			{/* Sessions */}
			<div className="space-y-4">
				<h2 className="font-semibold text-foreground">Study Sessions</h2>

				{loadingSessions ? (
					<div className="flex items-center justify-center py-8">
						<Loader2 className="h-6 w-6 animate-spin text-primary" />
					</div>
				) : sessions.length === 0 ? (
					<div className="text-center py-8 space-y-3">
						<GraduationCap className="h-10 w-10 mx-auto text-muted-foreground/50" />
						<p className="text-muted-foreground">No sessions yet</p>
						<button
							onClick={handleCreateSession}
							className="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors"
						>
							<Sparkles className="h-4 w-4" />
							Create First Session
						</button>
					</div>
				) : (
					<div className="grid gap-3">
						{sessions.map((session) => (
							<div key={session.id} className="relative group">
								<button
									onClick={() => handleSelectSession(session)}
									className="w-full p-4 rounded-xl border border-border/50 bg-card/30 hover:bg-card/60 hover:border-accent/40 hover:shadow-lg hover:shadow-accent/5 hover:scale-[1.02] transition-all duration-200 cursor-pointer text-left"
								>
									<div className="flex items-start gap-3">
										<div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-accent/10 border border-accent/20 group-hover:bg-accent/20 group-hover:border-accent/40 transition-colors">
											<GraduationCap className="h-4 w-4 text-accent" />
										</div>
										<div className="min-w-0 flex-1">
											<h3 className="font-medium text-foreground group-hover:text-accent transition-colors">{session.topic}</h3>
											<div className="flex items-center gap-3 mt-1">
												{session.keywords.length > 0 && (
													<div className="flex gap-1">
														{session.keywords.slice(0, 3).map((kw, i) => (
															<span key={i} className="px-1.5 py-0.5 rounded bg-primary/10 text-primary text-xs">
																{kw}
															</span>
														))}
													</div>
												)}
												<span className="text-xs text-muted-foreground uppercase">{session.language}</span>
												<span className={`text-xs ${session.status === 'completed' ? 'text-green-500' : 'text-yellow-500'}`}>
													{session.status}
												</span>
											</div>
										</div>
									</div>
								</button>
								{/* Delete button - appears on hover */}
								<button
									onClick={(e) => handleDeleteSession(e, session.id)}
									disabled={deletingSessionId === session.id}
									className="absolute top-2 right-2 p-1.5 rounded-lg bg-red-500/10 text-red-500 opacity-0 group-hover:opacity-100 hover:bg-red-500/20 transition-all z-10 cursor-pointer"
									title="Delete session"
								>
									{deletingSessionId === session.id ? (
										<Loader2 className="h-3.5 w-3.5 animate-spin" />
									) : (
										<Trash2 className="h-3.5 w-3.5" />
									)}
								</button>
							</div>
						))}
					</div>
				)}
			</div>
		</div>
	)
}
