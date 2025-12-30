"use client"

import { useState, useCallback } from "react"
import { Upload, X, Loader2, Send, Plus, CheckCircle2, Play } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {
  createCustomQuestion,
  createOpenQuestions,
  createFlashcards,
  createTrueOrFalse,
  createKeywords,
  createOrderPhrase,
  createFillBlank,
  type Level,
  type GameData,
  type NumQuestions,
  type QcmQuestion,
  type OpenQuestion,
  type Flashcard,
  type TrueOrFalseStatement,
  type KeywordQuestionData,
  type OrderPhraseQuestionData,
  type FillBlankQuestionData,
  VALID_NUM_QUESTIONS,
} from "@/lib/api/intello"
import { QcmPlayer } from "./qcm-player"
import { OpenQuestionPlayer } from "./open-question-player"
import { FlashcardPlayer } from "./flashcard-player"
import { TrueOrFalsePlayer } from "./true-false-player"
import { KeywordsPlayer } from "./keywords-player"
import { OrderPhrasePlayer } from "./order-phrase-player"
import { FillBlankPlayer } from "./fill-blank-player"

interface CustomQuestionFormProps {
  games: GameData[]
  onBack: () => void
  /** Pre-set the output game type (hides the selector) */
  defaultOutputGame?: string
  /** Callback to navigate to a specific game page */
  onNavigateToGame?: (gameType: "qcm" | "open" | "flashcard" | "true_false" | "keywords" | "order_phrase" | "fill_blank") => void
}

