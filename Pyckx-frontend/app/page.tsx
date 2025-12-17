"use client"

import { useState, useEffect } from "react"
import { GridBackground } from "@/components/grid-background"
import { TopNavigation } from "@/components/top-navigation"
import { AppGrid } from "@/components/app-grid"
import { AppDetails } from "@/components/app-details"
import { useRouter } from "next/navigation"
import { AuthGuard } from "@/components/auth-guard"
import { availableApps, getAppById, type App } from "@/lib/apps-data"
import { useUserApps } from "@/lib/user-apps-context"

export default function Home() {
  const [selectedApp, setSelectedApp] = useState<App | null>(null)
  const { currentPage, navigateTo } = useUserApps()
  const router = useRouter()

  // Handle app navigation in useEffect to avoid setState during render
  useEffect(() => {
    if (currentPage.type === "app") {
      const app = getAppById(currentPage.appId)
      if (app?.id === "collection") {
        router.push("/collection")
      } else if (app?.id === "intello") {
        router.push("/intello")
      }
    }
  }, [currentPage, router])

  const handleAppClick = (app: App) => {
    setSelectedApp(app)
  }

  const handleBackToDashboard = () => {
    setSelectedApp(null)
    navigateTo({ type: "dashboard" })
  }

  const renderContent = () => {
    // If viewing app details (from explore section)
    if (selectedApp && currentPage.type === "dashboard") {
      return <AppDetails app={selectedApp} onBack={handleBackToDashboard} />
    }

    // If on dashboard
    if (currentPage.type === "dashboard") {
      return <AppGrid otherApps={[...availableApps]} onAppClick={handleAppClick} />
    }

    // If on an app page (user has added the app) - show loading while redirecting
    if (currentPage.type === "app") {
      const app = getAppById(currentPage.appId)
      if (app?.id === "collection" || app?.id === "intello") {
        // Redirect handled by useEffect, show nothing while navigating
        return null
      }
      // Fallback for other apps
      return (
        <div className="flex items-center justify-center h-64">
          <p className="text-muted-foreground">App &quot;{app?.name}&quot; is coming soon...</p>
        </div>
      )
    }

    return null
  }

  return (
    <AuthGuard>
      <div className="relative min-h-screen overflow-hidden selection:bg-primary/30">
        <GridBackground />
        <div className="relative z-10">
          <TopNavigation />
          <main className="container mx-auto px-6 py-6">{renderContent()}</main>
        </div>
      </div>
    </AuthGuard>
  )
}
