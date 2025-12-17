// Color keys that map to the palette system
export type AppColorKey = "primary" | "secondary" | "accent" | "muted" | "success" | "warning" | "destructive"

export type App = {
  id: string
  name: string
  shortDescription: string
  description: string
  icon: string
  colorKey: AppColorKey  // Uses palette color keys instead of hex
  category: string
  users: string
  rating: number
  isNew: boolean
  isPro: boolean
  version: string
  lastUpdated: string
  features: string[]
}

export const availableApps: App[] = [
  {
    id: "collection",
    name: "Collection",
    shortDescription: "Organize and catalog all your collections",
    description:
      "Keep track of everything you own and love. From DVDs and Blu-rays to books, vinyl records, video games, and more. Collection helps you catalog, organize, and manage your personal collections with detailed entries, cover images, and smart search.",
    icon: "Library",
    colorKey: "primary",  // Uses palette primary color
    category: "Personal",
    users: "5.2k",
    rating: 4.9,
    isNew: true,
    isPro: false,
    version: "1.0.0",
    lastUpdated: "Jan 2, 2025",
    features: [
      "DVD & Blu-ray cataloging",
      "Book library management",
      "Video game tracking",
      "Vinyl & music collection",
      "Custom categories",
      "Barcode scanning",
    ],
  },
  {
    id: "intello",
    name: "Intello",
    shortDescription: "Create and practice QCM quizzes",
    description:
      "Build your knowledge with custom QCM (Multiple Choice Questions) sets. Create quizzes on any subject, organize them by difficulty level, and test yourself to improve your learning. Perfect for students, teachers, and lifelong learners.",
    icon: "Brain",
    colorKey: "accent",
    category: "Education",
    users: "1.2k",
    rating: 4.7,
    isNew: true,
    isPro: false,
    version: "1.0.0",
    lastUpdated: "Dec 13, 2025",
    features: [
      "Custom QCM creation",
      "Multiple subjects per set",
      "Difficulty levels (Easy, Medium, Hard)",
      "Detailed explanations",
      "Progress tracking",
      "Quiz practice mode",
    ],
  },
]

// Helper to get app by ID
export const getAppById = (id: string): App | undefined => {
  return availableApps.find((app) => app.id === id)
}
