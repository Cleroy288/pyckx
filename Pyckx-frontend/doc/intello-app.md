# Intello App Documentation

## Overview

Intello is an educational game platform within Pyckx that offers AI-powered quiz generation. Users can create quizzes from their own documents and test their knowledge.

## Features

### 1. QCM (Multiple Choice Questions)
- Pre-made quiz sets stored in the backend
- 4 answer options per question (1 correct, 3 wrong)
- Immediate feedback with explanations
- Score tracking and results review

### 2. AI QCM
- Upload documents (PDF, Word, PowerPoint, TXT)
- AI generates multiple choice questions from content
- Customizable: subjects, difficulty, language, number of questions
- **Select AI model** for generation (6 free models available)
- Questions target specific subjects from the source material

### 3. AI Open Questions
- Upload documents for AI analysis
- AI generates open-ended questions requiring written answers
- Hints available to guide users
- AI grades answers with feedback (right/medium/error)
- Detailed results with constructive feedback

### 4. AI Flashcards
- Upload documents for AI analysis
- AI generates flashcards with front/back content
- Study mode with card flipping
- Great for memorization and quick recall

### 5. AI True or False
- Upload documents for AI analysis
- AI generates true/false statements from content
- Each statement includes explanation
- Simple True/False button gameplay
- ~50/50 mix of true and false statements

### 6. AI Keywords
- Upload documents for AI analysis
- AI generates statements with keyword options
- Users identify which keywords are correct
- Tests comprehension and terminology understanding
- Each question includes explanation of correct keywords

### 7. AI Order Phrase
- Upload documents for AI analysis
- AI generates phrases that are split into shuffled words
- Users arrange words in the correct order
- Tests sentence structure and comprehension
- Each phrase includes optional hint

## Page Structure

```
app/intello/
├── page.tsx              # Router (~72 lines)
├── context.tsx           # IntelloProvider + useIntello hook
└── views/
    ├── index.ts          # Barrel export
    ├── home-view.tsx     # Create + Play buttons
    ├── create-views.tsx  # 6 create view components
    ├── play-views.tsx    # 5 set selection views
    ├── playing-views.tsx # 5 active game views
    └── results-view.tsx  # QCM results view
```

### View Flow
```
/intello
├── HomeView (default)
│   ├── CreateQcmView / CreateAi*View → Sets View → Playing View
│   └── PlayQcmView / PlayOpenView / PlayFlashcardView / PlayTrueOrFalseView / PlayKeywordsView
│       └── Playing*View → ResultsView (for QCM)
```

## Components

### IntelloPage (`app/intello/page.tsx`)

Minimal router component (~65 lines) that:
- Wraps content in `IntelloProvider`
- Routes to views based on `view` state from context

### IntelloProvider (`app/intello/context.tsx`)

React context managing all Intello state:
- View state (`home`, `create-*`, `play-*`, `playing-*`, `results`)
- Loading/error state
- Games and sets data (QCM, Open, Flashcard, TrueOrFalse, Keywords)
- Selected sets for playing
- QCM quiz state (current question, answers, score)
- Load functions and navigation handlers

**Hook: `useIntello()`** - Access context in any view component.

### View Components (`app/intello/views/`)

| File | Components |
|------|------------|
| `home-view.tsx` | HomeView |
| `create-views.tsx` | CreateQcmView, CreateAiQcmView, CreateAiOpenView, CreateAiFlashcardView, CreateAiTrueOrFalseView, CreateAiKeywordsView, CreateAiOrderPhraseView |
| `play-views.tsx` | PlayQcmView, PlayOpenView, PlayFlashcardView, PlayTrueOrFalseView, PlayKeywordsView, PlayOrderPhraseView |
| `playing-views.tsx` | PlayingQcmView, PlayingOpenView, PlayingFlashcardView, PlayingTrueOrFalseView, PlayingKeywordsView, PlayingOrderPhraseView |
| `results-view.tsx` | ResultsView |

### CustomQuestionForm (`components/custom-question-form.tsx`)

Form for creating AI-generated content:

**Props:**
```typescript
interface CustomQuestionFormProps {
  games: GameData[]
  onBack: () => void
  defaultOutputGame?: string  // "qcm", "open_question", "flashcard", "true_false", "keywords", "order_phrase"
  onNavigateToGame?: (gameType: "qcm" | "open" | "flashcard" | "true_false" | "keywords" | "order_phrase") => void
}
```

**Form Fields:**
- Name (required)
- Description (required)
- AI Instructions (optional)
- Language (en, fr, es, de, nl)
- Difficulty (easy, medium, hard)
- Number of Questions (5, 10, 15, 20, 25, 30)
- **AI Model** (selectable from 6 free models)
- Subjects (max 3)
- Documents (required, multiple files)

### Player Components

| Component | File | Description |
|-----------|------|-------------|
| QcmPlayer | `qcm-player.tsx` | Multiple choice quiz |
| OpenQuestionPlayer | `open-question-player.tsx` | Open-ended questions with AI grading |
| FlashcardPlayer | `flashcard-player.tsx` | Flashcard study mode |
| TrueOrFalsePlayer | `true-false-player.tsx` | True/False statement game |
| KeywordsPlayer | `keywords-player.tsx` | Keyword identification game |
| OrderPhrasePlayer | `order-phrase-player.tsx` | Word ordering game |

