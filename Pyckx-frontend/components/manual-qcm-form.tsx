"use client"

import { useState } from "react"
import { Plus, Trash2, CheckCircle, XCircle, Loader2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { addQcmSet } from "@/lib/api/intello"
import { useToast } from "@/hooks/use-toast"
import type { Level, CreateQcmQuestionInput } from "@/lib/classes/intello"

interface ManualQcmFormProps {
	onBack: () => void
	onSuccess?: (setId: string) => void
}

// Empty question template
const createEmptyQuestion = (): CreateQcmQuestionInput => ({
	question: "",
	right_answer: "",
	wrong_answers: ["", "", ""],
	explanation: "",
})

export function ManualQcmForm({ onBack, onSuccess }: ManualQcmFormProps) {
	const { toast } = useToast()
	const [isSubmitting, setIsSubmitting] = useState(false)

	// Form state
	const [name, setName] = useState("")
	const [description, setDescription] = useState("")
	const [language, setLanguage] = useState("en")
	const [level, setLevel] = useState<Level>("medium")
	const [subjectInput, setSubjectInput] = useState("")
	const [subjects, setSubjects] = useState<string[]>([])
	const [questions, setQuestions] = useState<CreateQcmQuestionInput[]>([createEmptyQuestion()])

	// Subject handling
	const addSubject = () => {
		const trimmed = subjectInput.trim()
		if (trimmed && subjects.length < 3 && !subjects.includes(trimmed)) {
			setSubjects([...subjects, trimmed])
			setSubjectInput("")
		}
	}

	const removeSubject = (index: number) => {
		setSubjects(subjects.filter((_, i) => i !== index))
	}

	// Question handling
	const addQuestion = () => {
		setQuestions([...questions, createEmptyQuestion()])
	}

	const removeQuestion = (index: number) => {
		if (questions.length > 1) {
			setQuestions(questions.filter((_, i) => i !== index))
		}
	}

	const updateQuestion = (index: number, field: keyof CreateQcmQuestionInput, value: string) => {
		const updated = [...questions]
		if (field === "wrong_answers") return // Handle separately
		updated[index] = { ...updated[index], [field]: value }
		setQuestions(updated)
	}

	const updateWrongAnswer = (qIndex: number, wIndex: number, value: string) => {
		const updated = [...questions]
		const newWrongAnswers = [...updated[qIndex].wrong_answers]
		newWrongAnswers[wIndex] = value
		updated[qIndex] = { ...updated[qIndex], wrong_answers: newWrongAnswers }
		setQuestions(updated)
	}

	// Validation
	const isValid = () => {
		if (!name.trim()) return false
		if (!description.trim()) return false
		for (const q of questions) {
			if (!q.question.trim()) return false
			if (!q.right_answer.trim()) return false
			if (q.wrong_answers.some((w) => !w.trim())) return false
			if (!q.explanation.trim()) return false
		}
		return true
	}

	// Submit
	const handleSubmit = async () => {
		if (!isValid()) {
			toast({ title: "Validation Error", description: "Please fill all required fields", variant: "destructive" })
			return
		}

		setIsSubmitting(true)
		try {
			const result = await addQcmSet({
				name: name.trim(),
				description: description.trim(),
				level,
				language,
				subjects: subjects.length > 0 ? subjects : undefined,
				questions,
			})
			toast({ title: "Success", description: `Created QCM set with ${questions.length} questions` })
			onSuccess?.(result.id)
		} catch (error) {
			toast({
				title: "Error",
				description: error instanceof Error ? error.message : "Failed to create QCM set",
				variant: "destructive",
			})
		} finally {
			setIsSubmitting(false)
		}
	}

	return (
		<div className="space-y-6">
			{/* Set Details */}
			<div className="space-y-4">
				<div className="grid gap-4 sm:grid-cols-2">
					<div className="space-y-2">
						<Label htmlFor="name">Name *</Label>
						<Input
							id="name"
							value={name}
							onChange={(e) => setName(e.target.value)}
							placeholder="My QCM Set"
						/>
					</div>
					<div className="space-y-2">
						<Label htmlFor="language">Language</Label>
						<Select value={language} onValueChange={setLanguage}>
							<SelectTrigger><SelectValue /></SelectTrigger>
							<SelectContent>
								<SelectItem value="en">English</SelectItem>
								<SelectItem value="fr">French</SelectItem>
								<SelectItem value="es">Spanish</SelectItem>
								<SelectItem value="de">German</SelectItem>
								<SelectItem value="nl">Dutch</SelectItem>
							</SelectContent>
						</Select>
					</div>
				</div>

				<div className="space-y-2">
					<Label htmlFor="description">Description *</Label>
					<Textarea
						id="description"
						value={description}
						onChange={(e) => setDescription(e.target.value)}
						placeholder="Describe your quiz..."
						rows={2}
					/>
				</div>

				<div className="grid gap-4 sm:grid-cols-2">
					<div className="space-y-2">
						<Label htmlFor="level">Difficulty</Label>
						<Select value={level} onValueChange={(v) => setLevel(v as Level)}>
							<SelectTrigger><SelectValue /></SelectTrigger>
							<SelectContent>
								<SelectItem value="easy">Easy</SelectItem>
								<SelectItem value="medium">Medium</SelectItem>
								<SelectItem value="hard">Hard</SelectItem>
							</SelectContent>
						</Select>
					</div>
					<div className="space-y-2">
						<Label>Subjects (max 3)</Label>
						<div className="flex gap-2">
							<Input
								value={subjectInput}
								onChange={(e) => setSubjectInput(e.target.value)}
								onKeyDown={(e) => e.key === "Enter" && (e.preventDefault(), addSubject())}
								placeholder="Add subject"
								disabled={subjects.length >= 3}
							/>
							<Button type="button" size="icon" variant="outline" onClick={addSubject} disabled={subjects.length >= 3}>
								<Plus className="h-4 w-4" />
							</Button>
						</div>
						{subjects.length > 0 && (
							<div className="flex flex-wrap gap-2 mt-2">
								{subjects.map((s, i) => (
									<span key={i} className="inline-flex items-center gap-1 px-2 py-1 text-xs bg-accent/20 rounded-md">
										{s}
										<button type="button" onClick={() => removeSubject(i)} className="hover:text-destructive">
											<XCircle className="h-3 w-3" />
										</button>
									</span>
								))}
							</div>
						)}
					</div>
				</div>
			</div>

			{/* Questions */}
			<div className="space-y-4">
				<div className="flex items-center justify-between">
					<h3 className="text-lg font-semibold">Questions ({questions.length})</h3>
					<Button type="button" size="sm" variant="outline" onClick={addQuestion}>
						<Plus className="h-4 w-4 mr-1" /> Add Question
					</Button>
				</div>

				{questions.map((q, qIndex) => (
					<div
						key={qIndex}
						className="rounded-lg border border-border/50 bg-card/30 backdrop-blur-sm p-4 space-y-3"
					>
						<div className="flex items-center justify-between">
							<span className="text-sm font-medium text-muted-foreground">Question {qIndex + 1}</span>
							{questions.length > 1 && (
								<Button type="button" size="icon" variant="ghost" onClick={() => removeQuestion(qIndex)}>
									<Trash2 className="h-4 w-4 text-destructive" />
								</Button>
							)}
						</div>

						<Input
							value={q.question}
							onChange={(e) => updateQuestion(qIndex, "question", e.target.value)}
							placeholder="Question text *"
						/>

						<div className="grid gap-2">
							<div className="flex items-center gap-2">
								<CheckCircle className="h-4 w-4 text-green-500 shrink-0" />
								<Input
									value={q.right_answer}
									onChange={(e) => updateQuestion(qIndex, "right_answer", e.target.value)}
									placeholder="Correct answer *"
									className="border-green-500/30"
								/>
							</div>
							{q.wrong_answers.map((w, wIndex) => (
								<div key={wIndex} className="flex items-center gap-2">
									<XCircle className="h-4 w-4 text-red-400 shrink-0" />
									<Input
										value={w}
										onChange={(e) => updateWrongAnswer(qIndex, wIndex, e.target.value)}
										placeholder={`Wrong answer ${wIndex + 1} *`}
										className="border-red-400/30"
									/>
								</div>
							))}
						</div>

						<Textarea
							value={q.explanation}
							onChange={(e) => updateQuestion(qIndex, "explanation", e.target.value)}
							placeholder="Explanation (shown after answer) *"
							rows={2}
						/>
					</div>
				))}
			</div>

			{/* Actions */}
			<div className="flex gap-3">
				<Button type="button" variant="outline" onClick={onBack} className="flex-1">
					Cancel
				</Button>
				<Button
					type="button"
					onClick={handleSubmit}
					disabled={!isValid() || isSubmitting}
					className="flex-1"
				>
					{isSubmitting && <Loader2 className="h-4 w-4 mr-2 animate-spin" />}
					Create QCM Set
				</Button>
			</div>
		</div>
	)
}
