"use client"

import { useState } from "react"
import { Palette, Check, Sun, Moon, RotateCcw, Download, ToggleLeft, ToggleRight } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { ScrollArea } from "@/components/ui/scroll-area"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { usePalette, type CustomColorKey } from "@/lib/palette-context"
import { useTheme } from "@/lib/theme-context"

// Color configuration for the UI
const COLOR_CONFIG: { key: CustomColorKey; label: string; defaultLight: string; defaultDark: string; category: "core" | "semantic" | "visual" }[] = [
  // Core colors
  { key: "primary", label: "Primary", defaultLight: "#14b8a6", defaultDark: "#5eead4", category: "core" },
  { key: "secondary", label: "Secondary", defaultLight: "#f1f5f9", defaultDark: "#1e293b", category: "core" },
  { key: "accent", label: "Accent", defaultLight: "#14b8a6", defaultDark: "#5eead4", category: "core" },
  { key: "muted", label: "Muted", defaultLight: "#f1f5f9", defaultDark: "#334155", category: "core" },
  { key: "background", label: "Background", defaultLight: "#fafafa", defaultDark: "#0a0a0a", category: "core" },
  // Visual colors
  { key: "grid", label: "Grid", defaultLight: "#14b8a6", defaultDark: "#5eead4", category: "visual" },
  // Semantic colors
  { key: "success", label: "Success", defaultLight: "#16a34a", defaultDark: "#22c55e", category: "semantic" },
  { key: "warning", label: "Warning", defaultLight: "#ca8a04", defaultDark: "#eab308", category: "semantic" },
  { key: "destructive", label: "Error", defaultLight: "#ef4444", defaultDark: "#dc2626", category: "semantic" },
]

const CORE_COLORS = COLOR_CONFIG.filter(c => c.category === "core")
const VISUAL_COLORS = COLOR_CONFIG.filter(c => c.category === "visual")
const SEMANTIC_COLORS = COLOR_CONFIG.filter(c => c.category === "semantic")

// Reusable color row component
function ColorRow({ 
  config, 
  customValue, 
  defaultValue, 
  onColorChange, 
  onReset 
}: { 
  config: typeof COLOR_CONFIG[0]
  customValue: string | undefined
  defaultValue: string
  onColorChange: (color: string) => void
  onReset: () => void
}) {
  return (
    <div className="flex items-center justify-between gap-2 py-1">
      <span className="text-sm shrink-0">{config.label}</span>
      <div className="flex items-center gap-1">
        <input
          type="color"
          value={customValue || defaultValue}
          onChange={(e) => onColorChange(e.target.value)}
          className="w-7 h-7 rounded border border-border/50 cursor-pointer appearance-none bg-transparent [&::-webkit-color-swatch-wrapper]:p-0.5 [&::-webkit-color-swatch]:rounded [&::-webkit-color-swatch]:border-0"
          title={`Pick ${config.label.toLowerCase()} color`}
        />
        {customValue && (
          <Button
            variant="ghost"
            size="icon"
            className="h-6 w-6 shrink-0"
            onClick={onReset}
            title="Reset"
          >
            <RotateCcw className="h-3 w-3" />
          </Button>
        )}
      </div>
    </div>
  )
}

