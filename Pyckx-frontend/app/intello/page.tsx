"use client"

import { useEffect } from "react"
import { useRouter } from "next/navigation"
import { GridBackground } from "@/components/grid-background"
import { TopNavigation } from "@/components/top-navigation"
import { AuthGuard } from "@/components/auth-guard"
import { useUserApps } from "@/lib/user-apps-context"
import { IntelloProvider, useIntello } from "./context"
import {
  HomeView,
  CreateQcmView, CreateAiQcmView, CreateAiOpenView, CreateAiFlashcardView, CreateAiTrueOrFalseView, CreateAiKeywordsView, CreateAiOrderPhraseView, CreateAiFillBlankView,
  PlayQcmView, PlayOpenView, PlayFlashcardView, PlayTrueOrFalseView, PlayKeywordsView, PlayOrderPhraseView, PlayFillBlankView,
  PlayingQcmView, PlayingOpenView, PlayingFlashcardView, PlayingTrueOrFalseView, PlayingKeywordsView, PlayingOrderPhraseView, PlayingFillBlankView,
  ResultsView
} from "./views"

// == Main Page Content ==
function IntelloContent() {
  const router = useRouter()
  const { currentPage } = useUserApps()
  const { view, loadGames } = useIntello()

  useEffect(() => {
    if (currentPage.type === "dashboard") router.push("/")
  }, [currentPage, router])

  useEffect(() => {
    loadGames()
  }, [loadGames])

  return (
    <main className="container mx-auto px-6 py-6">
      {view === "home" && <HomeView />}
      {view === "create-qcm" && <CreateQcmView />}
      {view === "create-ai-qcm" && <CreateAiQcmView />}
      {view === "create-ai-open" && <CreateAiOpenView />}
      {view === "create-ai-flashcard" && <CreateAiFlashcardView />}
      {view === "create-ai-true-false" && <CreateAiTrueOrFalseView />}
      {view === "create-ai-keywords" && <CreateAiKeywordsView />}
      {view === "create-ai-order-phrase" && <CreateAiOrderPhraseView />}
      {view === "create-ai-fill-blank" && <CreateAiFillBlankView />}
      {view === "play-qcm" && <PlayQcmView />}
      {view === "play-open" && <PlayOpenView />}
      {view === "play-flashcard" && <PlayFlashcardView />}
      {view === "play-true-false" && <PlayTrueOrFalseView />}
      {view === "play-keywords" && <PlayKeywordsView />}
      {view === "play-order-phrase" && <PlayOrderPhraseView />}
      {view === "play-fill-blank" && <PlayFillBlankView />}
      {view === "playing-qcm" && <PlayingQcmView />}
      {view === "playing-open" && <PlayingOpenView />}
      {view === "playing-flashcard" && <PlayingFlashcardView />}
      {view === "playing-true-false" && <PlayingTrueOrFalseView />}
      {view === "playing-keywords" && <PlayingKeywordsView />}
      {view === "playing-order-phrase" && <PlayingOrderPhraseView />}
      {view === "playing-fill-blank" && <PlayingFillBlankView />}
      {view === "results" && <ResultsView />}
    </main>
  )
}

// == Page Wrapper ==
export default function IntelloPage() {
  return (
    <AuthGuard>
      <IntelloProvider>
        <div className="relative min-h-screen overflow-hidden selection:bg-primary/30">
          <GridBackground />
          <div className="relative z-10">
            <TopNavigation />
            <IntelloContent />
          </div>
        </div>
      </IntelloProvider>
    </AuthGuard>
  )
}
