"use client"

import { memo } from "react"
import type React from "react"
import { Badge } from "@/components/ui/badge"
import type { App, AppColorKey } from "@/lib/apps-data"
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
  Zap,
  ArrowUpRight,
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
const colorClasses: Record<AppColorKey, { bg: string; text: string; border: string }> = {
  primary: { bg: "bg-primary/10", text: "text-primary", border: "border-primary/30" },
  secondary: { bg: "bg-secondary/50", text: "text-secondary-foreground", border: "border-secondary/30" },
  accent: { bg: "bg-accent/10", text: "text-accent", border: "border-accent/30" },
  muted: { bg: "bg-muted/50", text: "text-muted-foreground", border: "border-muted/30" },
  success: { bg: "bg-success/10", text: "text-success", border: "border-success/30" },
  warning: { bg: "bg-warning/10", text: "text-warning", border: "border-warning/30" },
  destructive: { bg: "bg-destructive/10", text: "text-destructive", border: "border-destructive/30" },
}

interface AppCardProps {
  app: App
  onClick: () => void
  variant?: "featured" | "default"
}

// Memoized to prevent unnecessary re-renders when parent context changes
export const AppCard = memo(function AppCard({ app, onClick, variant = "default" }: AppCardProps) {
  const Icon = iconMap[app.icon] || Zap
  const isFeatured = variant === "featured"
  const colors = colorClasses[app.colorKey] || colorClasses.primary

  return (
    <button
      onClick={onClick}
      className={`tech-glow group relative w-full overflow-hidden rounded-xl border border-accent/30 bg-accent/10 p-6 text-left backdrop-blur-sm transition-all duration-300 hover:border-accent/50 hover:bg-accent/20 cursor-pointer ${
        isFeatured ? "min-h-[200px]" : "min-h-[160px]"
      }`}
    >
      <div className="absolute inset-0 bg-gradient-to-br from-primary/[0.08] via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />

      <div className="absolute right-0 top-0 h-16 w-16 translate-x-8 -translate-y-8 rounded-full bg-primary/10 blur-2xl transition-transform duration-500 group-hover:translate-x-4 group-hover:-translate-y-4" />

      <div className="relative flex h-full flex-col">
        {/* Header with icon */}
        <div className="flex items-start justify-between">
          <div
            className={`flex h-12 w-12 items-center justify-center rounded-lg border border-border/50 transition-all duration-300 group-hover:${colors.border} group-hover:shadow-[0_0_20px_-5px_var(--glow)] ${colors.bg}`}
          >
            <Icon className={`h-6 w-6 transition-transform duration-300 group-hover:scale-110 ${colors.text}`} />
          </div>
          {app.isPro && (
            <Badge className="border-0 bg-warning/15 text-warning text-[10px] font-medium uppercase tracking-wider">
              Pro
            </Badge>
          )}
        </div>

        {/* Content */}
        <div className="mt-4 flex-1">
          <h3 className="flex items-center gap-2 text-base font-semibold text-foreground transition-colors group-hover:text-primary">
            {app.name}
            <ArrowUpRight className="h-3.5 w-3.5 opacity-0 transition-all duration-300 group-hover:opacity-100 group-hover:translate-x-0.5 group-hover:-translate-y-0.5" />
          </h3>
          <p className="mt-1.5 line-clamp-2 text-sm leading-relaxed text-muted-foreground">{app.shortDescription}</p>
        </div>
      </div>
    </button>
  )
})
