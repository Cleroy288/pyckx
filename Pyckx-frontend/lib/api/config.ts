// == API Configuration - Single Source of Truth for all API URLs // ==
// All endpoints are loaded from environment variables (.env file)

// == Base URL // ==
// Configure via NEXT_PUBLIC_API_BASE_URL or default to same origin
const API_BASE = process.env.NEXT_PUBLIC_API_BASE_URL ?? "";

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

// == API Endpoints // ==
export const endpoints = {
  auth: {
    login: () => buildUrl(process.env.NEXT_PUBLIC_AUTH_LOGIN, "/auth/login"),
    register: () => buildUrl(process.env.NEXT_PUBLIC_AUTH_REGISTER, "/auth/register"),
    logout: () => buildUrl(process.env.NEXT_PUBLIC_AUTH_LOGOUT, "/auth/logout"),
  },
  user: {
    me: () => buildUrl(process.env.NEXT_PUBLIC_USER_ME, "/user/me"),
  },
  apps: {
    list: () => buildUrl(process.env.NEXT_PUBLIC_APPS_LIST, "/api/apps"),
    userApps: () => buildUrl(process.env.NEXT_PUBLIC_APPS_USER, "/api/user/apps"),
  },
  collection: {
    base: () => buildUrl(process.env.NEXT_PUBLIC_COLLECTION_BASE, "/app/collection"),
    // DVD endpoints
    addDvd: () => buildUrl(process.env.NEXT_PUBLIC_COLLECTION_ADD_DVD, "/app/collection/add-dvd"),
    getUserDvds: () => buildUrl(process.env.NEXT_PUBLIC_COLLECTION_GET_USER_DVDS, "/app/collection/get-user-dvd"),
    getDvd: (id: string) => buildDynamicUrl(process.env.NEXT_PUBLIC_COLLECTION_GET_DVD, "/app/collection/dvd/{id}", id),
    updateDvd: (id: string) => buildDynamicUrl(process.env.NEXT_PUBLIC_COLLECTION_UPDATE_DVD, "/app/collection/mod-dvd/{id}", id),
    deleteDvd: (id: string) => buildDynamicUrl(process.env.NEXT_PUBLIC_COLLECTION_DELETE_DVD, "/app/collection/del/{id}", id),
  },
  intello: {
    // Games endpoint
    games: () => buildUrl(process.env.NEXT_PUBLIC_INTELLO_GAMES, "/app/intello/games"),
    // Models endpoint - list available AI models
    models: () => buildUrl(process.env.NEXT_PUBLIC_INTELLO_MODELS, "/app/intello/models"),
    // QCM endpoints
    qcmSets: () => buildUrl(process.env.NEXT_PUBLIC_INTELLO_QCM, "/app/intello/qcm"),
    qcmSet: (id: string) => buildDynamicUrl(process.env.NEXT_PUBLIC_INTELLO_QCM_ID, "/app/intello/qcm/{id}", id),
    // Custom question endpoint - uses direct backend URL to bypass Next.js proxy size limits
    customQuestion: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/custom-question`;
    },
    // Open question endpoints
    openQuestionCreate: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/open-question/create`;
    },
    openQuestionList: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/open-question/list`;
    },
    openQuestionCheck: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/open-question/check`;
    },
    // Flashcard endpoints
    flashcardCreate: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/flashcard/create`;
    },
    flashcardList: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/flashcard/list`;
    },
    // True or False endpoints
    trueOrFalseCreate: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/true-false/create`;
    },
    trueOrFalseList: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/true-false/list`;
    },
    // Keywords endpoints
    keywordsCreate: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/keywords/create`;
    },
    keywordsList: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/keywords/list`;
    },
    // Order Phrase endpoints
    orderPhraseCreate: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/order-phrase/create`;
    },
    orderPhraseList: () => {
      const directBackend = process.env.NEXT_PUBLIC_BACKEND_DIRECT_URL || "http://localhost:8080";
      return `${directBackend}/app/intello/order-phrase/list`;
    },
  },
} as const;
