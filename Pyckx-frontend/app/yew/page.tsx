"use client";

import { useEffect, useRef } from "react";

/**
 * Yew Test Page
 * 
 * This page dynamically loads and renders a Yew (Rust WebAssembly) component.
 * The Yew app is built with Trunk and outputs to /public/yew-wasm/
 * 
 * NOTE: We use script/link tag injection instead of dynamic import() because
 * Next.js Turbopack cannot resolve dynamic import paths at build time.
 */
export default function YewPage() {
	const isMounted = useRef(false);

	useEffect(() => {
		// Prevent double-mounting in React 18 strict mode
		if (isMounted.current) return;
		isMounted.current = true;

		// Load Yew CSS and WASM
		const initYew = async () => {
			try {
				// 1. Inject the CSS stylesheet
				const link = document.createElement("link");
				link.rel = "stylesheet";
				link.href = "/yew-wasm/styles-73a30a68b3ee46f7.css";
				document.head.appendChild(link);

				// 2. Inject the WASM loader script
				const script = document.createElement("script");
				script.type = "module";
				script.textContent = `
          import init from '/yew-wasm/yew-app-9df1a184133a423e.js';
          await init('/yew-wasm/yew-app-9df1a184133a423e_bg.wasm');
          console.log('✅ Yew WASM initialized successfully!');
        `;
				document.head.appendChild(script);
			} catch (error) {
				console.error("❌ Failed to load Yew WASM:", error);
			}
		};

		initYew();

		// Cleanup on unmount
		return () => {
			// Remove injected CSS when leaving page
			const links = document.querySelectorAll('link[href*="yew-wasm"]');
			links.forEach((link) => link.remove());
		};
	}, []);

	return (
		<div className="min-h-screen">
			{/* The Yew app renders to document.body with its own styles */}
			<noscript>
				<div className="flex items-center justify-center min-h-screen text-white bg-[#0a0a1a]">
					<p>JavaScript is required to run this Yew application.</p>
				</div>
			</noscript>
		</div>
	);
}
