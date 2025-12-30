"use client"

import { useState, useCallback } from "react"
import { ArrowLeft, ArrowRight, RotateCcw, Trophy, Eye, EyeOff } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import type { Flashcard } from "@/lib/api/intello"

interface FlashcardPlayerProps {
  cards: Flashcard[]
  title: string
  onBack: () => void
}

export function FlashcardPlayer({ cards, title, onBack }: FlashcardPlayerProps) {
  const [currentIndex, setCurrentIndex] = useState(0)
  const [isFlipped, setIsFlipped] = useState(false)
  const [showResults, setShowResults] = useState(false)

  const currentCard = cards[currentIndex]
  const progress = ((currentIndex + 1) / cards.length) * 100

  const handleFlip = useCallback(() => {
    setIsFlipped((prev) => !prev)
  }, [])

  const handleNext = useCallback(() => {
    if (currentIndex < cards.length - 1) {
      setCurrentIndex((prev) => prev + 1)
      setIsFlipped(false)
    } else {
      setShowResults(true)
    }
  }, [currentIndex, cards.length])

  const handlePrev = useCallback(() => {
    if (currentIndex > 0) {
      setCurrentIndex((prev) => prev - 1)
      setIsFlipped(false)
    }
  }, [currentIndex])

  const handlePlayAgain = useCallback(() => {
    setCurrentIndex(0)
    setIsFlipped(false)
    setShowResults(false)
  }, [])

  // Results view
  if (showResults) {
    return (
      <div className="space-y-6 max-w-3xl mx-auto">
        <div className="rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-8 text-center space-y-4">
          <div className="flex justify-center">
            <div className="flex h-20 w-20 items-center justify-center rounded-full bg-green-500/20 border-2 border-green-500/30">
              <Trophy className="h-10 w-10 text-green-500" />
            </div>
          </div>
          <div>
            <h2 className="text-2xl font-bold text-foreground mb-1">Session Complete!</h2>
            <p className="text-muted-foreground">{title}</p>
          </div>
          <p className="text-muted-foreground">
            You reviewed all {cards.length} cards
          </p>
          <div className="flex gap-3 justify-center pt-4">
            <Button variant="outline" onClick={onBack} className="gap-2">
              <ArrowLeft className="h-4 w-4" />
              Back
            </Button>
            <Button onClick={handlePlayAgain} className="gap-2 bg-accent text-accent-foreground hover:bg-accent/90">
              <RotateCcw className="h-4 w-4" />
              Study Again
            </Button>
          </div>
        </div>
      </div>
    )
  }

  // Main flashcard view
  return (
    <div className="space-y-6 max-w-3xl mx-auto">
      {/* Header */}
      <div className="flex items-center justify-between">
        <Button variant="ghost" size="sm" onClick={onBack} className="gap-2">
          <ArrowLeft className="h-4 w-4" />
          Exit
        </Button>
        <div className="text-sm text-muted-foreground">
          Card {currentIndex + 1} of {cards.length}
        </div>
      </div>

      {/* Progress */}
      <div className="space-y-2">
        <Progress value={progress} className="h-2" />
        <div className="flex justify-between text-xs text-muted-foreground">
          <span>{title}</span>
          <span>{currentIndex + 1} / {cards.length}</span>
        </div>
      </div>

      {/* Flashcard with flip animation */}
      <div
        onClick={handleFlip}
        className="cursor-pointer"
        style={{ perspective: "1000px" }}
      >
        <div
          className="relative w-full min-h-[300px] transition-transform duration-500"
          style={{
            transformStyle: "preserve-3d",
            transform: isFlipped ? "rotateY(180deg)" : "rotateY(0deg)",
          }}
        >
          {/* Front side */}
          <div
            className="absolute inset-0 rounded-2xl border border-accent/30 bg-card/60 backdrop-blur-sm p-8 flex flex-col items-center justify-center"
            style={{
              backfaceVisibility: "hidden",
              WebkitBackfaceVisibility: "hidden",
            }}
          >
            <div className="text-xs text-muted-foreground mb-4 flex items-center gap-2">
              <Eye className="h-4 w-4" />
              Question - Click to see answer
            </div>
            <p className="text-xl font-semibold text-foreground text-center leading-relaxed">
              {currentCard.front}
            </p>
          </div>

          {/* Back side */}
          <div
            className="absolute inset-0 rounded-2xl border border-green-500/30 bg-card/60 backdrop-blur-sm p-8 flex flex-col items-center justify-center"
            style={{
              backfaceVisibility: "hidden",
              WebkitBackfaceVisibility: "hidden",
              transform: "rotateY(180deg)",
            }}
          >
            <div className="text-xs text-muted-foreground mb-4 flex items-center gap-2">
              <EyeOff className="h-4 w-4" />
              Answer - Click to flip back
            </div>
            <p className="text-xl text-foreground text-center leading-relaxed">
              {currentCard.back}
            </p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <div className="flex items-center justify-between pt-4">
        <Button
          variant="outline"
          onClick={handlePrev}
          disabled={currentIndex === 0}
          className="gap-2"
        >
          <ArrowLeft className="h-4 w-4" />
          Previous
        </Button>

        <Button
          onClick={handleNext}
          className="gap-2 bg-accent text-accent-foreground hover:bg-accent/90"
        >
          {currentIndex < cards.length - 1 ? (
            <>
              Next
              <ArrowRight className="h-4 w-4" />
            </>
          ) : (
            <>
              Finish
              <Trophy className="h-4 w-4" />
            </>
          )}
        </Button>
      </div>

      {/* Card Navigator */}
      <div className="flex flex-wrap gap-2 justify-center">
        {cards.map((card, index) => {
          const isCurrent = index === currentIndex
          return (
            <button
              key={card.id}
              onClick={() => {
                setCurrentIndex(index)
                setIsFlipped(false)
              }}
              className={`w-8 h-8 rounded-lg text-sm font-medium transition-all ${
                isCurrent
                  ? "bg-accent text-accent-foreground"
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
