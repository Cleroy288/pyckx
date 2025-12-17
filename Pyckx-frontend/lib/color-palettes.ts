// == Color Palettes Configuration ==
// Each palette defines colors that override CSS variables in real-time
// Colors use oklch format for better perceptual uniformity

export interface ColorPalette {
  id: string
  name: string
  description: string
  // Core colors
  primary: string        // Main brand/action color
  secondary: string      // Supporting color
  accent: string         // Highlight/emphasis color
  muted: string          // Subtle backgrounds
  background: string     // Page background
  // Semantic colors
  success: string        // Success states
  error: string          // Error/destructive states
  // Preview color (for the selector UI)
  preview: string        // Hex color for preview swatch
}

// Light mode color mappings
export interface ThemeColors {
  light: {
    primary: string
    primaryForeground: string
    secondary: string
    secondaryForeground: string
    accent: string
    accentForeground: string
    muted: string
    mutedForeground: string
    background: string
    foreground: string
    card: string
    cardForeground: string
    border: string
    input: string
    ring: string
    destructive: string
    destructiveForeground: string
    glow: string
    grid: string  // Background grid color
  }
  dark: {
    primary: string
    primaryForeground: string
    secondary: string
    secondaryForeground: string
    accent: string
    accentForeground: string
    muted: string
    mutedForeground: string
    background: string
    foreground: string
    card: string
    cardForeground: string
    border: string
    input: string
    ring: string
    destructive: string
    destructiveForeground: string
    glow: string
    grid: string  // Background grid color
  }
}

export interface FullColorPalette extends ColorPalette {
  colors: ThemeColors
}

// == Pre-configured Color Palettes ==

