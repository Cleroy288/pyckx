"use client"

import { useState, useCallback, useMemo } from "react"
import { CheckCircle, XCircle, ArrowRight, RotateCcw, Trophy } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { QcmQuestion } from "@/lib/api/intello"

interface QcmPlayerProps {
  questions: QcmQuestion[]
  title: string
  onBack: () => void
}

export function QcmPlayer({ questions, title, onBack }: QcmPlayerProps) {
  const [currentIndex, setCurrentIndex] = useState(0)
  const [selectedAnswer, setSelectedAnswer] = useState<string | null>(null)
  const [showResult, setShowResult] = useState(false)
  const [score, setScore] = useState(0)
  const [finished, setFinished] = useState(false)

  const currentQuestion = questions[currentIndex]

  // Shuffle answers for current question
  const shuffledAnswers = useMemo(() => {
    if (!currentQuestion) return []
    const allAnswers = [...currentQuestion.wrong_answers, currentQuestion.right_answer]
    return allAnswers.sort(() => Math.random() - 0.5)
  }, [currentQuestion])

  const handleAnswerSelect = useCallback((answer: string) => {
    if (showResult) return
    setSelectedAnswer(answer)
  }, [showResult])

  const handleSubmit = useCallback(() => {
    if (!selectedAnswer || !currentQuestion) return
    setShowResult(true)
    if (selectedAnswer === currentQuestion.right_answer) {
      setScore((prev) => prev + 1)
    }
  }, [selectedAnswer, currentQuestion])

  const handleNext = useCallback(() => {
    if (currentIndex < questions.length - 1) {
      setCurrentIndex((prev) => prev + 1)
      setSelectedAnswer(null)
      setShowResult(false)
    } else {
      setFinished(true)
    }
  }, [currentIndex, questions.length])

  const handleRestart = useCallback(() => {
    setCurrentIndex(0)
    setSelectedAnswer(null)
    setShowResult(false)
    setScore(0)
    setFinished(false)
  }, [])

  const getAnswerStyle = (answer: string) => {
    if (!showResult) {
      return selectedAnswer === answer
        ? "border-primary bg-primary/10"
        : "border-border/50 hover:border-accent/50"
    }
    if (answer === currentQuestion?.right_answer) {
      return "border-green-500 bg-green-500/10"
    }
    if (answer === selectedAnswer && answer !== currentQuestion?.right_answer) {
      return "border-red-500 bg-red-500/10"
    }
    return "border-border/30 opacity-50"
  }

  // Finished screen
  if (finished) {
    const percentage = Math.round((score / questions.length) * 100)
    return (
      <div className="flex flex-col items-center justify-center py-12 space-y-6">
        <Trophy className="h-16 w-16 text-yellow-500" />
        <h2 className="text-2xl font-bold">Quiz Complete!</h2>
        <div className="text-center">
          <p className="text-4xl font-bold text-primary">{score}/{questions.length}</p>
          <p className="text-muted-foreground mt-1">{percentage}% correct</p>
        </div>
        <div className="flex gap-3 pt-4">
          <Button variant="outline" onClick={onBack}>
            Back to Form
          </Button>
          <Button onClick={handleRestart} className="gap-2">
            <RotateCcw className="h-4 w-4" />
            Play Again
          </Button>
        </div>
      </div>
    )
  }

  if (!currentQuestion) {
    return <div>No questions available</div>
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h3 className="font-semibold">{title}</h3>
        <span className="text-sm text-muted-foreground">
          Question {currentIndex + 1} of {questions.length}
        </span>
      </div>

      {/* Progress bar */}
      <div className="h-2 bg-border/30 rounded-full overflow-hidden">
        <div
          className="h-full bg-primary transition-all duration-300"
          style={{ width: `${((currentIndex + 1) / questions.length) * 100}%` }}
        />
      </div>

      {/* Question */}
      <div className="p-6 rounded-xl bg-card/60 border border-border/50">
        <p className="text-lg font-medium">{currentQuestion.question}</p>
      </div>

      {/* Answers */}
      <div className="space-y-3">
        {shuffledAnswers.map((answer, i) => (
          <button
            key={i}
            onClick={() => handleAnswerSelect(answer)}
            disabled={showResult}
            className={`w-full p-4 rounded-xl border-2 text-left transition-all ${getAnswerStyle(answer)}`}
          >
            <div className="flex items-center gap-3">
              <span className="flex-shrink-0 w-8 h-8 rounded-full bg-background border border-border/50 flex items-center justify-center text-sm font-medium">
                {String.fromCharCode(65 + i)}
              </span>
              <span className="flex-1">{answer}</span>
              {showResult && answer === currentQuestion.right_answer && (
                <CheckCircle className="h-5 w-5 text-green-500" />
              )}
              {showResult && answer === selectedAnswer && answer !== currentQuestion.right_answer && (
                <XCircle className="h-5 w-5 text-red-500" />
              )}
            </div>
          </button>
        ))}
      </div>

      {/* Explanation */}
      {showResult && (
        <div className="p-4 rounded-xl bg-accent/10 border border-accent/20">
          <p className="text-sm font-medium text-accent mb-1">Explanation</p>
          <p className="text-sm text-muted-foreground">{currentQuestion.explanation}</p>
        </div>
      )}

      {/* Actions */}
      <div className="flex gap-3 pt-2">
        <Button variant="outline" onClick={onBack}>
          Exit Quiz
        </Button>
        {!showResult ? (
          <Button onClick={handleSubmit} disabled={!selectedAnswer} className="flex-1">
            Check Answer
          </Button>
        ) : (
          <Button onClick={handleNext} className="flex-1 gap-2">
            {currentIndex < questions.length - 1 ? (
              <>
                Next Question
                <ArrowRight className="h-4 w-4" />
              </>
            ) : (
              <>
                See Results
                <Trophy className="h-4 w-4" />
              </>
            )}
          </Button>
        )}
      </div>

      {/* Score */}
      <div className="text-center text-sm text-muted-foreground">
        Current score: {score}/{currentIndex + (showResult ? 1 : 0)}
      </div>
    </div>
  )
}
