"use client"

import { memo, useCallback } from "react"
import { AppCard } from "./app-card"
import type { App } from "@/lib/apps-data"
import { Grid3X3, LayoutGrid, Loader2 } from "lucide-react"
import { useUserApps } from "@/lib/user-apps-context"

interface AppGridProps {
  otherApps: App[]
  onAppClick: (app: App) => void
}

export const AppGrid = memo(function AppGrid({ otherApps, onAppClick }: AppGridProps) {
  const { getUserAppsData, loading, navigateTo } = useUserApps()
  const userApps = getUserAppsData()

  const handleUserAppClick = useCallback((app: App) => {
    navigateTo({ type: "app", appId: app.id })
  }, [navigateTo])

  return (
    <div className="space-y-8">
      {/* My Apps Section */}
      {(userApps.length > 0 || loading) && (
        <section className="space-y-6">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 border border-primary/20">
              <LayoutGrid className="h-5 w-5 text-primary" />
            </div>
            <div>
              <h2 className="text-xl font-semibold tracking-tight text-foreground">My Apps</h2>
              <p className="text-sm text-muted-foreground">Your added applications</p>
            </div>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-6 w-6 animate-spin text-primary" />
            </div>
          ) : (
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
              {userApps.map((app) => (
                <AppCard key={app.id} app={app} onClick={() => handleUserAppClick(app)} variant="default" />
              ))}
            </div>
          )}
        </section>
      )}

      {/* Explore Apps Section */}
      <section className="space-y-6">
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 border border-primary/20">
            <Grid3X3 className="h-5 w-5 text-primary" />
          </div>
          <div>
            <h2 className="text-xl font-semibold tracking-tight text-foreground">Explore Apps</h2>
            <p className="text-sm text-muted-foreground">Discover tools for your workflow</p>
          </div>
        </div>

        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {otherApps.map((app) => (
            <AppCard key={app.id} app={app} onClick={() => onAppClick(app)} variant="featured" />
          ))}
        </div>
      </section>
    </div>
  )
})
