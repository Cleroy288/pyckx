// == API Configuration - Single Source of Truth for all API URLs // ==
// All endpoints are loaded from environment variables (.env file)

// == Base URL // ==
// Configure via NEXT_PUBLIC_API_BASE_URL or default to same origin
const API_BASE = process.env.NEXT_PUBLIC_API_BASE_URL ?? "";

// == Direct Backend URL for large file uploads // ==
const DIRECT_BACKEND = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";

// == Helper: Build full URL from env variable // ==
function buildUrl(envPath: string | undefined, fallback: string): string {
  const path = envPath ?? fallback;
  return `${API_BASE}${path}`;
}

// == Helper: Build dynamic URL with ID parameter // ==
function buildDynamicUrl(envPath: string | undefined, fallback: string, id: string): string {
  const path = envPath ?? fallback;
  return `${API_BASE}${path.replace("{id}", id)}`;
}

// == Helper: Build direct backend URL (bypasses Next.js proxy for large uploads) // ==
function buildDirectUrl(path: string): string {
  return `${DIRECT_BACKEND}${path}`;
}

// == API Endpoints // ==
// Pattern: /api/{service}/{feature}/{id}
export const endpoints = {
  // == Auth Service: /api/auth ==
  auth: {
    login: () => buildUrl(process.env.NEXT_PUBLIC_AUTH_LOGIN, "/api/auth/login"),
    register: () => buildUrl(process.env.NEXT_PUBLIC_AUTH_REGISTER, "/api/auth/register"),
    logout: () => buildUrl(process.env.NEXT_PUBLIC_AUTH_LOGOUT, "/api/auth/logout"),
  },

  // == User Service: /api/user ==
  user: {
    me: () => buildUrl(process.env.NEXT_PUBLIC_USER_ME, "/api/user/me"),
  },

  // == Apps Service: /api/apps ==
  apps: {
    list: () => buildUrl(process.env.NEXT_PUBLIC_APPS_LIST, "/api/apps"),
    userApps: () => buildUrl(process.env.NEXT_PUBLIC_APPS_USER, "/api/user/apps"),
  },

  // == Collection Service: /api/collection ==
  collection: {
    base: () => buildUrl(process.env.NEXT_PUBLIC_COLLECTION_BASE, "/api/collection"),
    // DVD endpoints - RESTful: /api/collection/dvds
    dvds: () => buildUrl(process.env.NEXT_PUBLIC_COLLECTION_DVDS, "/api/collection/dvds"),
    dvd: (id: string) => buildDynamicUrl(process.env.NEXT_PUBLIC_COLLECTION_DVD, "/api/collection/dvds/{id}", id),
  },

  // == Intello Service: /api/intello ==
  intello: {
    // Games & Models
    games: () => buildUrl(process.env.NEXT_PUBLIC_INTELLO_GAMES, "/api/intello/games"),
    models: () => buildUrl(process.env.NEXT_PUBLIC_INTELLO_MODELS, "/api/intello/models"),

    // QCM: /api/intello/qcm
    qcm: () => buildUrl(process.env.NEXT_PUBLIC_INTELLO_QCM, "/api/intello/qcm"),
    qcmById: (id: string) => buildDynamicUrl(process.env.NEXT_PUBLIC_INTELLO_QCM_ID, "/api/intello/qcm/{id}", id),
    qcmGenerate: () => buildDirectUrl("/api/intello/qcm/generate"),

    // Open Questions: /api/intello/open-questions
    openQuestions: () => buildDirectUrl("/api/intello/open-questions"),
    openQuestionsCheck: () => buildDirectUrl("/api/intello/open-questions/check"),

    // Flashcards: /api/intello/flashcards
    flashcards: () => buildDirectUrl("/api/intello/flashcards"),

    // True/False: /api/intello/true-false
    trueFalse: () => buildDirectUrl("/api/intello/true-false"),

    // Keywords: /api/intello/keywords
    keywords: () => buildDirectUrl("/api/intello/keywords"),

    // Order Phrases: /api/intello/order-phrases
    orderPhrases: () => buildDirectUrl("/api/intello/order-phrases"),

    // Fill Blanks: /api/intello/fill-blanks
    fillBlanks: () => buildDirectUrl("/api/intello/fill-blanks"),
  },
} as const;