export const COLOR_PALETTES: FullColorPalette[] = [
  {
    id: "teal-tech",
    name: "Teal Tech",
    description: "Modern teal with clean tech aesthetics",
    primary: "oklch(0.55 0.18 190)",
    secondary: "oklch(0.94 0.01 260)",
    accent: "oklch(0.55 0.18 190)",
    muted: "oklch(0.92 0.008 260)",
    background: "oklch(0.98 0.005 260)",
    success: "oklch(0.55 0.18 145)",
    error: "oklch(0.55 0.22 27)",
    preview: "#14b8a6",
    colors: {
      light: {
        primary: "oklch(0.55 0.18 190)",
        primaryForeground: "oklch(0.98 0 0)",
        secondary: "oklch(0.94 0.01 260)",
        secondaryForeground: "oklch(0.25 0.01 260)",
        accent: "oklch(0.55 0.18 190)",
        accentForeground: "oklch(0.98 0 0)",
        muted: "oklch(0.92 0.008 260)",
        mutedForeground: "oklch(0.45 0.01 260)",
        background: "oklch(0.98 0.005 260)",
        foreground: "oklch(0.15 0.01 260)",
        card: "oklch(1 0 0)",
        cardForeground: "oklch(0.15 0.01 260)",
        border: "oklch(0.88 0.01 260)",
        input: "oklch(0.94 0.008 260)",
        ring: "oklch(0.55 0.18 190)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.55 0.18 190 / 0.25)",
        grid: "oklch(0.55 0.18 190)",
      },
      dark: {
        primary: "oklch(0.78 0.15 190)",
        primaryForeground: "oklch(0.11 0.008 260)",
        secondary: "oklch(0.18 0.008 260)",
        secondaryForeground: "oklch(0.85 0 0)",
        accent: "oklch(0.78 0.15 190)",
        accentForeground: "oklch(0.11 0.008 260)",
        muted: "oklch(0.22 0.008 260)",
        mutedForeground: "oklch(0.55 0 0)",
        background: "oklch(0.11 0.008 260)",
        foreground: "oklch(0.95 0 0)",
        card: "oklch(0.14 0.008 260)",
        cardForeground: "oklch(0.95 0 0)",
        border: "oklch(0.22 0.01 260)",
        input: "oklch(0.18 0.008 260)",
        ring: "oklch(0.78 0.15 190)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.78 0.15 190 / 0.4)",
        grid: "oklch(0.78 0.15 190)",
      },
    },
  },
  {
    id: "purple-dream",
    name: "Purple Dream",
    description: "Elegant purple with soft violet tones",
    primary: "oklch(0.55 0.2 280)",
    secondary: "oklch(0.94 0.02 280)",
    accent: "oklch(0.6 0.18 300)",
    muted: "oklch(0.92 0.015 280)",
    background: "oklch(0.98 0.008 280)",
    success: "oklch(0.55 0.18 145)",
    error: "oklch(0.55 0.22 27)",
    preview: "#8b5cf6",
    colors: {
      light: {
        primary: "oklch(0.55 0.2 280)",
        primaryForeground: "oklch(0.98 0 0)",
        secondary: "oklch(0.94 0.02 280)",
        secondaryForeground: "oklch(0.25 0.02 280)",
        accent: "oklch(0.6 0.18 300)",
        accentForeground: "oklch(0.98 0 0)",
        muted: "oklch(0.92 0.015 280)",
        mutedForeground: "oklch(0.45 0.02 280)",
        background: "oklch(0.98 0.008 280)",
        foreground: "oklch(0.15 0.02 280)",
        card: "oklch(1 0 0)",
        cardForeground: "oklch(0.15 0.02 280)",
        border: "oklch(0.88 0.02 280)",
        input: "oklch(0.94 0.015 280)",
        ring: "oklch(0.55 0.2 280)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.55 0.2 280 / 0.25)",
        grid: "oklch(0.55 0.2 280)",
      },
      dark: {
        primary: "oklch(0.72 0.18 280)",
        primaryForeground: "oklch(0.11 0.015 280)",
        secondary: "oklch(0.18 0.015 280)",
        secondaryForeground: "oklch(0.85 0 0)",
        accent: "oklch(0.7 0.16 300)",
        accentForeground: "oklch(0.11 0.015 280)",
        muted: "oklch(0.22 0.015 280)",
        mutedForeground: "oklch(0.55 0 0)",
        background: "oklch(0.11 0.015 280)",
        foreground: "oklch(0.95 0 0)",
        card: "oklch(0.14 0.015 280)",
        cardForeground: "oklch(0.95 0 0)",
        border: "oklch(0.22 0.02 280)",
        input: "oklch(0.18 0.015 280)",
        ring: "oklch(0.72 0.18 280)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.72 0.18 280 / 0.4)",
        grid: "oklch(0.72 0.18 280)",
      },
    },
  },
  {
    id: "ocean-blue",
    name: "Ocean Blue",
    description: "Deep ocean blue with crisp accents",
    primary: "oklch(0.55 0.2 240)",
    secondary: "oklch(0.94 0.015 240)",
    accent: "oklch(0.6 0.18 220)",
    muted: "oklch(0.92 0.01 240)",
    background: "oklch(0.98 0.006 240)",
    success: "oklch(0.55 0.18 145)",
    error: "oklch(0.55 0.22 27)",
    preview: "#3b82f6",
    colors: {
      light: {
        primary: "oklch(0.55 0.2 240)",
        primaryForeground: "oklch(0.98 0 0)",
        secondary: "oklch(0.94 0.015 240)",
        secondaryForeground: "oklch(0.25 0.015 240)",
        accent: "oklch(0.6 0.18 220)",
        accentForeground: "oklch(0.98 0 0)",
        muted: "oklch(0.92 0.01 240)",
        mutedForeground: "oklch(0.45 0.015 240)",
        background: "oklch(0.98 0.006 240)",
        foreground: "oklch(0.15 0.015 240)",
        card: "oklch(1 0 0)",
        cardForeground: "oklch(0.15 0.015 240)",
        border: "oklch(0.88 0.015 240)",
        input: "oklch(0.94 0.01 240)",
        ring: "oklch(0.55 0.2 240)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.55 0.2 240 / 0.25)",
        grid: "oklch(0.55 0.2 240)",
      },
      dark: {
        primary: "oklch(0.7 0.18 240)",
        primaryForeground: "oklch(0.11 0.012 240)",
        secondary: "oklch(0.18 0.012 240)",
        secondaryForeground: "oklch(0.85 0 0)",
        accent: "oklch(0.68 0.16 220)",
        accentForeground: "oklch(0.11 0.012 240)",
        muted: "oklch(0.22 0.012 240)",
        mutedForeground: "oklch(0.55 0 0)",
        background: "oklch(0.11 0.012 240)",
        foreground: "oklch(0.95 0 0)",
        card: "oklch(0.14 0.012 240)",
        cardForeground: "oklch(0.95 0 0)",
        border: "oklch(0.22 0.015 240)",
        input: "oklch(0.18 0.012 240)",
        ring: "oklch(0.7 0.18 240)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.7 0.18 240 / 0.4)",
        grid: "oklch(0.7 0.18 240)",
      },
    },
  },
  {
    id: "emerald-forest",
    name: "Emerald Forest",
    description: "Rich emerald green with natural vibes",
    primary: "oklch(0.55 0.18 155)",
    secondary: "oklch(0.94 0.015 155)",
    accent: "oklch(0.58 0.16 140)",
    muted: "oklch(0.92 0.01 155)",
    background: "oklch(0.98 0.006 155)",
    success: "oklch(0.55 0.18 145)",
    error: "oklch(0.55 0.22 27)",
    preview: "#10b981",
    colors: {
      light: {
        primary: "oklch(0.55 0.18 155)",
        primaryForeground: "oklch(0.98 0 0)",
        secondary: "oklch(0.94 0.015 155)",
        secondaryForeground: "oklch(0.25 0.015 155)",
        accent: "oklch(0.58 0.16 140)",
        accentForeground: "oklch(0.98 0 0)",
        muted: "oklch(0.92 0.01 155)",
        mutedForeground: "oklch(0.45 0.015 155)",
        background: "oklch(0.98 0.006 155)",
        foreground: "oklch(0.15 0.015 155)",
        card: "oklch(1 0 0)",
        cardForeground: "oklch(0.15 0.015 155)",
        border: "oklch(0.88 0.015 155)",
        input: "oklch(0.94 0.01 155)",
        ring: "oklch(0.55 0.18 155)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.55 0.18 155 / 0.25)",
        grid: "oklch(0.55 0.18 155)",
      },
      dark: {
        primary: "oklch(0.72 0.16 155)",
        primaryForeground: "oklch(0.11 0.012 155)",
        secondary: "oklch(0.18 0.012 155)",
        secondaryForeground: "oklch(0.85 0 0)",
        accent: "oklch(0.68 0.14 140)",
        accentForeground: "oklch(0.11 0.012 155)",
        muted: "oklch(0.22 0.012 155)",
        mutedForeground: "oklch(0.55 0 0)",
        background: "oklch(0.11 0.012 155)",
        foreground: "oklch(0.95 0 0)",
        card: "oklch(0.14 0.012 155)",
        cardForeground: "oklch(0.95 0 0)",
        border: "oklch(0.22 0.015 155)",
        input: "oklch(0.18 0.012 155)",
        ring: "oklch(0.72 0.16 155)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.72 0.16 155 / 0.4)",
        grid: "oklch(0.72 0.16 155)",
      },
    },
  },
  {
    id: "sunset-orange",
    name: "Sunset Orange",
    description: "Warm sunset orange with golden highlights",
    primary: "oklch(0.65 0.2 45)",
    secondary: "oklch(0.94 0.02 45)",
    accent: "oklch(0.6 0.18 30)",
    muted: "oklch(0.92 0.015 45)",
    background: "oklch(0.98 0.008 45)",
    success: "oklch(0.55 0.18 145)",
    error: "oklch(0.55 0.22 27)",
    preview: "#f97316",
    colors: {
      light: {
        primary: "oklch(0.65 0.2 45)",
        primaryForeground: "oklch(0.98 0 0)",
        secondary: "oklch(0.94 0.02 45)",
        secondaryForeground: "oklch(0.25 0.02 45)",
        accent: "oklch(0.6 0.18 30)",
        accentForeground: "oklch(0.98 0 0)",
        muted: "oklch(0.92 0.015 45)",
        mutedForeground: "oklch(0.45 0.02 45)",
        background: "oklch(0.98 0.008 45)",
        foreground: "oklch(0.15 0.02 45)",
        card: "oklch(1 0 0)",
        cardForeground: "oklch(0.15 0.02 45)",
        border: "oklch(0.88 0.02 45)",
        input: "oklch(0.94 0.015 45)",
        ring: "oklch(0.65 0.2 45)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.65 0.2 45 / 0.25)",
        grid: "oklch(0.65 0.2 45)",
      },
      dark: {
        primary: "oklch(0.75 0.18 45)",
        primaryForeground: "oklch(0.11 0.015 45)",
        secondary: "oklch(0.18 0.015 45)",
        secondaryForeground: "oklch(0.85 0 0)",
        accent: "oklch(0.7 0.16 30)",
        accentForeground: "oklch(0.11 0.015 45)",
        muted: "oklch(0.22 0.015 45)",
        mutedForeground: "oklch(0.55 0 0)",
        background: "oklch(0.11 0.015 45)",
        foreground: "oklch(0.95 0 0)",
        card: "oklch(0.14 0.015 45)",
        cardForeground: "oklch(0.95 0 0)",
        border: "oklch(0.22 0.02 45)",
        input: "oklch(0.18 0.015 45)",
        ring: "oklch(0.75 0.18 45)",
        destructive: "oklch(0.55 0.22 27)",
        destructiveForeground: "oklch(0.98 0 0)",
        glow: "oklch(0.75 0.18 45 / 0.4)",
        grid: "oklch(0.75 0.18 45)",
      },
    },
  },
]

// Default palette ID
export const DEFAULT_PALETTE_ID = "teal-tech"

// Get palette by ID
export function getPaletteById(id: string): FullColorPalette | undefined {
  return COLOR_PALETTES.find(p => p.id === id)
}

// Get default palette
export function getDefaultPalette(): FullColorPalette {
  return COLOR_PALETTES.find(p => p.id === DEFAULT_PALETTE_ID) ?? COLOR_PALETTES[0]
}
