# Theming System Documentation

## Overview

Pyckx uses a dual-layer theming system:
1. **Theme**: Light/Dark mode toggle
2. **Palette**: Color scheme selection + custom colors

## Theme (Light/Dark)

### Storage
- Key: `pyckx-theme`
- Values: `"light"` | `"dark"`

### Provider
```typescript
// lib/theme-context.tsx
import { useTheme } from "@/lib/theme-context"

const { theme, setTheme, toggleTheme } = useTheme()
// theme: "light" | "dark"
```

### Implementation
- Adds/removes `dark` class on `<html>` element
- Tailwind's `dark:` variants respond automatically

## Color Palettes

### Pre-configured Palettes

| ID | Name | Preview Color |
|----|------|---------------|
| `teal-tech` | Teal Tech | #14b8a6 |
| `purple-dream` | Purple Dream | #a855f7 |
| `ocean-blue` | Ocean Blue | #3b82f6 |
| `emerald-forest` | Emerald Forest | #10b981 |
| `sunset-orange` | Sunset Orange | #f97316 |

### Storage
- Key: `pyckx-palette`
- Value: Palette ID string

### Provider
```typescript
// lib/palette-context.tsx
import { usePalette } from "@/lib/palette-context"

const { 
  currentPalette,      // Current palette object
  setPalette,          // Set by ID
  previewPalette,      // Hover preview
  clearPreview,        // Clear preview
  palettes,            // All available palettes
} = usePalette()
```

## Custom Colors

### Customizable Colors

| Key | Description | Default Light | Default Dark |
|-----|-------------|---------------|--------------|
| `primary` | Main brand color | #14b8a6 | #5eead4 |
| `secondary` | Supporting color | #f1f5f9 | #1e293b |
| `accent` | Highlights | #14b8a6 | #5eead4 |
| `muted` | Subtle backgrounds | #f1f5f9 | #334155 |
| `background` | Page background | #fafafa | #0a0a0a |
| `success` | Success states | #16a34a | #22c55e |
| `warning` | Warning states | #ca8a04 | #eab308 |
| `destructive` | Error states | #ef4444 | #dc2626 |

### Storage
- Key: `pyckx-custom-colors`
- Format: `{ light: { [key]: color }, dark: { [key]: color } }`

### Provider API
```typescript
const {
  customColors,              // { light: {...}, dark: {...} }
  setCustomColor,            // (mode, key, color | null)
  resetAllCustomColors,      // Clear all custom
  hasCustomColors,           // Any custom set?
  hasCustomColorsForMode,    // (mode) => boolean
  importColorsFromOtherMode, // Copy from other mode (excl. background)
} = usePalette()

// Set custom color
setCustomColor("dark", "primary", "#ff0000")

// Reset single color to default
setCustomColor("dark", "primary", null)

// Import from other mode
importColorsFromOtherMode() // Copies all except background
```

## CSS Variables

All colors are applied as CSS custom properties on `:root`:

```css
/* Core */
--primary
--primary-foreground
--secondary
--secondary-foreground
--accent
--accent-foreground
--muted
--muted-foreground
--background
--foreground

/* Semantic */
--destructive
--destructive-foreground
--success
--success-foreground
--warning
--warning-foreground

/* UI */
--card
--card-foreground
--popover
--popover-foreground
--border
--input
--ring
--glow

/* Sidebar */
--sidebar
--sidebar-foreground
--sidebar-primary
--sidebar-primary-foreground
--sidebar-accent
--sidebar-accent-foreground
--sidebar-border
--sidebar-ring

/* Charts */
--chart-1 through --chart-5
```

## Usage in Components

### Tailwind Classes
```tsx
<div className="bg-primary text-primary-foreground" />
<div className="bg-accent/10 border-accent/30" />
<div className="text-destructive bg-destructive/10" />
```

### Dynamic Styles
```tsx
<div style={{ backgroundColor: 'var(--primary)' }} />
```

## Palette Selector Component

Located at `components/settings/palette-selector.tsx`:
- Dropdown with palette list (hover to preview)
- Color pickers for each customizable color
- Mode indicator (shows current light/dark)
- Import button (when other mode has custom colors)
- Reset all button

## Provider Setup

In `app/layout.tsx`:
```tsx
<ThemeProvider>
  <PaletteProvider>
    {children}
  </PaletteProvider>
</ThemeProvider>
```

Order matters: PaletteProvider depends on ThemeProvider.