export function CustomQuestionForm({ games, onBack, defaultOutputGame, onNavigateToGame }: CustomQuestionFormProps) {
  // Form state
  const [name, setName] = useState("")
  const [description, setDescription] = useState("")
  const [instructions, setInstructions] = useState("")
  const [language, setLanguage] = useState("en")
  const [level, setLevel] = useState<Level>("medium")
  const [outputGame, setOutputGame] = useState(defaultOutputGame || games[0]?.id || "qcm")
  const [numQuestions, setNumQuestions] = useState<NumQuestions>(10)
  const [subjects, setSubjects] = useState<string[]>([])
  const [newSubject, setNewSubject] = useState("")
  const [files, setFiles] = useState<File[]>([])

  // UI state
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Success state - shows success view with navigation button
  const [successData, setSuccessData] = useState<{
    message: string
    gameType: "qcm" | "open" | "flashcard" | "true_false" | "keywords" | "order_phrase" | "fill_blank"
    itemCount: number
    setName: string
  } | null>(null)

  // Game state - QCM
  const [generatedQuestions, setGeneratedQuestions] = useState<QcmQuestion[] | null>(null)
  const [quizTitle, setQuizTitle] = useState("")

  // Game state - Open Questions
  const [openQuestions, setOpenQuestions] = useState<OpenQuestion[] | null>(null)
  const [openQuestionSetId, setOpenQuestionSetId] = useState("")

  // Game state - Flashcards
  const [flashcards, setFlashcards] = useState<Flashcard[] | null>(null)
  const [flashcardTitle, setFlashcardTitle] = useState("")

  // Game state - True or False
  const [trueOrFalseStatements, setTrueOrFalseStatements] = useState<TrueOrFalseStatement[] | null>(null)
  const [trueOrFalseTitle, setTrueOrFalseTitle] = useState("")

  // Game state - Keywords
  const [keywordQuestions, setKeywordQuestions] = useState<KeywordQuestionData[] | null>(null)
  const [keywordsTitle, setKeywordsTitle] = useState("")

  // Game state - Order Phrase
  const [orderPhraseQuestions, setOrderPhraseQuestions] = useState<OrderPhraseQuestionData[] | null>(null)
  const [orderPhraseTitle, setOrderPhraseTitle] = useState("")

  // Game state - Fill Blank
  const [fillBlankQuestions, setFillBlankQuestions] = useState<FillBlankQuestionData[] | null>(null)
  const [fillBlankTitle, setFillBlankTitle] = useState("")

  const handleAddSubject = useCallback(() => {
    if (newSubject.trim() && subjects.length < 3) {
      setSubjects((prev) => [...prev, newSubject.trim()])
      setNewSubject("")
    }
  }, [newSubject, subjects.length])

  const handleRemoveSubject = useCallback((index: number) => {
    setSubjects((prev) => prev.filter((_, i) => i !== index))
  }, [])

  const handleFileChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFiles = Array.from(e.target.files || [])
    const validFiles = selectedFiles.filter((file) => {
      const ext = file.name.toLowerCase()
      return ext.endsWith(".txt") || ext.endsWith(".pdf") || ext.endsWith(".docx") || ext.endsWith(".pptx")
    })
    setFiles((prev) => [...prev, ...validFiles])
  }, [])

  const handleRemoveFile = useCallback((index: number) => {
    setFiles((prev) => prev.filter((_, i) => i !== index))
  }, [])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    setSuccessData(null)

    // Validation
    if (!name.trim()) {
      setError("Name is required")
      return
    }
    if (!description.trim()) {
      setError("Description is required")
      return
    }
    if (files.length === 0) {
      setError("At least one file is required")
      return
    }

    setLoading(true)

    try {
      // Handle different output game types
      if (outputGame === "open_question") {
        // Create open questions
        const response = await createOpenQuestions(
          {
            name: name.trim(),
            description: description.trim(),
            instructions: instructions.trim(),
            language,
            level,
            subjects,
            num_questions: numQuestions,
          },
          files
        )

        // Show success view with navigation option
        setSuccessData({
          message: `Successfully generated ${response.questions.length} open questions!`,
          gameType: "open",
          itemCount: response.questions.length,
          setName: name.trim(),
        })
        // Also store for immediate play option
        if (response.questions && response.questions.length > 0) {
          setQuizTitle(name.trim())
          setOpenQuestionSetId(response.id)
          setOpenQuestions(response.questions)
        }
      } else if (outputGame === "flashcard") {
        // Create flashcards
        const response = await createFlashcards(
          {
            name: name.trim(),
            description: description.trim(),
            instructions: instructions.trim(),
            language,
            level,
            subjects,
            num_questions: numQuestions,
          },
          files
        )

        // Show success view with navigation option
        setSuccessData({
          message: `Successfully generated ${response.cards.length} flashcards!`,
          gameType: "flashcard",
          itemCount: response.cards.length,
          setName: name.trim(),
        })
        // Also store for immediate play option
        if (response.cards && response.cards.length > 0) {
          setFlashcardTitle(name.trim())
          setFlashcards(response.cards)
        }
      } else if (outputGame === "true_false") {
        // Create true/false statements
        const response = await createTrueOrFalse(
          {
            name: name.trim(),
            description: description.trim(),
            instructions: instructions.trim(),
            language,
            level,
            subjects,
            num_questions: numQuestions,
          },
          files
        )

        // Show success view with navigation option
        setSuccessData({
          message: `Successfully generated ${response.statements.length} true/false statements!`,
          gameType: "true_false",
          itemCount: response.statements.length,
          setName: name.trim(),
        })
        // Also store for immediate play option
        if (response.statements && response.statements.length > 0) {
          setTrueOrFalseTitle(name.trim())
          setTrueOrFalseStatements(response.statements)
        }
      } else if (outputGame === "keywords") {
        // Create keywords questions
        const response = await createKeywords(
          {
            name: name.trim(),
            description: description.trim(),
            instructions: instructions.trim(),
            language,
            level,
            subjects,
            num_questions: numQuestions,
          },
          files
        )

        // Show success view with navigation option
        setSuccessData({
          message: `Successfully generated ${response.questions.length} keyword questions!`,
          gameType: "keywords",
          itemCount: response.questions.length,
          setName: name.trim(),
        })
        // Also store for immediate play option
        if (response.questions && response.questions.length > 0) {
          setKeywordsTitle(name.trim())
          setKeywordQuestions(response.questions)
        }
      } else if (outputGame === "order_phrase") {
        // Create order phrase questions
        const response = await createOrderPhrase(
          {
            name: name.trim(),
            description: description.trim(),
            instructions: instructions.trim(),
            language,
            level,
            subjects,
            num_questions: numQuestions,
          },
          files
        )

        // Show success view with navigation option
        setSuccessData({
          message: `Successfully generated ${response.questions.length} order phrase questions!`,
          gameType: "order_phrase",
          itemCount: response.questions.length,
          setName: name.trim(),
        })
        // Also store for immediate play option
        if (response.questions && response.questions.length > 0) {
          setOrderPhraseTitle(name.trim())
          setOrderPhraseQuestions(response.questions)
        }
      } else if (outputGame === "fill_blank") {
        // Create fill blank questions
        const response = await createFillBlank(
          {
            name: name.trim(),
            description: description.trim(),
            instructions: instructions.trim(),
            language,
            level,
            subjects,
            num_questions: numQuestions,
          },
          files
        )

        // Show success view with navigation option
        setSuccessData({
          message: `Successfully generated ${response.questions.length} fill-in-the-blank questions!`,
          gameType: "fill_blank",
          itemCount: response.questions.length,
          setName: name.trim(),
        })
        // Also store for immediate play option
        if (response.questions && response.questions.length > 0) {
          setFillBlankTitle(name.trim())
          setFillBlankQuestions(response.questions)
        }
      } else {
        // Create QCM questions (default)
        const response = await createCustomQuestion(
          {
            name: name.trim(),
            description: description.trim(),
            instructions: instructions.trim(),
            language,
            level,
            output_game: outputGame,
            subjects,
            num_questions: numQuestions,
          },
          files
        )

        // Show success view with navigation option
        setSuccessData({
          message: `Successfully generated ${response.questions.length} QCM questions!`,
          gameType: "qcm",
          itemCount: response.questions.length,
          setName: name.trim(),
        })
        // Also store for immediate play option
        if (response.questions && response.questions.length > 0) {
          setQuizTitle(name.trim())
          setGeneratedQuestions(response.questions)
        }
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Failed to create custom question"
      // Check for rate limit error
      if (errorMessage.includes("rate limit") || errorMessage.includes("429")) {
        setError("⏱️ Please wait 1 minute before creating another AI resource")
      } else {
        setError(errorMessage)
      }
    } finally {
      setLoading(false)
    }
  }

  const getFileIcon = (filename: string) => {
    if (filename.endsWith(".pdf")) return "📄"
    if (filename.endsWith(".docx")) return "📝"
    if (filename.endsWith(".pptx")) return "📊"
    return "📃"
  }

  const handleBackFromPlayer = useCallback(() => {
    setGeneratedQuestions(null)
    setOpenQuestions(null)
    setOpenQuestionSetId("")
    setFlashcards(null)
    setFlashcardTitle("")
    setTrueOrFalseStatements(null)
    setTrueOrFalseTitle("")
    setKeywordQuestions(null)
    setKeywordsTitle("")
    setOrderPhraseQuestions(null)
    setOrderPhraseTitle("")
    setFillBlankQuestions(null)
    setFillBlankTitle("")
    setQuizTitle("")
    setSuccessData(null)
    // Reset form for new quiz
    setName("")
    setDescription("")
    setInstructions("")
    setSubjects([])
    setFiles([])
  }, [])

  const handleNavigateToGamePage = useCallback(() => {
    if (successData && onNavigateToGame) {
      onNavigateToGame(successData.gameType)
    }
  }, [successData, onNavigateToGame])

  const handlePlayNow = useCallback(() => {
    // Clear success data to show the player
    setSuccessData(null)
  }, [])

  // Show success view with navigation options
  if (successData) {
    const gameTypeLabels = {
      qcm: "QCM Sets",
      open: "Open Question Sets",
      flashcard: "Flashcard Sets",
      true_false: "True or False Sets",
      keywords: "Keyword Sets",
      order_phrase: "Order Phrase Sets",
      fill_blank: "Fill in the Blank Sets",
    }

    return (
      <div className="space-y-6 text-center py-8">
        <div className="flex justify-center">
          <div className="flex h-16 w-16 items-center justify-center rounded-full bg-green-500/20 border-2 border-green-500/30">
            <CheckCircle2 className="h-8 w-8 text-green-500" />
          </div>
        </div>

        <div>
          <h2 className="text-xl font-bold text-foreground mb-2">
            {successData.message}
          </h2>
          <p className="text-muted-foreground">
            &quot;{successData.setName}&quot; has been saved and is ready to use.
          </p>
        </div>

        <div className="flex flex-col gap-3 pt-4 max-w-xs mx-auto">
          {/* Play Now button - plays immediately with generated content */}
          <Button
            onClick={handlePlayNow}
            className="gap-2 bg-accent text-accent-foreground hover:bg-accent/90"
          >
            <Play className="h-4 w-4" />
            Play Now
          </Button>

          {/* Go to game page button */}
          {onNavigateToGame && (
            <Button
              variant="outline"
              onClick={handleNavigateToGamePage}
              className="gap-2"
            >
              Go to {gameTypeLabels[successData.gameType]}
            </Button>
          )}

          {/* Create another */}
          <Button
            variant="ghost"
            onClick={handleBackFromPlayer}
          >
            Create Another
          </Button>
        </div>
      </div>
    )
  }

  // Show QCM player if we have generated QCM questions (after clicking Play Now)
  if (generatedQuestions && generatedQuestions.length > 0 && !successData) {
    return (
      <QcmPlayer
        questions={generatedQuestions}
        title={quizTitle}
        onBack={handleBackFromPlayer}
      />
    )
  }

  // Show Open Question player if we have generated open questions (after clicking Play Now)
  if (openQuestions && openQuestions.length > 0 && !successData) {
    return (
      <OpenQuestionPlayer
        questions={openQuestions}
        title={quizTitle}
        setId={openQuestionSetId}
        onBack={handleBackFromPlayer}
      />
    )
  }

  // Show Flashcard player if we have generated flashcards (after clicking Play Now)
  if (flashcards && flashcards.length > 0 && !successData) {
    return (
      <FlashcardPlayer
        cards={flashcards}
        title={flashcardTitle}
        onBack={handleBackFromPlayer}
      />
    )
  }

  // Show True or False player if we have generated statements (after clicking Play Now)
  if (trueOrFalseStatements && trueOrFalseStatements.length > 0 && !successData) {
    return (
      <TrueOrFalsePlayer
        statements={trueOrFalseStatements}
        title={trueOrFalseTitle}
        onBack={handleBackFromPlayer}
      />
    )
  }

  // Show Keywords player if we have generated questions (after clicking Play Now)
  if (keywordQuestions && keywordQuestions.length > 0 && !successData) {
    return (
      <KeywordsPlayer
        questions={keywordQuestions}
        title={keywordsTitle}
        onBack={handleBackFromPlayer}
      />
    )
  }

  // Show Order Phrase player if we have generated questions (after clicking Play Now)
  if (orderPhraseQuestions && orderPhraseQuestions.length > 0 && !successData) {
    return (
      <OrderPhrasePlayer
        questions={orderPhraseQuestions}
        title={orderPhraseTitle}
        onBack={handleBackFromPlayer}
      />
    )
  }

  // Show Fill Blank player if we have generated questions (after clicking Play Now)
  if (fillBlankQuestions && fillBlankQuestions.length > 0 && !successData) {
    return (
      <FillBlankPlayer
        questions={fillBlankQuestions}
        title={fillBlankTitle}
        onBack={handleBackFromPlayer}
      />
    )
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Name */}
      <div className="space-y-2">
        <Label htmlFor="name">Name *</Label>
        <Input
          id="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="My Custom Quiz"
          disabled={loading}
        />
      </div>

      {/* Description */}
      <div className="space-y-2">
        <Label htmlFor="description">Description *</Label>
        <Textarea
          id="description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="What kind of questions should be generated?"
          rows={2}
          disabled={loading}
        />
      </div>

      {/* Instructions */}
      <div className="space-y-2">
        <Label htmlFor="instructions">AI Instructions</Label>
        <Textarea
          id="instructions"
          value={instructions}
          onChange={(e) => setInstructions(e.target.value)}
          placeholder="Specific instructions for the AI (e.g., 'Focus on chapter 3', 'Make questions about dates')"
          rows={3}
          disabled={loading}
        />
      </div>

      {/* Language & Level */}
      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label>Language</Label>
          <Select value={language} onValueChange={setLanguage} disabled={loading}>
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="en">English</SelectItem>
              <SelectItem value="fr">French</SelectItem>
              <SelectItem value="es">Spanish</SelectItem>
              <SelectItem value="de">German</SelectItem>
              <SelectItem value="nl">Dutch</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="space-y-2">
          <Label>Difficulty</Label>
          <Select value={level} onValueChange={(v) => setLevel(v as Level)} disabled={loading}>
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="easy">Easy</SelectItem>
              <SelectItem value="medium">Medium</SelectItem>
              <SelectItem value="hard">Hard</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      {/* Output Game & Number of Questions */}
      <div className={defaultOutputGame ? "" : "grid grid-cols-2 gap-4"}>
        {!defaultOutputGame && (
          <div className="space-y-2">
            <Label>Output Game</Label>
            <Select value={outputGame} onValueChange={setOutputGame} disabled={loading}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {games.map((game) => (
                  <SelectItem key={game.id} value={game.id}>
                    {game.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        )}

        <div className="space-y-2">
          <Label>Number of Questions</Label>
          <Select
            value={numQuestions.toString()}
            onValueChange={(v) => setNumQuestions(Number(v) as NumQuestions)}
            disabled={loading}
          >
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {VALID_NUM_QUESTIONS.map((num) => (
                <SelectItem key={num} value={num.toString()}>
                  {num} questions
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>

      {/* Subjects */}
      <div className="space-y-2">
        <Label>Subjects (max 3)</Label>
        <div className="flex gap-2">
          <Input
            value={newSubject}
            onChange={(e) => setNewSubject(e.target.value)}
            placeholder="Add a subject"
            disabled={loading || subjects.length >= 3}
            onKeyDown={(e) => e.key === "Enter" && (e.preventDefault(), handleAddSubject())}
          />
          <Button
            type="button"
            variant="outline"
            size="icon"
            onClick={handleAddSubject}
            disabled={loading || subjects.length >= 3 || !newSubject.trim()}
          >
            <Plus className="h-4 w-4" />
          </Button>
        </div>
        {subjects.length > 0 && (
          <div className="flex flex-wrap gap-2 mt-2">
            {subjects.map((subject, i) => (
              <span
                key={i}
                className="inline-flex items-center gap-1 px-2 py-1 text-sm rounded-full bg-primary/10 text-primary border border-primary/20"
              >
                {subject}
                <button
                  type="button"
                  onClick={() => handleRemoveSubject(i)}
                  className="hover:text-destructive"
                >
                  <X className="h-3 w-3" />
                </button>
              </span>
            ))}
          </div>
        )}
      </div>

      {/* File Upload */}
      <div className="space-y-2">
        <Label>Documents * (PDF, Word, PowerPoint, TXT)</Label>
        <div className="border-2 border-dashed border-border/50 rounded-xl p-6 text-center hover:border-accent/50 transition-colors">
          <input
            type="file"
            multiple
            accept=".txt,.pdf,.docx,.pptx"
            onChange={handleFileChange}
            className="hidden"
            id="file-upload"
            disabled={loading}
          />
          <label htmlFor="file-upload" className="cursor-pointer">
            <Upload className="h-8 w-8 mx-auto text-muted-foreground mb-2" />
            <p className="text-sm text-muted-foreground">
              Click to upload or drag and drop
            </p>
            <p className="text-xs text-muted-foreground mt-1">
              PDF, Word (.docx), PowerPoint (.pptx), or Text files
            </p>
          </label>
        </div>

        {files.length > 0 && (
          <div className="space-y-2 mt-3">
            {files.map((file, i) => (
              <div
                key={i}
                className="flex items-center justify-between p-3 rounded-lg bg-card/40 border border-border/50"
              >
                <div className="flex items-center gap-2">
                  <span>{getFileIcon(file.name)}</span>
                  <span className="text-sm truncate max-w-[200px]">{file.name}</span>
                  <span className="text-xs text-muted-foreground">
                    ({(file.size / 1024).toFixed(1)} KB)
                  </span>
                </div>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  className="h-6 w-6"
                  onClick={() => handleRemoveFile(i)}
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Error Message */}
      {error && (
        <div className="rounded-lg bg-destructive/10 border border-destructive/20 p-3">
          <p className="text-sm text-destructive">{error}</p>
        </div>
      )}

      {/* Actions */}
      <div className="flex gap-3 pt-4">
        <Button type="button" variant="outline" onClick={onBack} disabled={loading}>
          Back
        </Button>
        <Button type="submit" disabled={loading} className="flex-1 gap-2">
          {loading ? (
            <>
              <Loader2 className="h-4 w-4 animate-spin" />
              Processing...
            </>
          ) : (
            <>
              <Send className="h-4 w-4" />
              Generate Questions
            </>
          )}
        </Button>
      </div>
    </form>
  )
}
