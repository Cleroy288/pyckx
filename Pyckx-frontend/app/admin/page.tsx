"use client"

import { useEffect, useState } from "react"
import { BarChart3, DollarSign, Cpu, Zap, ArrowLeft, Loader2, AlertCircle } from "lucide-react"
import Link from "next/link"
import { fetchAdminStats, type AdminStatsResponse } from "@/lib/api/intello"

export default function AdminStatsPage() {
	const [stats, setStats] = useState<AdminStatsResponse | null>(null)
	const [loading, setLoading] = useState(true)
	const [error, setError] = useState<string | null>(null)

	useEffect(() => {
		fetchAdminStats()
			.then(setStats)
			.catch((e) => setError(e.message))
			.finally(() => setLoading(false))
	}, [])

	if (loading) {
		return (
			<div className="min-h-screen bg-background flex items-center justify-center">
				<Loader2 className="h-8 w-8 animate-spin text-primary" />
			</div>
		)
	}

	if (error) {
		return (
			<div className="min-h-screen bg-background flex items-center justify-center p-4">
				<div className="max-w-md w-full p-6 rounded-xl border border-destructive/50 bg-destructive/10">
					<div className="flex items-center gap-3 mb-4">
						<AlertCircle className="h-6 w-6 text-destructive" />
						<h1 className="text-lg font-semibold text-destructive">Access Denied</h1>
					</div>
					<p className="text-sm text-muted-foreground">{error}</p>
					<Link
						href="/intello"
						className="mt-4 inline-flex items-center gap-2 text-sm text-primary hover:underline"
					>
						<ArrowLeft className="h-4 w-4" />
						Back to Intello
					</Link>
				</div>
			</div>
		)
	}

	if (!stats) return null

	// Format numbers
	const formatNumber = (n: number) => n.toLocaleString()
	const formatCurrency = (n: number) => `$${n.toFixed(4)}`
	const formatTokens = (n: number) => {
		if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(2)}M`
		if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`
		return n.toString()
	}

	return (
		<div className="min-h-screen bg-background p-6">
			<div className="max-w-5xl mx-auto space-y-6">
				{/* Header */}
				<div className="flex items-center gap-4">
					<Link
						href="/intello"
						className="p-2 rounded-lg hover:bg-muted/50 transition-colors"
					>
						<ArrowLeft className="h-5 w-5" />
					</Link>
					<div>
						<h1 className="text-2xl font-bold tracking-tight text-foreground">Admin Statistics</h1>
						<p className="text-sm text-muted-foreground">AI usage tracking and cost analysis</p>
					</div>
				</div>

				{/* Summary Cards */}
				<div className="grid grid-cols-2 md:grid-cols-4 gap-4">
					<div className="p-4 rounded-xl border border-border/50 bg-card/30">
						<div className="flex items-center gap-2 mb-2">
							<BarChart3 className="h-4 w-4 text-primary" />
							<span className="text-xs text-muted-foreground">Total Requests</span>
						</div>
						<p className="text-2xl font-bold text-foreground">{formatNumber(stats.total_requests)}</p>
					</div>

					<div className="p-4 rounded-xl border border-border/50 bg-card/30">
						<div className="flex items-center gap-2 mb-2">
							<DollarSign className="h-4 w-4 text-green-500" />
							<span className="text-xs text-muted-foreground">Total Cost</span>
						</div>
						<p className="text-2xl font-bold text-foreground">{formatCurrency(stats.total_cost_usd)}</p>
					</div>

					<div className="p-4 rounded-xl border border-border/50 bg-card/30">
						<div className="flex items-center gap-2 mb-2">
							<Cpu className="h-4 w-4 text-blue-500" />
							<span className="text-xs text-muted-foreground">Input Tokens</span>
						</div>
						<p className="text-2xl font-bold text-foreground">{formatTokens(stats.total_input_tokens)}</p>
					</div>

					<div className="p-4 rounded-xl border border-border/50 bg-card/30">
						<div className="flex items-center gap-2 mb-2">
							<Zap className="h-4 w-4 text-yellow-500" />
							<span className="text-xs text-muted-foreground">Output Tokens</span>
						</div>
						<p className="text-2xl font-bold text-foreground">{formatTokens(stats.total_output_tokens)}</p>
					</div>
				</div>

				{/* Feature Breakdown Table */}
				<div className="p-4 rounded-xl border border-border/50 bg-card/30">
					<h2 className="font-semibold text-foreground mb-4">Usage by Feature</h2>

					<div className="overflow-x-auto">
						<table className="w-full text-sm">
							<thead>
								<tr className="border-b border-border/50">
									<th className="text-left py-2 px-3 text-muted-foreground font-medium">Feature</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Requests</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Total Cost</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Avg Cost</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Input Tokens</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Output Tokens</th>
								</tr>
							</thead>
							<tbody>
								{stats.by_feature_type.map((feature) => (
									<tr key={feature.feature_type} className="border-b border-border/30 hover:bg-muted/30 transition-colors">
										<td className="py-2 px-3 font-mono text-xs text-foreground">{feature.feature_type}</td>
										<td className="py-2 px-3 text-right text-foreground">{formatNumber(feature.count)}</td>
										<td className="py-2 px-3 text-right text-green-500">{formatCurrency(feature.total_cost_usd)}</td>
										<td className="py-2 px-3 text-right text-muted-foreground">{formatCurrency(feature.avg_cost_usd)}</td>
										<td className="py-2 px-3 text-right text-blue-500">{formatTokens(feature.total_input_tokens)}</td>
										<td className="py-2 px-3 text-right text-yellow-500">{formatTokens(feature.total_output_tokens)}</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>

				{/* Model Usage Table */}
				<div className="p-4 rounded-xl border border-border/50 bg-card/30">
					<h2 className="font-semibold text-foreground mb-4">Usage by Model</h2>

					<div className="overflow-x-auto">
						<table className="w-full text-sm">
							<thead>
								<tr className="border-b border-border/50">
									<th className="text-left py-2 px-3 text-muted-foreground font-medium">Model</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Requests</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Total Cost</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Avg Cost</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Input Tokens</th>
									<th className="text-right py-2 px-3 text-muted-foreground font-medium">Output Tokens</th>
								</tr>
							</thead>
							<tbody>
								{stats.by_model.map((model) => (
									<tr key={model.model_id} className="border-b border-border/30 hover:bg-muted/30 transition-colors">
										<td className="py-2 px-3 font-mono text-xs text-foreground">{model.model_id}</td>
										<td className="py-2 px-3 text-right text-foreground">{formatNumber(model.count)}</td>
										<td className="py-2 px-3 text-right text-green-500">{formatCurrency(model.total_cost_usd)}</td>
										<td className="py-2 px-3 text-right text-muted-foreground">{formatCurrency(model.avg_cost_usd)}</td>
										<td className="py-2 px-3 text-right text-blue-500">{formatTokens(model.total_input_tokens)}</td>
										<td className="py-2 px-3 text-right text-yellow-500">{formatTokens(model.total_output_tokens)}</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	)
}

