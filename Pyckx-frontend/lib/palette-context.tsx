"use client"

import { createContext, useContext, useState, useEffect, useCallback, useMemo, type ReactNode } from "react"
import { COLOR_PALETTES, getPaletteById, getDefaultPalette, type FullColorPalette } from "./color-palettes"
import { useTheme } from "./theme-context"

// == Types ==
export type CustomColorKey = "primary" | "secondary" | "accent" | "muted" | "background" | "success" | "warning" | "destructive" | "grid"

interface CustomColors {
  light: Partial<Record<CustomColorKey, string>>
  dark: Partial<Record<CustomColorKey, string>>
}

interface PaletteContextType {
  currentPalette: FullColorPalette
  setPalette: (paletteId: string) => void
  previewPalette: (paletteId: string) => void
  clearPreview: () => void
  palettes: FullColorPalette[]
  customColors: CustomColors
  setCustomColor: (mode: "light" | "dark", colorKey: CustomColorKey, color: string | null) => void
  resetAllCustomColors: () => void
  hasCustomColors: boolean
  hasCustomColorsForMode: (mode: "light" | "dark") => boolean
  importColorsFromOtherMode: () => void
  // Toggle between palette and custom colors
  useCustomColors: boolean
  setUseCustomColors: (use: boolean) => void
}

const PaletteContext = createContext<PaletteContextType | undefined>(undefined)

const STORAGE_KEY = "pyckx-palette"
const CUSTOM_COLORS_KEY = "pyckx-custom-colors"
const USE_CUSTOM_KEY = "pyckx-use-custom-colors"

// Default empty custom colors
const DEFAULT_CUSTOM_COLORS: CustomColors = { light: {}, dark: {} }

// Apply palette colors to CSS variables
function applyPaletteColors(palette: FullColorPalette, isDark: boolean, customColors?: CustomColors) {
  const root = document.documentElement
  const colors = isDark ? palette.colors.dark : palette.colors.light
  const custom = isDark ? customColors?.dark : customColors?.light

  // Get color with custom override
  const getColor = (key: CustomColorKey, defaultColor: string) => custom?.[key] || defaultColor

  // Apply all color variables with custom overrides
  const primary = getColor("primary", colors.primary)
  const secondary = getColor("secondary", colors.secondary)
  const accent = getColor("accent", colors.accent)
  const muted = getColor("muted", colors.muted)
  const background = getColor("background", colors.background)
  const destructive = getColor("destructive", colors.destructive)
  // Success and warning with defaults
  const success = custom?.success || (isDark ? "#22c55e" : "#16a34a")
  const warning = custom?.warning || (isDark ? "#eab308" : "#ca8a04")

  root.style.setProperty("--primary", primary)
  root.style.setProperty("--primary-foreground", colors.primaryForeground)
  root.style.setProperty("--secondary", secondary)
  root.style.setProperty("--secondary-foreground", colors.secondaryForeground)
  root.style.setProperty("--accent", accent)
  root.style.setProperty("--accent-foreground", colors.accentForeground)
  root.style.setProperty("--muted", muted)
  root.style.setProperty("--muted-foreground", colors.mutedForeground)
  root.style.setProperty("--foreground", colors.foreground)
  root.style.setProperty("--card", colors.card)
  root.style.setProperty("--card-foreground", colors.cardForeground)
  root.style.setProperty("--popover", colors.card)
  root.style.setProperty("--popover-foreground", colors.cardForeground)
  root.style.setProperty("--border", colors.border)
  root.style.setProperty("--input", colors.input)
  root.style.setProperty("--ring", primary) // Ring follows primary
  root.style.setProperty("--destructive", destructive)
  root.style.setProperty("--destructive-foreground", colors.destructiveForeground)
  root.style.setProperty("--success", success)
  root.style.setProperty("--success-foreground", "#ffffff")
  root.style.setProperty("--warning", warning)
  root.style.setProperty("--warning-foreground", "#000000")
  root.style.setProperty("--glow", `${primary.replace(")", " / 0.25)")}`) // Glow follows primary
  root.style.setProperty("--background", background)
  
  // Sidebar colors (mirror main colors)
  root.style.setProperty("--sidebar", background)
  root.style.setProperty("--sidebar-foreground", colors.foreground)
  root.style.setProperty("--sidebar-primary", primary)
  root.style.setProperty("--sidebar-primary-foreground", colors.primaryForeground)
  root.style.setProperty("--sidebar-accent", muted)
  root.style.setProperty("--sidebar-accent-foreground", colors.foreground)
  root.style.setProperty("--sidebar-border", colors.border)
  root.style.setProperty("--sidebar-ring", primary)
  
  // Chart colors
  root.style.setProperty("--chart-1", primary)
  root.style.setProperty("--chart-2", accent)
  root.style.setProperty("--chart-3", secondary)
  root.style.setProperty("--chart-4", muted)
  root.style.setProperty("--chart-5", primary)
  
  // Grid background color with pre-computed opacity variants
  const grid = getColor("grid", colors.grid)
  root.style.setProperty("--grid", grid)
  // Create transparent versions for grid lines and dots
  const gridLineOpacity = isDark ? 0.12 : 0.08
  const gridDotOpacity = isDark ? 0.2 : 0.15
  root.style.setProperty("--grid-line", `color-mix(in srgb, ${grid} ${gridLineOpacity * 100}%, transparent)`)
  root.style.setProperty("--grid-dot", `color-mix(in srgb, ${grid} ${gridDotOpacity * 100}%, transparent)`)
}

