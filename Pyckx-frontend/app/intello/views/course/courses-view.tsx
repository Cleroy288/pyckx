"use client"

import { useEffect, useState } from "react"
import { BookOpen, Plus, ArrowLeft, Loader2, Clock, FileText, Trash2 } from "lucide-react"
import { useIntello } from "../../context"
import { deleteCourse } from "@/lib/api/intello"

export function CoursesView() {
	const { courses, loading, error, handleBackToHome, handleSelectCourse, loadCourses, handleNavigateToCreateCourse } = useIntello()
	const [deletingId, setDeletingId] = useState<string | null>(null)

	useEffect(() => {
		loadCourses()
	}, [loadCourses])

	const handleDeleteCourse = async (e: React.MouseEvent, courseId: string) => {
		e.stopPropagation()
		if (!confirm("Are you sure you want to delete this course? This will remove all associated sessions and resources.")) return

		setDeletingId(courseId)
		try {
			await deleteCourse(courseId)
			loadCourses() // Refresh the list
		} catch (err) {
			console.error("Failed to delete course:", err)
			alert("Failed to delete course.")
		} finally {
			setDeletingId(null)
		}
	}

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
						<div key={course.id} className="relative group">
							<button
								onClick={() => handleSelectCourse(course)}
								className="w-full p-4 rounded-xl border border-border/50 bg-card/30 hover:bg-card/60 hover:border-primary/40 hover:shadow-lg hover:shadow-primary/5 hover:scale-[1.02] transition-all duration-200 cursor-pointer text-left"
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
							{/* Delete button - appears on hover */}
							<button
								onClick={(e) => handleDeleteCourse(e, course.id)}
								disabled={deletingId === course.id}
								className="absolute top-2 right-2 p-1.5 rounded-lg bg-red-500/10 text-red-500 opacity-0 group-hover:opacity-100 hover:bg-red-500/20 transition-all z-10 cursor-pointer"
								title="Delete course"
							>
								{deletingId === course.id ? (
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
	)
}
