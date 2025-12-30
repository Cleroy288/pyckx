"use client"

import { useEffect, useMemo, useRef, useState, useId } from "react"
import { GitBranch, AlertCircle, ZoomIn, ZoomOut, RotateCcw, Maximize2 } from "lucide-react"

interface MermaidDiagramProps {
	code: string
	diagramType?: string
	title?: string
}

let mermaidInitPromise: Promise<any> | null = null

async function getMermaid() {
	if (!mermaidInitPromise) {
		mermaidInitPromise = import("mermaid").then((m) => {
			const mermaid = m.default
			mermaid.initialize({
				startOnLoad: false,
				theme: "dark",
				securityLevel: "strict",
				fontFamily: "ui-sans-serif, system-ui, sans-serif",
				flowchart: { curve: "basis", useMaxWidth: true },
				mindmap: { useMaxWidth: true },
			})
			return mermaid
		})
	}
	return mermaidInitPromise
}

function normalizeMermaid(raw: string): string {
	let clean = raw
		.replace(/\\n/g, '\n')
		.replace(/^```mermaid\s*/i, "")
		.replace(/^```\s*/gm, "")
		.replace(/\s*```$/gm, "")
		.trim();

	// Fix: Quote unquoted labels containing parentheses
	// Matches patterns like: id[Label (with parens)] and converts to: id["Label (with parens)"]
	// We strictly look for [ ... ( ... ) ... ] where the content inside [] isn't already quoted
	clean = clean.replace(/\[([^"\]]*\([^"\]]*\)[^"\]]*)\]/g, '["$1"]');

	return clean;
}

export function MermaidDiagram({ code, diagramType = "diagram", title }: MermaidDiagramProps) {
	const containerRef = useRef<HTMLDivElement>(null)
	const renderId = useId()
	const [error, setError] = useState<string | null>(null)
	const [isLoading, setIsLoading] = useState(true)
	const [zoom, setZoom] = useState(1)
	const [isFullscreen, setIsFullscreen] = useState(false)

	const cleanCode = useMemo(() => normalizeMermaid(code), [code])

	const handleZoomIn = () => setZoom(z => Math.min(2, z + 0.25))
	const handleZoomOut = () => setZoom(z => Math.max(0.5, z - 0.25))
	const handleZoomReset = () => setZoom(1)

	useEffect(() => {
		let cancelled = false

		async function run() {
			if (!containerRef.current) return
			setIsLoading(true)
			setError(null)

			try {
				const mermaid = await getMermaid()
				containerRef.current.innerHTML = ""

				if (typeof mermaid.parse === "function") {
					try {
						await mermaid.parse(cleanCode)
					} catch (parseErr: any) {
						throw new Error(`Syntax error: ${parseErr?.message || "invalid mermaid code"}`)
					}
				}

				const id = `mermaid-${renderId.replace(/:/g, "-")}-${Date.now()}`
				const { svg } = await mermaid.render(id, cleanCode)

				if (!cancelled && containerRef.current) {
					containerRef.current.innerHTML = svg
				}
			} catch (e: any) {
				console.error("Mermaid rendering error:", e)
				if (!cancelled) {
					setError(e?.message ?? "Mermaid failed to render")
				}
			} finally {
				if (!cancelled) setIsLoading(false)
			}
		}

		run()
		return () => { cancelled = true }
	}, [cleanCode, renderId])

	return (
		<div className={`my-8 rounded-xl overflow-hidden border border-border/50 bg-card/30 shadow-lg ${isFullscreen ? 'fixed inset-4 z-50 bg-background' : ''}`}>
			{/* Header */}
			<div className="flex items-center justify-between px-4 py-3 bg-gradient-to-r from-muted/50 to-muted/30 border-b border-border/30">
				<div className="flex items-center gap-2">
					<GitBranch className="h-4 w-4 text-primary" />
					<div className="flex flex-col">
						<span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
							{diagramType}
						</span>
						{title && (
							<span className="text-sm font-medium text-foreground">{title}</span>
						)}
					</div>
				</div>

				{/* Zoom Controls */}
				<div className="flex items-center gap-1">
					{error && (
						<span className="text-xs text-red-400 flex items-center gap-1 mr-2">
							<AlertCircle className="h-3 w-3" />
							Error
						</span>
					)}
					<button
						onClick={handleZoomOut}
						disabled={zoom <= 0.5}
						className="p-1.5 rounded-md hover:bg-muted/50 transition-colors disabled:opacity-30"
						title="Zoom out"
					>
						<ZoomOut className="h-4 w-4" />
					</button>
					<span className="text-xs text-muted-foreground min-w-[45px] text-center font-medium">
						{Math.round(zoom * 100)}%
					</span>
					<button
						onClick={handleZoomIn}
						disabled={zoom >= 2}
						className="p-1.5 rounded-md hover:bg-muted/50 transition-colors disabled:opacity-30"
						title="Zoom in"
					>
						<ZoomIn className="h-4 w-4" />
					</button>
					<button
						onClick={handleZoomReset}
						className="p-1.5 rounded-md hover:bg-muted/50 transition-colors ml-1"
						title="Reset zoom"
					>
						<RotateCcw className="h-3.5 w-3.5" />
					</button>
					<button
						onClick={() => setIsFullscreen(!isFullscreen)}
						className="p-1.5 rounded-md hover:bg-muted/50 transition-colors"
						title={isFullscreen ? "Exit fullscreen" : "Fullscreen"}
					>
						<Maximize2 className="h-3.5 w-3.5" />
					</button>
				</div>
			</div>

			{/* Diagram Container */}
			<div className={`p-6 bg-gradient-to-b from-background/80 to-background overflow-auto ${isFullscreen ? 'h-[calc(100%-50px)]' : 'min-h-[300px]'}`}>
				{isLoading && !error && (
					<div className="flex flex-col items-center gap-3 text-muted-foreground animate-pulse py-8">
						<div className="relative">
							<GitBranch className="h-10 w-10 opacity-30" />
							<div className="absolute inset-0 flex items-center justify-center">
								<div className="h-3 w-3 rounded-full bg-primary animate-ping" />
							</div>
						</div>
						<span className="text-sm font-medium">Rendering diagram...</span>
					</div>
				)}

				<div
					ref={containerRef}
					className={`mermaid-container transition-all duration-200 ${isLoading && !error ? "opacity-0 absolute" : "opacity-100"}`}
					style={{
						transform: `scale(${zoom})`,
						transformOrigin: 'center top',
						display: "flex",
						justifyContent: "center",
					}}
				/>

				{error && (
					<div className="w-full space-y-4">
						<div className="text-sm text-red-400/90 bg-red-500/10 p-4 rounded-lg border border-red-500/20 font-medium">
							⚠️ {error}
						</div>
						<div className="relative">
							<div className="absolute top-2 right-2 text-xs text-muted-foreground/70 bg-background/90 px-2 py-1 rounded-md border border-border/30">
								Raw Mermaid Code
							</div>
							<pre className="text-xs font-mono bg-black/30 p-4 pt-8 rounded-lg overflow-x-auto text-muted-foreground/80 border border-border/20">
								{cleanCode}
							</pre>
						</div>
					</div>
				)}
			</div>

			{/* Fullscreen backdrop */}
			{isFullscreen && (
				<div
					className="fixed inset-0 bg-black/50 -z-10"
					onClick={() => setIsFullscreen(false)}
				/>
			)}
		</div>
	)
}

export default MermaidDiagram
