"use client";

import { useState, useEffect } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { Loader2, Plus, X, FileText, Upload, ArrowLeft, CheckSquare } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import {
	Form,
	FormControl,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";

import {
	createCourse,
	getUserResources,
	checkResourceExists,
	createResource,
	linkResourceToCourse,
	type ResourceSummary,
} from "@/lib/api/intello";
import { extractTextFromFile } from "@/lib/pdf-utils";
import { useIntello } from "../../context";

// Schema
const formSchema = z.object({
	name: z.string().min(2, "Name must be at least 2 characters"),
	description: z.string().min(10, "Description must be at least 10 characters"),
});

type FormValues = z.infer<typeof formSchema>;

export function CreateCourseView() {
	const { handleBackToHome, handleNavigateToCourses } = useIntello();
	const [isSubmitting, setIsSubmitting] = useState(false);
	const [files, setFiles] = useState<File[]>([]);
	const [uploadProgress, setUploadProgress] = useState<string>("");

	// Resource management state
	const [existingResources, setExistingResources] = useState<ResourceSummary[]>([]);
	const [selectedResourceIds, setSelectedResourceIds] = useState<Set<string>>(new Set());
	const [loadingResources, setLoadingResources] = useState(true);

	const form = useForm<FormValues>({
		resolver: zodResolver(formSchema),
		defaultValues: {
			name: "",
			description: "",
		},
	});

	// Load existing resources on mount
	useEffect(() => {
		getUserResources()
			.then(setExistingResources)
			.catch((err) => {
				console.error("Failed to load resources:", err);
				toast.error("Failed to load existing resources");
			})
			.finally(() => setLoadingResources(false));
	}, []);

	const toggleResource = (id: string, checked: boolean) => {
		setSelectedResourceIds((prev) => {
			const next = new Set(prev);
			if (checked) {
				next.add(id);
			} else {
				next.delete(id);
			}
			return next;
		});
	};

	const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		if (e.target.files) {
			const newFiles = Array.from(e.target.files);
			setFiles((prev) => [...prev, ...newFiles]);
		}
	};

	const removeFile = (index: number) => {
		setFiles((prev) => prev.filter((_, i) => i !== index));
	};

	const calculateTokens = (text: string) => {
		return Math.ceil(text.length / 4);
	};

	const onSubmit = async (values: FormValues) => {
		setIsSubmitting(true);
		setUploadProgress("Creating course...");

		try {
			// 1. Create course
			const courseData = await createCourse(values.name, values.description);
			const courseId = courseData.id;

			// 2. Link selected existing resources
			if (selectedResourceIds.size > 0) {
				setUploadProgress(`Linking ${selectedResourceIds.size} existing resources...`);
				for (const resourceId of selectedResourceIds) {
					try {
						await linkResourceToCourse(courseId, resourceId);
					} catch (err) {
						console.error(`Failed to link resource ${resourceId}:`, err);
						toast.error(`Failed to link resource: ${err instanceof Error ? err.message : "Unknown error"}`);
					}
				}
			}

			// 3. Process new file uploads with duplicate detection
			if (files.length > 0) {
				setUploadProgress(`Processing ${files.length} files...`);

				for (let i = 0; i < files.length; i++) {
					const file = files[i];
					setUploadProgress(`Processing ${file.name} (${i + 1}/${files.length})...`);

					try {
						// Check if resource already exists
						const exists = await checkResourceExists(file.name);

						if (exists) {
							// Find the existing resource and link it
							const existing = existingResources.find((r) => r.filename === file.name);
							if (existing) {
								toast.info(`"${file.name}" already exists, linking existing resource`);
								await linkResourceToCourse(courseId, existing.id);
							} else {
								toast.warning(`"${file.name}" exists but couldn't be linked`);
							}
						} else {
							// Create new resource and link it
							const content = await extractTextFromFile(file);
							const tokenCount = calculateTokens(content);
							const newResource = await createResource(file.name, content, tokenCount);
							await linkResourceToCourse(courseId, newResource.id);
						}
					} catch (err) {
						console.error(`Failed to process ${file.name}`, err);
						toast.error(`Failed to upload ${file.name}: ${err instanceof Error ? err.message : "Unknown error"}`);
					}
				}
			}

			toast.success("Course created successfully!");
			handleNavigateToCourses();
		} catch (error) {
			console.error(error);
			toast.error(error instanceof Error ? error.message : "Failed to create course");
		} finally {
			setIsSubmitting(false);
			setUploadProgress("");
		}
	};

	return (
		<div className="space-y-6">
			{/* Header */}
			<div className="flex items-center gap-3">
				<Button variant="ghost" size="icon" onClick={handleBackToHome}>
					<ArrowLeft className="h-4 w-4" />
				</Button>
				<div>
					<h1 className="text-xl font-bold tracking-tight">Create New Course</h1>
					<p className="text-xs text-muted-foreground">
						Create a course and add learning materials
					</p>
				</div>
			</div>

			<Form {...form}>
				<form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
					<Card>
						<CardHeader className="pb-3">
							<CardTitle className="text-base">Course Details</CardTitle>
							<CardDescription className="text-xs">Basic information about your course.</CardDescription>
						</CardHeader>
						<CardContent className="space-y-4">
							<FormField
								control={form.control}
								name="name"
								render={({ field }) => (
									<FormItem>
										<FormLabel className="text-sm">Course Name</FormLabel>
										<p className="text-xs text-muted-foreground mb-2">
											Give your course a clear, descriptive title (e.g., "Introduction to Rust", "Advanced React Patterns")
										</p>
										<FormControl>
											<Input placeholder="e.g. Introduction to Rust" {...field} />
										</FormControl>
										<FormMessage />
									</FormItem>
								)}
							/>
							<FormField
								control={form.control}
								name="description"
								render={({ field }) => (
									<FormItem>
										<FormLabel className="text-sm">Description</FormLabel>
										<p className="text-xs text-muted-foreground mb-2">
											Describe what you want to learn. Be specific about topics, concepts, or skills you'd like to cover.
										</p>
										<FormControl>
											<Textarea
												placeholder="Briefly describe what this course is about..."
												className="min-h-[80px]"
												{...field}
											/>
										</FormControl>
										<FormMessage />
									</FormItem>
								)}
							/>
						</CardContent>
					</Card>

					{/* Existing Resources Picker */}
					<Card>
						<CardHeader className="pb-3">
							<CardTitle className="text-base flex items-center gap-2">
								<CheckSquare className="h-4 w-4" />
								Your Resources
							</CardTitle>
							<CardDescription className="text-xs">
								Select existing resources to include in this course
							</CardDescription>
						</CardHeader>
						<CardContent>
							{loadingResources ? (
								<div className="flex items-center gap-2 text-sm text-muted-foreground">
									<Loader2 className="h-4 w-4 animate-spin" />
									Loading resources...
								</div>
							) : existingResources.length === 0 ? (
								<p className="text-sm text-muted-foreground">No existing resources. Upload new ones below.</p>
							) : (
								<div className="space-y-2 max-h-40 overflow-y-auto pr-2">
									{existingResources.map((resource) => (
										<label
											key={resource.id}
											className="flex items-center gap-3 p-2 hover:bg-muted rounded-md cursor-pointer transition-colors"
										>
											<Checkbox
												checked={selectedResourceIds.has(resource.id)}
												onCheckedChange={(checked) => toggleResource(resource.id, checked === true)}
											/>
											<FileText className="h-4 w-4 shrink-0 text-muted-foreground" />
											<span className="text-sm truncate flex-1">{resource.filename}</span>
											<span className="text-xs text-muted-foreground">
												{resource.token_count} tokens
											</span>
										</label>
									))}
								</div>
							)}
							{selectedResourceIds.size > 0 && (
								<p className="text-xs text-primary mt-2">
									{selectedResourceIds.size} resource(s) selected
								</p>
							)}
						</CardContent>
					</Card>

					{/* Upload New Resources */}
					<Card>
						<CardHeader className="pb-3">
							<CardTitle className="text-base flex items-center gap-2">
								<Upload className="h-4 w-4" />
								Upload New Resources
							</CardTitle>
							<CardDescription className="text-xs">
								Upload documents (.txt, .md, .pdf) as new resources
							</CardDescription>
						</CardHeader>
						<CardContent className="space-y-4">
							<div className="border-2 border-dashed rounded-lg p-4 flex flex-col items-center justify-center text-center hover:bg-muted/50 transition-colors cursor-pointer relative">
								<input
									type="file"
									onChange={handleFileChange}
									multiple
									accept=".txt,.md,.pdf"
									className="absolute inset-0 opacity-0 cursor-pointer"
								/>
								<Upload className="h-6 w-6 mb-2 text-muted-foreground" />
								<p className="text-sm font-medium">Click to upload</p>
								<p className="text-xs text-muted-foreground mt-1">PDF, TXT, MD (Max 10MB)</p>
							</div>

							{files.length > 0 && (
								<div className="space-y-2">
									{files.map((file, index) => (
										<div key={index} className="flex items-center justify-between p-2 bg-muted rounded-md text-sm">
											<div className="flex items-center gap-2 overflow-hidden">
												<FileText className="h-4 w-4 shrink-0" />
												<span className="truncate">{file.name}</span>
												<span className="text-xs text-muted-foreground">
													({(file.size / 1024).toFixed(0)}KB)
												</span>
											</div>
											<Button
												type="button"
												variant="ghost"
												size="icon"
												className="h-6 w-6 text-destructive hover:text-destructive/90"
												onClick={() => removeFile(index)}
											>
												<X className="h-3 w-3" />
											</Button>
										</div>
									))}
								</div>
							)}
						</CardContent>
					</Card>

					<div className="flex justify-end gap-3">
						<Button type="button" variant="outline" size="sm" onClick={handleBackToHome}>
							Cancel
						</Button>
						<Button type="submit" size="sm" disabled={isSubmitting}>
							{isSubmitting ? (
								<>
									<Loader2 className="mr-2 h-4 w-4 animate-spin" />
									{uploadProgress || "Creating..."}
								</>
							) : (
								<>
									<Plus className="mr-2 h-4 w-4" />
									Create Course
								</>
							)}
						</Button>
					</div>
				</form>
			</Form>
		</div>
	);
}
