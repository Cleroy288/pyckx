"use client"

import { memo, useCallback } from "react"
import { ChevronLeft, ChevronRight, User, Hexagon, LogOut } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useUserApps } from "@/lib/user-apps-context"
import { useAuth } from "@/lib/auth-context"
import { ThemeToggle } from "@/components/theme-toggle"
import { PaletteSelector } from "@/components/palette-selector"

export const TopNavigation = memo(function TopNavigation() {
  const { getCurrentPageName, navigatePrev, navigateNext, canNavigatePrev, canNavigateNext, navigateTo } = useUserApps()
  const { user, logout } = useAuth()

  const handleHome = useCallback(() => {
    navigateTo({ type: "dashboard" })
  }, [navigateTo])

  const handleLogout = useCallback(async () => {
    try {
      await logout()
    } catch (error) {
      console.error("Logout failed:", error)
    }
  }, [logout])

  return (
    <header className="sticky top-0 z-50 py-4">
      <div className="container mx-auto flex items-center justify-between px-6">
        <button onClick={handleHome} className="group flex items-center gap-2 transition-opacity hover:opacity-80 cursor-pointer">
          <div className="relative flex h-8 w-8 items-center justify-center">
            <Hexagon className="h-8 w-8 text-primary" strokeWidth={1.5} />
            <span className="absolute text-[10px] font-bold text-primary">P</span>
          </div>
          <span className="text-lg font-semibold tracking-tight text-foreground">PYCKX</span>
        </button>

        <div className="flex items-center">
          <div className="flex items-center overflow-hidden rounded-lg border border-border/60 bg-card/40 backdrop-blur-md">
            <Button
              variant="ghost"
              size="icon"
              onClick={navigatePrev}
              disabled={!canNavigatePrev}
              className="h-9 w-9 rounded-none border-r border-border/40 text-muted-foreground transition-colors hover:bg-primary/10 hover:text-primary disabled:opacity-30"
            >
              <ChevronLeft className="h-4 w-4" />
            </Button>
            <div className="flex min-w-[120px] items-center justify-center gap-2 px-4">
              <div className="h-1.5 w-1.5 rounded-full bg-primary animate-pulse" />
              <span className="text-sm font-medium text-foreground">{getCurrentPageName()}</span>
            </div>
            <Button
              variant="ghost"
              size="icon"
              onClick={navigateNext}
              disabled={!canNavigateNext}
              className="h-9 w-9 rounded-none border-l border-border/40 text-muted-foreground transition-colors hover:bg-primary/10 hover:text-primary disabled:opacity-30"
            >
              <ChevronRight className="h-4 w-4" />
            </Button>
          </div>
        </div>

        {/* Theme Toggle, Palette Selector & User Menu */}
        <div className="flex items-center gap-2">
          <ThemeToggle />
          <PaletteSelector />
          
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                className="h-9 gap-2 rounded-lg border border-border/60 bg-card/40 px-3 backdrop-blur-md transition-colors hover:bg-primary/10 hover:text-primary"
              >
                <div className="flex h-5 w-5 items-center justify-center rounded-full bg-primary/20">
                  <User className="h-3 w-3 text-primary" />
                </div>
                <span className="text-sm font-medium">{user?.username ?? "User"}</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-48">
              <div className="px-2 py-1.5">
                <p className="text-sm font-medium">{user?.username}</p>
                <p className="text-xs text-muted-foreground">{user?.email}</p>
              </div>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={handleLogout} className="text-destructive focus:text-destructive cursor-pointer">
                <LogOut className="mr-2 h-4 w-4" />
                Sign out
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
    </header>
  )
})
