"use client"

import { useEffect } from "react"
import { BookOpen, Plus, ArrowLeft, Loader2, Clock, FileText } from "lucide-react"
import { useIntello } from "../../context"

export function CoursesView() {
	const { courses, loading, error, handleBackToHome, handleSelectCourse, loadCourses, handleNavigateToCreateCourse } = useIntello()

	useEffect(() => {
		loadCourses()
	}, [loadCourses])

	return (
		<div className="space-y-6">
			{/* Header */}
			<div className="flex items-center justify-between">
				<div className="flex items-center gap-3">
					<button onClick={handleBackToHome} className="p-2 rounded-lg hover:bg-muted/50 transition-colors">
						<ArrowLeft className="h-5 w-5" />
					</button>
					<div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
						<BookOpen className="h-5 w-5 text-primary" />
					</div>
					<div>
						<h1 className="text-xl font-bold tracking-tight text-foreground">My Courses</h1>
						<p className="text-xs text-muted-foreground">Organize your study materials</p>
					</div>
				</div>
				<button
					onClick={handleNavigateToCreateCourse}
					className="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 transition-colors text-sm font-medium"
				>
					<Plus className="h-4 w-4" />
					Create Course
				</button>
			</div>

			{/* Error */}
			{error && (
				<div className="p-4 rounded-lg bg-destructive/10 border border-destructive/20 text-destructive text-sm">
					{error}
				</div>
			)}

			{/* Loading */}
			{loading ? (
				<div className="flex items-center justify-center py-12">
					<Loader2 className="h-8 w-8 animate-spin text-primary" />
				</div>
			) : courses.length === 0 ? (
				/* Empty State */
				<div className="text-center py-12 space-y-4">
					<BookOpen className="h-12 w-12 mx-auto text-muted-foreground/50" />
					<p className="text-muted-foreground">No courses yet</p>
					<p className="text-sm text-muted-foreground">Create your first course to get started</p>
				</div>
			) : (
				/* Course List */
				<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
					{courses.map((course) => (
						<button
							key={course.id}
							onClick={() => handleSelectCourse(course)}
							className="group p-4 rounded-xl border border-border/50 bg-card/30 hover:bg-card/60 hover:border-primary/40 hover:shadow-lg hover:shadow-primary/5 hover:scale-[1.02] transition-all duration-200 cursor-pointer text-left"
						>
							<div className="flex items-start gap-3">
								<div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-primary/10 border border-primary/20 group-hover:bg-primary/20 group-hover:border-primary/40 transition-colors">
									<BookOpen className="h-5 w-5 text-primary" />
								</div>
								<div className="min-w-0 flex-1">
									<h3 className="font-semibold text-foreground truncate group-hover:text-primary transition-colors">{course.name}</h3>
									{course.description && (
										<p className="text-sm text-muted-foreground line-clamp-2">{course.description}</p>
									)}
									<div className="flex items-center gap-2 mt-2 text-xs text-muted-foreground">
										<Clock className="h-3 w-3" />
										<span>{new Date(course.created_at).toLocaleDateString()}</span>
									</div>
								</div>
							</div>
						</button>
					))}
				</div>
			)}
		</div>
	)
}
