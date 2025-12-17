"use client"

import { useState, useCallback, useMemo } from "react"
import { ArrowLeft, ArrowRight, Loader2, CheckCircle2, XCircle, AlertCircle, Trophy, RotateCcw, Lightbulb, Send } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Progress } from "@/components/ui/progress"
import {
  checkOpenQuestionAnswers,
  type OpenQuestion,
  type GradedAnswer,
  type UserAnswer,
  type GradeLevel,
} from "@/lib/api/intello"

interface OpenQuestionPlayerProps {
  questions: OpenQuestion[]
  title: string
  setId: string
  onBack: () => void
}

type ViewState = "answering" | "grading" | "results"

interface AnswerState {
  questionId: string
  answer: string
}

export function OpenQuestionPlayer({ questions, title, setId, onBack }: OpenQuestionPlayerProps) {
  const [view, setView] = useState<ViewState>("answering")
  const [currentIndex, setCurrentIndex] = useState(0)
  const [answers, setAnswers] = useState<AnswerState[]>(
    questions.map(q => ({ questionId: q.id, answer: "" }))
  )
  const [showHint, setShowHint] = useState(false)
  const [grades, setGrades] = useState<GradedAnswer[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const currentQuestion = questions[currentIndex]
  const currentAnswer = answers[currentIndex]
  const progress = ((currentIndex + 1) / questions.length) * 100

  const handleAnswerChange = useCallback((value: string) => {
    setAnswers(prev => prev.map((a, i) => 
      i === currentIndex ? { ...a, answer: value } : a
    ))
  }, [currentIndex])

  const handleNext = useCallback(() => {
    if (currentIndex < questions.length - 1) {
      setCurrentIndex(prev => prev + 1)
      setShowHint(false)
    }
  }, [currentIndex, questions.length])

  const handlePrev = useCallback(() => {
    if (currentIndex > 0) {
      setCurrentIndex(prev => prev - 1)
      setShowHint(false)
    }
  }, [currentIndex])

  const handleSubmit = useCallback(async () => {
    setLoading(true)
    setError(null)
    setView("grading")

    try {
      const userAnswers: UserAnswer[] = answers.map(a => ({
        question_id: a.questionId,
        user_answer: a.answer,
      }))

      const response = await checkOpenQuestionAnswers(setId, userAnswers)
      setGrades(response.grades)
      setView("results")
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to grade answers")
      setView("answering")
    } finally {
      setLoading(false)
    }
  }, [answers, setId])

  const handlePlayAgain = useCallback(() => {
    setAnswers(questions.map(q => ({ questionId: q.id, answer: "" })))
    setCurrentIndex(0)
    setShowHint(false)
    setGrades([])
    setView("answering")
  }, [questions])

  const score = useMemo(() => {
    if (grades.length === 0) return { right: 0, medium: 0, error: 0, total: 0, percentage: 0 }
    const right = grades.filter(g => g.grade === "right").length
    const medium = grades.filter(g => g.grade === "medium").length
    const errorCount = grades.filter(g => g.grade === "error").length
    const percentage = Math.round(((right + medium * 0.5) / grades.length) * 100)
    return { right, medium, error: errorCount, total: grades.length, percentage }
  }, [grades])

  const getGradeIcon = (grade: GradeLevel) => {
    switch (grade) {
      case "right": return <CheckCircle2 className="h-5 w-5 text-green-500" />
      case "medium": return <AlertCircle className="h-5 w-5 text-yellow-500" />
      case "error": return <XCircle className="h-5 w-5 text-red-500" />
    }
  }

  const getGradeColor = (grade: GradeLevel) => {
    switch (grade) {
      case "right": return "border-green-500/30 bg-green-500/5"
      case "medium": return "border-yellow-500/30 bg-yellow-500/5"
      case "error": return "border-red-500/30 bg-red-500/5"
    }
  }

  const getGradeLabel = (grade: GradeLevel) => {
    switch (grade) {
      case "right": return "Correct"
      case "medium": return "Partially Correct"
      case "error": return "Incorrect"
    }
  }

  const answeredCount = answers.filter(a => a.answer.trim().length > 0).length

  // == GRADING VIEW ==
  if (view === "grading") {
    return (
      <div className="flex flex-col items-center justify-center py-16 space-y-4">
        <Loader2 className="h-12 w-12 animate-spin text-accent" />
        <h2 className="text-xl font-semibold text-foreground">Grading your answers...</h2>
        <p className="text-sm text-muted-foreground">AI is reviewing your responses</p>
      </div>
    )
  }

  // == RESULTS VIEW ==
  if (view === "results") {
    return (
      <div className="space-y-6 max-w-3xl mx-auto">
        {/* Results Header */}
        <div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-8 text-center space-y-4">
          <div className="flex justify-center">
            <div className={`flex h-20 w-20 items-center justify-center rounded-full ${score.percentage >= 60 ? 'bg-green-500/20 border-2 border-green-500/30' : 'bg-red-500/20 border-2 border-red-500/30'}`}>
              <Trophy className={`h-10 w-10 ${score.percentage >= 60 ? 'text-green-500' : 'text-red-500'}`} />
            </div>
          </div>
          <div>
            <h2 className="text-2xl font-bold text-foreground mb-1">Quiz Complete!</h2>
            <p className="text-muted-foreground">{title}</p>
          </div>
          <div className={`text-5xl font-bold ${score.percentage >= 80 ? 'text-green-500' : score.percentage >= 60 ? 'text-yellow-500' : 'text-red-500'}`}>
            {score.percentage}%
          </div>
          <div className="flex justify-center gap-6 text-sm">
            <div className="flex items-center gap-2">
              <CheckCircle2 className="h-4 w-4 text-green-500" />
              <span className="text-muted-foreground">{score.right} correct</span>
            </div>
            <div className="flex items-center gap-2">
              <AlertCircle className="h-4 w-4 text-yellow-500" />
              <span className="text-muted-foreground">{score.medium} partial</span>
            </div>
            <div className="flex items-center gap-2">
              <XCircle className="h-4 w-4 text-red-500" />
              <span className="text-muted-foreground">{score.error} incorrect</span>
            </div>
          </div>
          
          <div className="flex gap-3 justify-center pt-4">
            <Button variant="outline" onClick={onBack} className="gap-2">
              <ArrowLeft className="h-4 w-4" />
              Back
            </Button>
            <Button onClick={handlePlayAgain} className="gap-2 bg-accent text-accent-foreground hover:bg-accent/90">
              <RotateCcw className="h-4 w-4" />
              Try Again
            </Button>
          </div>
        </div>

        {/* Detailed Results */}
        <div className="space-y-4">
          <h3 className="text-lg font-semibold text-foreground">Review Your Answers</h3>
          {grades.map((grade, index) => {
            const question = questions.find(q => q.id === grade.question_id)
            const answer = answers.find(a => a.questionId === grade.question_id)
            if (!question) return null

            return (
              <div key={grade.question_id} className={`rounded-xl border p-4 ${getGradeColor(grade.grade)}`}>
                <div className="flex items-start gap-3 mb-3">
                  <div className="shrink-0 mt-0.5">{getGradeIcon(grade.grade)}</div>
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-2">
                      <span className="text-sm font-medium text-muted-foreground">Q{index + 1}</span>
                      <span className={`text-xs px-2 py-0.5 rounded-full ${
                        grade.grade === "right" ? "bg-green-500/20 text-green-500" :
                        grade.grade === "medium" ? "bg-yellow-500/20 text-yellow-500" :
                        "bg-red-500/20 text-red-500"
                      }`}>
                        {getGradeLabel(grade.grade)}
                      </span>
                    </div>
                    <p className="font-medium text-foreground mb-3">{question.question}</p>
                    
                    <div className="space-y-2 text-sm">
                      <div className="bg-background/30 rounded-lg p-3">
                        <span className="font-medium text-foreground/80">Your answer:</span>
                        <p className="text-muted-foreground mt-1">{answer?.answer || "(No answer)"}</p>
                      </div>
                      
                      {question.expected_answer && (
                        <div className="bg-green-500/10 rounded-lg p-3">
                          <span className="font-medium text-green-500">Expected answer:</span>
                          <p className="text-muted-foreground mt-1">{question.expected_answer}</p>
                        </div>
                      )}
                      
                      <div className="bg-accent/10 rounded-lg p-3">
                        <span className="font-medium text-accent">Feedback:</span>
                        <p className="text-muted-foreground mt-1">{grade.feedback}</p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    )
  }

  // == ANSWERING VIEW ==
  return (
    <div className="space-y-6 max-w-3xl mx-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <Button variant="ghost" size="sm" onClick={onBack} className="gap-2">
          <ArrowLeft className="h-4 w-4" />
          Exit Quiz
        </Button>
        <div className="text-sm text-muted-foreground">
          Question {currentIndex + 1} of {questions.length}
        </div>
      </div>

      {/* Progress */}
      <div className="space-y-2">
        <Progress value={progress} className="h-2" />
        <div className="flex justify-between text-xs text-muted-foreground">
          <span>{title}</span>
          <span>{answeredCount}/{questions.length} answered</span>
        </div>
      </div>

      {/* Question Card */}
      <div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-6 space-y-6">
        <h2 className="text-xl font-semibold text-foreground leading-relaxed">
          {currentQuestion.question}
        </h2>

        {/* Hint Toggle */}
        {currentQuestion.hint && (
          <div>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowHint(!showHint)}
              className="gap-2 text-accent hover:text-accent"
            >
              <Lightbulb className="h-4 w-4" />
              {showHint ? "Hide Hint" : "Show Hint"}
            </Button>
            {showHint && (
              <div className="mt-2 rounded-lg bg-accent/10 border border-accent/20 p-3">
                <p className="text-sm text-muted-foreground">{currentQuestion.hint}</p>
              </div>
            )}
          </div>
        )}

        {/* Answer Input */}
        <div className="space-y-2">
          <Textarea
            value={currentAnswer.answer}
            onChange={(e) => handleAnswerChange(e.target.value)}
            placeholder="Type your answer here..."
            rows={5}
            className="resize-none"
          />
          <p className="text-xs text-muted-foreground text-right">
            {currentAnswer.answer.length} characters
          </p>
        </div>

        {/* Error Message */}
        {error && (
          <div className="rounded-lg bg-destructive/10 border border-destructive/20 p-3">
            <p className="text-sm text-destructive">{error}</p>
          </div>
        )}

        {/* Navigation */}
        <div className="flex items-center justify-between pt-4 border-t border-border/30">
          <Button
            variant="outline"
            onClick={handlePrev}
            disabled={currentIndex === 0}
            className="gap-2"
          >
            <ArrowLeft className="h-4 w-4" />
            Previous
          </Button>

          <div className="flex gap-2">
            {currentIndex < questions.length - 1 ? (
              <Button onClick={handleNext} className="gap-2 bg-accent text-accent-foreground hover:bg-accent/90">
                Next
                <ArrowRight className="h-4 w-4" />
              </Button>
            ) : (
              <Button
                onClick={handleSubmit}
                disabled={loading || answeredCount === 0}
                className="gap-2 bg-primary text-primary-foreground hover:bg-primary/90"
              >
                {loading ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin" />
                    Submitting...
                  </>
                ) : (
                  <>
                    <Send className="h-4 w-4" />
                    Submit Answers
                  </>
                )}
              </Button>
            )}
          </div>
        </div>
      </div>

      {/* Question Navigator */}
      <div className="flex flex-wrap gap-2 justify-center">
        {questions.map((_, index) => {
          const isAnswered = answers[index].answer.trim().length > 0
          const isCurrent = index === currentIndex
          return (
            <button
              key={index}
              onClick={() => { setCurrentIndex(index); setShowHint(false) }}
              className={`w-8 h-8 rounded-lg text-sm font-medium transition-all ${
                isCurrent
                  ? "bg-accent text-accent-foreground"
                  : isAnswered
                  ? "bg-green-500/20 text-green-500 border border-green-500/30"
                  : "bg-card/40 text-muted-foreground border border-border/50 hover:border-accent/50"
              }`}
            >
              {index + 1}
            </button>
          )
        })}
      </div>
    </div>
  )
}