export function PaletteSelector() {
  const { 
    currentPalette, 
    setPalette, 
    previewPalette, 
    clearPreview, 
    palettes,
    customColors,
    setCustomColor,
    resetAllCustomColors,
    hasCustomColors,
    hasCustomColorsForMode,
    importColorsFromOtherMode,
    useCustomColors,
    setUseCustomColors
  } = usePalette()
  const { theme } = useTheme()
  
  const [isOpen, setIsOpen] = useState(false)

  const handleOpenChange = (open: boolean) => {
    setIsOpen(open)
    if (!open) clearPreview()
  }

  const currentMode = theme === "dark" ? "dark" : "light"
  const otherMode = currentMode === "dark" ? "light" : "dark"
  const otherModeHasColors = hasCustomColorsForMode(otherMode)

  return (
    <Popover open={isOpen} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="icon"
          className="h-9 w-9 rounded-lg border border-border/60 bg-card/40 backdrop-blur-md transition-colors hover:bg-primary/10 hover:text-primary"
          title="Color Theme"
        >
          <Palette className="h-4 w-4" />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="end" className="w-72 p-0 bg-background border border-black/20 dark:border-white/10 shadow-lg">
        <ScrollArea className="max-h-[75vh]">
          {/* Header */}
          <div className="flex items-center justify-between px-3 py-2.5 border-b border-border/50 sticky top-0 bg-background z-10">
            <div className="flex items-center gap-2">
              <Palette className="h-4 w-4 text-primary" />
              <span className="font-semibold text-sm">Theme</span>
            </div>
            <div className="flex items-center gap-1 text-xs text-muted-foreground">
              {theme === "dark" ? <Moon className="h-3 w-3" /> : <Sun className="h-3 w-3" />}
              <span>{theme === "dark" ? "Dark" : "Light"}</span>
            </div>
          </div>
          
          {/* Color Palettes */}
          <div className="p-2">
            <Label className="text-[10px] text-muted-foreground uppercase tracking-wider px-1 mb-1 block">Palettes</Label>
            <div className="grid grid-cols-5 gap-1.5">
              {palettes.map((palette) => (
                <button
                  key={palette.id}
                  onClick={() => setPalette(palette.id)}
                  onMouseEnter={() => previewPalette(palette.id)}
                  onMouseLeave={clearPreview}
                  className="relative group"
                  title={palette.name}
                >
                  <div 
                    className={`h-8 w-full rounded-md border-2 transition-all ${
                      currentPalette.id === palette.id 
                        ? "border-primary ring-2 ring-primary/20" 
                        : "border-border/50 hover:border-primary/50"
                    }`}
                    style={{ backgroundColor: palette.preview }}
                  />
                  {currentPalette.id === palette.id && (
                    <Check className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 h-4 w-4 text-white drop-shadow-md" />
                  )}
                </button>
              ))}
            </div>
          </div>
          
          <div className="border-t border-border/50 mx-2" />
          
          {/* Custom Colors Toggle */}
          {hasCustomColors && (
            <div className="px-3 py-2">
              <button
                onClick={() => setUseCustomColors(!useCustomColors)}
                className="flex items-center justify-between w-full p-2 rounded-lg border border-border/50 hover:bg-accent/10 transition-colors cursor-pointer"
              >
                <span className="text-sm font-medium">Use Custom Colors</span>
                {useCustomColors ? (
                  <ToggleRight className="h-5 w-5 text-primary" />
                ) : (
                  <ToggleLeft className="h-5 w-5 text-muted-foreground" />
                )}
              </button>
            </div>
          )}
          
          {/* Custom Colors Section */}
          <div className={`p-3 space-y-3 ${hasCustomColors && !useCustomColors ? 'opacity-50' : ''}`}>
            {/* Core Colors */}
            <div>
              <Label className="text-[10px] text-muted-foreground uppercase tracking-wider mb-1 block">Core</Label>
              <div className="space-y-0.5">
                {CORE_COLORS.map((config) => (
                  <ColorRow
                    key={config.key}
                    config={config}
                    customValue={customColors[currentMode][config.key]}
                    defaultValue={currentMode === "dark" ? config.defaultDark : config.defaultLight}
                    onColorChange={(color) => setCustomColor(currentMode, config.key, color)}
                    onReset={() => setCustomColor(currentMode, config.key, null)}
                  />
                ))}
              </div>
            </div>
            
            {/* Visual Colors */}
            <div>
              <Label className="text-[10px] text-muted-foreground uppercase tracking-wider mb-1 block">Visual</Label>
              <div className="space-y-0.5">
                {VISUAL_COLORS.map((config) => (
                  <ColorRow
                    key={config.key}
                    config={config}
                    customValue={customColors[currentMode][config.key]}
                    defaultValue={currentMode === "dark" ? config.defaultDark : config.defaultLight}
                    onColorChange={(color) => setCustomColor(currentMode, config.key, color)}
                    onReset={() => setCustomColor(currentMode, config.key, null)}
                  />
                ))}
              </div>
            </div>
            
            {/* Semantic Colors */}
            <div>
              <Label className="text-[10px] text-muted-foreground uppercase tracking-wider mb-1 block">Status</Label>
              <div className="space-y-0.5">
                {SEMANTIC_COLORS.map((config) => (
                  <ColorRow
                    key={config.key}
                    config={config}
                    customValue={customColors[currentMode][config.key]}
                    defaultValue={currentMode === "dark" ? config.defaultDark : config.defaultLight}
                    onColorChange={(color) => setCustomColor(currentMode, config.key, color)}
                    onReset={() => setCustomColor(currentMode, config.key, null)}
                  />
                ))}
              </div>
            </div>
          </div>
          
          {/* Actions */}
          <div className="p-3 pt-0 space-y-2">
            {otherModeHasColors && (
              <Button
                variant="outline"
                size="sm"
                className="w-full h-8 text-xs"
                onClick={importColorsFromOtherMode}
              >
                <Download className="h-3 w-3 mr-1.5" />
                Import from {otherMode} mode
              </Button>
            )}
            
            {hasCustomColors && (
              <Button
                variant="outline"
                size="sm"
                className="w-full h-8 text-xs"
                onClick={resetAllCustomColors}
              >
                <RotateCcw className="h-3 w-3 mr-1.5" />
                Reset all
              </Button>
            )}
          </div>
        </ScrollArea>
      </PopoverContent>
    </Popover>
  )
}
