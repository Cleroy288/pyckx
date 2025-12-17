import { memo } from "react"

// Grid pattern style - defined outside component to avoid recreation
const gridPatternStyle = {
  backgroundImage: `
    linear-gradient(to right, var(--grid-line) 1px, transparent 1px),
    linear-gradient(to bottom, var(--grid-line) 1px, transparent 1px)
  `,
  backgroundSize: '20px 20px'
} as const

// Memoized GridBackground - prevents re-renders when parent updates
export const GridBackground = memo(function GridBackground() {
  return (
    <div className="fixed inset-0 z-0 overflow-hidden pointer-events-none">
      {/* Base background */}
      <div className="absolute inset-0 bg-background" />

      {/* CSS Grid pattern - simple horizontal and vertical lines */}
      <div className="absolute inset-0" style={gridPatternStyle} />

      {/* Subtle radial gradient overlay */}
      <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-primary/[0.08] via-transparent to-transparent" />
      
      {/* Vignette effect */}
      <div className="absolute inset-0 bg-[radial-gradient(circle_at_center,_transparent_0%,_var(--background)_100%)] opacity-40" />
    </div>
  )
})
