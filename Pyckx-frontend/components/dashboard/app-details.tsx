"use client"

import type React from "react"
import { useState } from "react"
import { ArrowLeft, ExternalLink, Zap, Check, Plus, Trash2, Loader2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import type { App, AppColorKey } from "@/lib/apps-data"
import { useUserApps } from "@/lib/user-apps-context"
import {
  BarChart3,
  Users,
  ShoppingCart,
  FileText,
  Calendar,
  MessageSquare,
  Kanban,
  Wallet,
  Package,
  Settings,
  Shield,
  Library,
} from "lucide-react"

const iconMap: Record<string, React.ComponentType<{ className?: string }>> = {
  BarChart3,
  Users,
  ShoppingCart,
  FileText,
  Calendar,
  MessageSquare,
  Kanban,
  Wallet,
  Package,
  Settings,
  Shield,
  Zap,
  Library,
}

// Map color keys to Tailwind classes
const colorClasses: Record<AppColorKey, { bg: string; text: string }> = {
  primary: { bg: "bg-primary/10", text: "text-primary" },
  secondary: { bg: "bg-secondary/50", text: "text-secondary-foreground" },
  accent: { bg: "bg-accent/10", text: "text-accent" },
  muted: { bg: "bg-muted/50", text: "text-muted-foreground" },
  success: { bg: "bg-success/10", text: "text-success" },
  warning: { bg: "bg-warning/10", text: "text-warning" },
  destructive: { bg: "bg-destructive/10", text: "text-destructive" },
}

interface AppDetailsProps {
  app: App
  onBack: () => void
}

export function AppDetails({ app, onBack }: AppDetailsProps) {
  const Icon = iconMap[app.icon] || Zap
  const { hasApp, addApp, removeApp, navigateTo } = useUserApps()
  const isAdded = hasApp(app.id)
  const [isLoading, setIsLoading] = useState(false)
  const colors = colorClasses[app.colorKey] || colorClasses.primary

  const handleAddOrRemove = async () => {
    setIsLoading(true)
    try {
      if (isAdded) {
        await removeApp(app.id)
      } else {
        await addApp(app.id)
      }
    } catch (err) {
      console.error("Failed to update app:", err)
    } finally {
      setIsLoading(false)
    }
  }

  const handleLaunch = async () => {
    if (!isAdded) {
      setIsLoading(true)
      try {
        await addApp(app.id)
      } catch (err) {
        console.error("Failed to add app:", err)
        setIsLoading(false)
        return
      }
      setIsLoading(false)
    }
    navigateTo({ type: "app", appId: app.id })
  }

  return (
    <div className="space-y-8">
      <button
        onClick={onBack}
        className="group flex items-center gap-2 text-sm text-muted-foreground transition-colors hover:text-primary cursor-pointer"
      >
        <ArrowLeft className="h-4 w-4 transition-transform group-hover:-translate-x-1" />
        Back to Dashboard
      </button>

      <div className="relative overflow-hidden rounded-2xl border border-border/50 bg-card/40 p-8 backdrop-blur-sm">
        <div className={`absolute -right-20 -top-20 h-64 w-64 rounded-full blur-3xl opacity-20 ${colors.bg.replace('/10', '/30')}`} />
        <div className={`absolute -left-20 -bottom-20 h-48 w-48 rounded-full blur-3xl opacity-10 ${colors.bg.replace('/10', '/30')}`} />

        <div className="relative flex flex-col gap-6 lg:flex-row lg:items-start lg:gap-8">
          <div className={`flex h-24 w-24 shrink-0 items-center justify-center rounded-2xl border border-border/50 shadow-lg ${colors.bg}`}>
            <Icon className={`h-12 w-12 ${colors.text}`} />
          </div>

          <div className="flex-1 space-y-4">
            <div className="flex flex-wrap items-center gap-3">
              <h1 className="text-3xl font-bold tracking-tight text-foreground">{app.name}</h1>
              {app.isPro && (
                <Badge className="border-0 bg-warning/15 text-warning text-xs font-medium uppercase tracking-wider">
                  Pro
                </Badge>
              )}
              {isAdded && (
                <Badge className="border-0 bg-success/15 text-success text-xs font-medium uppercase tracking-wider">
                  Added
                </Badge>
              )}
            </div>
            <p className="text-lg text-muted-foreground max-w-2xl">{app.shortDescription}</p>
          </div>

          <div className="flex shrink-0 flex-col gap-3 lg:min-w-[200px]">
            <Button
              onClick={handleLaunch}
              disabled={isLoading}
              className="h-11 gap-2 bg-primary text-primary-foreground shadow-lg shadow-primary/20 hover:bg-primary/90 hover:shadow-primary/30"
            >
              {isLoading ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : (
                <ExternalLink className="h-4 w-4" />
              )}
              {isAdded ? "Open App" : "Add & Open"}
            </Button>
            <Button
              variant="outline"
              onClick={handleAddOrRemove}
              disabled={isLoading}
              className={`h-11 gap-2 bg-transparent ${
                isAdded
                  ? "hover:bg-destructive/10 hover:text-destructive hover:border-destructive/30"
                  : "hover:bg-primary/10 hover:text-primary hover:border-primary/30"
              }`}
            >
              {isLoading ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : isAdded ? (
                <>
                  <Trash2 className="h-4 w-4" />
                  Remove from Apps
                </>
              ) : (
                <>
                  <Plus className="h-4 w-4" />
                  Add to My Apps
                </>
              )}
            </Button>
          </div>
        </div>
      </div>

      <div className="space-y-6">
        <div className="rounded-xl border border-border/50 bg-card/40 p-6 backdrop-blur-sm">
          <h2 className="text-lg font-semibold text-foreground mb-4">About</h2>
          <p className="text-muted-foreground leading-relaxed">{app.description}</p>
        </div>

        <div className="rounded-xl border border-border/50 bg-card/40 p-6 backdrop-blur-sm">
          <h2 className="text-lg font-semibold text-foreground mb-4">Key Features</h2>
          <div className="grid gap-3 sm:grid-cols-2">
            {app.features.map((feature, index) => (
              <div key={index} className="flex items-start gap-3 rounded-lg bg-secondary/30 p-3">
                <div className="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-primary/20">
                  <Check className="h-3 w-3 text-primary" />
                </div>
                <span className="text-sm text-foreground">{feature}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