## Classes (`lib/classes/intello/`)

### Entity Classes

| Class | File | Description |
|-------|------|-------------|
| QcmSet | `QcmSet.ts` | QCM set with questions |
| QcmQuestion | `QcmQuestion.ts` | Individual QCM question |
| OpenQuestionSet | `OpenQuestionSet.ts` | Open question set |
| OpenQuestion | `OpenQuestion.ts` | Individual open question |
| TrueOrFalseSet | `TrueOrFalseSet.ts` | True/false set with statements |
| TrueOrFalseStatement | `TrueOrFalseStatement.ts` | Individual statement |
| FlashcardSet | `FlashcardSet.ts` | Flashcard set with cards |
| Flashcard | `Flashcard.ts` | Individual flashcard |
| KeywordSet | `KeywordSet.ts` | Keyword set with questions |
| KeywordQuestion | `KeywordQuestion.ts` | Individual keyword question |
| Keyword | `Keyword.ts` | Individual keyword option |

## API Functions (`lib/api/intello.ts`)

### QCM Functions
```typescript
getAllQcmSets(): Promise<QcmSetData[]>
addQcmSet(input: CreateQcmSetInput): Promise<QcmSetData>
modifyQcmSet(id: string, input: UpdateQcmSetInput): Promise<QcmSetData>
deleteQcmSet(id: string): Promise<void>
getAvailableGames(): Promise<GameData[]>
createCustomQuestion(input, files): Promise<CustomQuestionResponse>
```

### Open Question Functions
```typescript
createOpenQuestions(input, files): Promise<CreateOpenQuestionResponse>
getAllOpenQuestionSets(): Promise<OpenQuestionSetData[]>
checkOpenQuestionAnswers(setId, answers): Promise<CheckAnswersResponse>
```

### Flashcard Functions
```typescript
createFlashcards(input, files): Promise<CreateFlashcardResponse>
getAllFlashcardSets(): Promise<FlashcardSetData[]>
```

### True or False Functions
```typescript
createTrueOrFalse(input, files): Promise<CreateTrueOrFalseResponse>
getAllTrueOrFalseSets(): Promise<TrueOrFalseSetData[]>
```

### Keywords Functions
```typescript
getAllKeywordSets(): Promise<KeywordSetData[]>
createKeywords(input, files): Promise<CreateKeywordsResponse>
```

### Order Phrase Functions
```typescript
getAllOrderPhraseSets(): Promise<OrderPhraseSetListResponse>
createOrderPhrase(input, files): Promise<CreateOrderPhraseResponse>
```

### AI Model Functions
```typescript
getAvailableModels(): Promise<AvailableModelsResponse>
```

## Types

### Statement/Question Types
```typescript
interface TrueOrFalseStatement {
  id: string
  statement: string
  answer: boolean       // true = statement is true
  explanation: string
}

interface Flashcard {
  id: string
  front: string
  back: string
}

interface Keyword {
  id: string
  word: string
  is_correct: boolean
}

interface KeywordQuestion {
  id: string
  statement: string
  keywords: Keyword[]
  explanation: string
}

interface OrderPhraseWord {
  id: string
  word: string
  position: number
}

interface OrderPhraseQuestion {
  id: string
  original_phrase: string
  words: OrderPhraseWord[]
  hint?: string
}
```

## API Endpoints

Configured in `lib/api/config.ts`:

```typescript
intello: {
  games: () => "/app/intello/games",
  models: () => "/app/intello/models",
  qcmSets: () => "/app/intello/qcm",
  customQuestion: () => `${BACKEND}/app/intello/custom-question`,
  openQuestionCreate: () => `${BACKEND}/app/intello/open-question/create`,
  openQuestionList: () => `${BACKEND}/app/intello/open-question/list`,
  openQuestionCheck: () => `${BACKEND}/app/intello/open-question/check`,
  flashcardCreate: () => `${BACKEND}/app/intello/flashcard/create`,
  flashcardList: () => `${BACKEND}/app/intello/flashcard/list`,
  trueOrFalseCreate: () => `${BACKEND}/app/intello/true-false/create`,
  trueOrFalseList: () => `${BACKEND}/app/intello/true-false/list`,
  keywordsCreate: () => `${BACKEND}/app/intello/keywords/create`,
  keywordsList: () => `${BACKEND}/app/intello/keywords/list`,
  orderPhraseCreate: () => `${BACKEND}/app/intello/order-phrase/create`,
  orderPhraseList: () => `${BACKEND}/app/intello/order-phrase/list`,
}
```

## Styling

### Game Cards
- Manual QCM: `accent` color (teal/cyan)
- AI Games: `primary` color (purple)

### Grade Colors
- Right/True: `green-500`
- Medium: `yellow-500`
- Error/False: `red-500`

## User Experience Flow

### AI True or False Flow
1. Click "AI True or False" card
2. Fill form (name, description, settings)
3. Upload documents
4. Click "Generate Questions"
5. Wait for AI processing
6. Play through statements with True/False buttons
7. View results and explanations

## Supported File Types

| Type | Extensions |
|------|------------|
| Text | `.txt` |
| PDF | `.pdf` |
| Word | `.docx` |
| PowerPoint | `.pptx` |

