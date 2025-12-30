/**
 * PDF and text extraction utilities
 * Uses dynamic import for pdfjs-dist to avoid SSR issues (DOMMatrix not defined in Node.js)
 */

let pdfjsLib: typeof import('pdfjs-dist') | null = null;

async function getPdfLib() {
	if (pdfjsLib) return pdfjsLib;

	if (typeof window === 'undefined') {
		throw new Error('PDF extraction is only available in browser environment');
	}

	pdfjsLib = await import('pdfjs-dist');
	pdfjsLib.GlobalWorkerOptions.workerSrc = `//unpkg.com/pdfjs-dist@${pdfjsLib.version}/build/pdf.worker.min.mjs`;
	return pdfjsLib;
}

export async function extractTextFromFile(file: File): Promise<string> {
	if (file.type === "application/pdf") {
		return extractTextFromPdf(file);
	} else {
		return extractTextFromPlain(file);
	}
}

async function extractTextFromPlain(file: File): Promise<string> {
	return new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.onload = (e) => resolve(e.target?.result as string || "");
		reader.onerror = (e) => reject(e);
		reader.readAsText(file);
	});
}

async function extractTextFromPdf(file: File): Promise<string> {
	const pdfjs = await getPdfLib();
	const arrayBuffer = await file.arrayBuffer();
	// @ts-ignore - type definition mismatch sometimes occurs with pdfjs-dist
	const loadingTask = pdfjs.getDocument({ data: arrayBuffer });
	const pdf = await loadingTask.promise;
	let fullText = "";

	for (let i = 1; i <= pdf.numPages; i++) {
		const page = await pdf.getPage(i);
		const textContent = await page.getTextContent();
		// @ts-ignore - item has str property
		const pageText = textContent.items.map((item: any) => item.str).join(" ");
		fullText += pageText + "\n\n";
	}

	return fullText;
}
