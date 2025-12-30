"use client"

import { useEffect, useState } from "react"
import { BookOpen, Plus, ArrowLeft, Loader2, Clock, FileText, GraduationCap, Sparkles } from "lucide-react"
import { useIntello } from "../../context"
import { listSessions, type SessionData } from "@/lib/api/intello"

export function CourseDetailView() {
	const { selectedCourse, loading, error, handleBackToHome, handleCreateSession, handleSelectSession, handleNavigateToCourses } = useIntello()
	const [sessions, setSessions] = useState<SessionData[]>([])
	const [loadingSessions, setLoadingSessions] = useState(false)

	useEffect(() => {
		if (selectedCourse) {
			setLoadingSessions(true)
			listSessions(selectedCourse.id)
				.then(setSessions)
				.catch(console.error)
				.finally(() => setLoadingSessions(false))
		}
	}, [selectedCourse])

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
			<div className="flex items-center justify-between">
				<div className="flex items-center gap-3">
					<button onClick={handleNavigateToCourses} className="p-2 rounded-lg hover:bg-muted/50 transition-colors">
						<ArrowLeft className="h-5 w-5" />
					</button>
					<div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<BookOpen className="h-5 w-5 text-primary" />
					</div>
					<div>
						<h1 className="text-xl font-bold tracking-tight text-foreground">{selectedCourse.name}</h1>
						{selectedCourse.description && (
							<p className="text-xs text-muted-foreground">{selectedCourse.description}</p>
						)}
					</div>
				</div>
				<button
					onClick={handleCreateSession}
					className="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
				>
					<Sparkles className="h-4 w-4" />
					New Session
				</button>
			</div>

			{/* Resources */}
			{selectedCourse.resources && selectedCourse.resources.length > 0 && (
				<div className="p-4 rounded-xl border border-border/50 bg-card/30">
					<h2 className="font-semibold text-foreground mb-3">Resources ({selectedCourse.resources.length})</h2>
					<div className="flex flex-wrap gap-2">
						{selectedCourse.resources.map((r) => (
							<span key={r.id} className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-muted/50 text-sm">
								<FileText className="h-3 w-3 text-muted-foreground" />
								{r.filename}
							</span>
						))}
					</div>
				</div>
			)}

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
							<button
								key={session.id}
								onClick={() => handleSelectSession(session)}
								className="group p-4 rounded-xl border border-border/50 bg-card/30 hover:bg-card/60 hover:border-accent/40 hover:shadow-lg hover:shadow-accent/5 hover:scale-[1.02] transition-all duration-200 cursor-pointer text-left"
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
						))}
					</div>
				)}
			</div>
		</div>
	)
}
