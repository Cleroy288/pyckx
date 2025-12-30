"use client"

import { useTheme } from "@/lib/theme-context"
import { Button } from "@/components/ui/button"

export function ThemeToggle() {
  const { theme, toggleTheme } = useTheme()

  return (
    <Button
      variant="ghost"
      size="sm"
      onClick={toggleTheme}
      className="h-9 w-9 rounded-lg border border-border/60 bg-card/40 backdrop-blur-md transition-all duration-300 hover:bg-primary/10 hover:border-primary/30 hover:scale-105"
      title={theme === "light" ? "Switch to dark mode" : "Switch to light mode"}
    >
      <span className="text-lg transition-transform duration-300 hover:rotate-12">
        {theme === "light" ? "🌙" : "☀️"}
      </span>
    </Button>
  )
}