export function PaletteProvider({ children }: { children: ReactNode }) {
  const [currentPalette, setCurrentPalette] = useState<FullColorPalette>(getDefaultPalette())
  const [customColors, setCustomColors] = useState<CustomColors>(DEFAULT_CUSTOM_COLORS)
  const [useCustomColors, setUseCustomColorsState] = useState(false)
  const [mounted, setMounted] = useState(false)
  const { theme } = useTheme()

  // Check if any custom colors are set
  const hasCustomColors = Object.keys(customColors.light).length > 0 || Object.keys(customColors.dark).length > 0

  // Check if a specific mode has custom colors
  const hasCustomColorsForMode = useCallback((mode: "light" | "dark") => {
    return Object.keys(customColors[mode]).length > 0
  }, [customColors])

  // Import colors from the other mode (excluding background)
  const importColorsFromOtherMode = useCallback(() => {
    const currentMode = theme === "dark" ? "dark" : "light"
    const otherMode = currentMode === "dark" ? "light" : "dark"
    const otherModeColors = customColors[otherMode]
    
    if (Object.keys(otherModeColors).length === 0) return
    
    setCustomColors(prev => {
      // Copy all colors except background
      const importedColors: Partial<Record<CustomColorKey, string>> = {}
      for (const [key, value] of Object.entries(otherModeColors)) {
        if (key !== "background" && value) {
          importedColors[key as CustomColorKey] = value
        }
      }
      
      const updated = { 
        ...prev, 
        [currentMode]: { ...prev[currentMode], ...importedColors }
      }
      localStorage.setItem(CUSTOM_COLORS_KEY, JSON.stringify(updated))
      return updated
    })
  }, [theme, customColors])

  // Initialize palette, custom colors, and toggle state from localStorage on mount
  useEffect(() => {
    if (typeof window === "undefined") return
    
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored) {
      const palette = getPaletteById(stored)
      if (palette) {
        setCurrentPalette(palette)
      }
    }
    
    // Load custom colors
    const storedColors = localStorage.getItem(CUSTOM_COLORS_KEY)
    if (storedColors) {
      try {
        const parsed = JSON.parse(storedColors) as CustomColors
        setCustomColors(parsed)
      } catch {
        // Invalid JSON, ignore
      }
    }
    
    // Load toggle state
    const storedUseCustom = localStorage.getItem(USE_CUSTOM_KEY)
    if (storedUseCustom === "true") {
      setUseCustomColorsState(true)
    }
    
    setMounted(true)
  }, [])

  // Apply palette colors when palette, theme, custom colors, or toggle change
  useEffect(() => {
    if (!mounted) return
    const isDark = theme === "dark"
    // Only apply custom colors if toggle is enabled
    applyPaletteColors(currentPalette, isDark, useCustomColors ? customColors : undefined)
  }, [currentPalette, theme, mounted, customColors, useCustomColors])

  const setPalette = useCallback((paletteId: string) => {
    const palette = getPaletteById(paletteId)
    if (palette) {
      setCurrentPalette(palette)
      localStorage.setItem(STORAGE_KEY, paletteId)
    }
  }, [])

  // Preview a palette temporarily (on hover)
  const previewPalette = useCallback((paletteId: string) => {
    const palette = getPaletteById(paletteId)
    if (palette) {
      const isDark = theme === "dark"
      // When previewing palettes, show without custom colors to see the actual palette
      applyPaletteColors(palette, isDark, undefined)
    }
  }, [theme])

  // Clear preview and restore current palette
  const clearPreview = useCallback(() => {
    const isDark = theme === "dark"
    applyPaletteColors(currentPalette, isDark, useCustomColors ? customColors : undefined)
  }, [currentPalette, theme, customColors, useCustomColors])

  // Set custom color for light or dark mode
  const setCustomColor = useCallback((mode: "light" | "dark", colorKey: CustomColorKey, color: string | null) => {
    setCustomColors(prev => {
      const modeColors = { ...prev[mode] }
      if (color === null) {
        delete modeColors[colorKey]
      } else {
        modeColors[colorKey] = color
        // Auto-enable custom colors when user sets a color
        if (!useCustomColors) {
          setUseCustomColorsState(true)
          localStorage.setItem(USE_CUSTOM_KEY, "true")
        }
      }
      const updated = { ...prev, [mode]: modeColors }
      localStorage.setItem(CUSTOM_COLORS_KEY, JSON.stringify(updated))
      return updated
    })
  }, [useCustomColors])

  // Reset all custom colors
  const resetAllCustomColors = useCallback(() => {
    setCustomColors(DEFAULT_CUSTOM_COLORS)
    localStorage.removeItem(CUSTOM_COLORS_KEY)
  }, [])

  // Toggle custom colors on/off
  const setUseCustomColors = useCallback((use: boolean) => {
    setUseCustomColorsState(use)
    localStorage.setItem(USE_CUSTOM_KEY, use ? "true" : "false")
  }, [])

  // Memoize context value to prevent unnecessary re-renders
  const contextValue = useMemo(() => ({ 
    currentPalette, 
    setPalette,
    previewPalette,
    clearPreview,
    palettes: COLOR_PALETTES,
    customColors,
    setCustomColor,
    resetAllCustomColors,
    hasCustomColors,
    hasCustomColorsForMode,
    importColorsFromOtherMode,
    useCustomColors,
    setUseCustomColors
  }), [
    currentPalette,
    setPalette,
    previewPalette,
    clearPreview,
    customColors,
    setCustomColor,
    resetAllCustomColors,
    hasCustomColors,
    hasCustomColorsForMode,
    importColorsFromOtherMode,
    useCustomColors,
    setUseCustomColors
  ]);

  return (
    <PaletteContext.Provider value={contextValue}>
      {children}
    </PaletteContext.Provider>
  )
}

export function usePalette() {
  const context = useContext(PaletteContext)
  if (context === undefined) {
    throw new Error("usePalette must be used within a PaletteProvider")
  }
  return context
}
