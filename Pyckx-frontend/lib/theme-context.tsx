"use client"

import { createContext, useContext, useState, useEffect, useCallback, useMemo, type ReactNode } from "react"

// == Types // ==
type Theme = "light" | "dark"

interface ThemeContextType {
  theme: Theme
  toggleTheme: () => void
  setTheme: (theme: Theme) => void
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined)

const STORAGE_KEY = "pyckx-theme"

// Helper to get initial theme from DOM (set by inline script)
function getInitialTheme(): Theme {
  if (typeof window === "undefined") return "light"
  
  // Check if dark class is already on html (set by inline script)
  if (document.documentElement.classList.contains("dark")) {
    return "dark"
  }
  
  // Fallback to localStorage
  const stored = localStorage.getItem(STORAGE_KEY)
  if (stored === "dark") return "dark"
  
  return "light"
}

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setThemeState] = useState<Theme>("light")
  const [mounted, setMounted] = useState(false)

  // Initialize theme from DOM/localStorage on mount
  useEffect(() => {
    setThemeState(getInitialTheme())
    setMounted(true)
  }, [])

  // Apply theme to document whenever it changes
  const applyTheme = useCallback((newTheme: Theme) => {
    const root = document.documentElement
    
    if (newTheme === "dark") {
      root.classList.add("dark")
    } else {
      root.classList.remove("dark")
    }
    
    // Save to localStorage
    localStorage.setItem(STORAGE_KEY, newTheme)
  }, [])

  // Apply theme when it changes (after mount)
  useEffect(() => {
    if (!mounted) return
    applyTheme(theme)
  }, [theme, mounted, applyTheme])

  const toggleTheme = useCallback(() => {
    setThemeState(prev => {
      const newTheme = prev === "light" ? "dark" : "light"
      return newTheme
    })
  }, [])

  const setTheme = useCallback((newTheme: Theme) => {
    setThemeState(newTheme)
  }, [])

  // Memoize context value to prevent unnecessary re-renders
  const contextValue = useMemo(() => ({
    theme: mounted ? theme : "light",
    toggleTheme,
    setTheme
  }), [mounted, theme, toggleTheme, setTheme]);

  // Show children even before mount - the inline script handles initial theme
  return (
    <ThemeContext.Provider value={contextValue}>
      {children}
    </ThemeContext.Provider>
  )
}

export function useTheme() {
  const context = useContext(ThemeContext)
  if (context === undefined) {
    throw new Error("useTheme must be used within a ThemeProvider")
  }
  return context
}
