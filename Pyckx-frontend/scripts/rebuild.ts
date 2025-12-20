// Bun-native frontend rebuild script with file watching
// Uses Bun's built-in file system watcher - faster than chokidar!

import { $ } from "bun";
import { watch } from "fs";
import { resolve } from "path";

const WATCH_DIRS = ["app", "components", "lib", "public"];
const DEBOUNCE_MS = 1500;

let debounceTimer: ReturnType<typeof setTimeout> | null = null;
let isBuilding = false;

async function rebuild() {
	if (isBuilding) return;
	isBuilding = true;

	console.log("\n[NEXT] 🔄 Rebuilding frontend...");
	const start = Date.now();

	try {
		await $`BUILD_MODE=export bun run next build`.quiet();
		await $`rm -rf ../backend/static`.quiet();
		await $`cp -r out ../backend/static`.quiet();
		console.log(`[NEXT] ✅ Built in ${Date.now() - start}ms → backend/static`);
	} catch (error) {
		console.error("[NEXT] ❌ Build failed:", error);
	} finally {
		isBuilding = false;
	}
}

function scheduleRebuild() {
	if (debounceTimer) clearTimeout(debounceTimer);
	debounceTimer = setTimeout(rebuild, DEBOUNCE_MS);
}

// Initial build
await rebuild();

// Watch directories
console.log("[NEXT] 👀 Watching for changes...");
for (const dir of WATCH_DIRS) {
	const fullPath = resolve(import.meta.dir, "..", dir);
	watch(fullPath, { recursive: true }, (event, filename) => {
		if (filename && !filename.includes("node_modules")) {
			console.log(`[NEXT] 📝 ${event}: ${dir}/${filename}`);
			scheduleRebuild();
		}
	});
}

// Keep process alive
await new Promise(() => { });

