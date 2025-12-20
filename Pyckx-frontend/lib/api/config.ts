// == API Endpoints - Single Source of Truth // ==
// Pattern: /api/{service}/{feature}/{id}

export const endpoints = {
  // == Auth: /api/auth ==
  auth: {
    login: "/api/auth/login",
    register: "/api/auth/register",
    logout: "/api/auth/logout",
  },

  // == User: /api/user ==
  user: {
    me: "/api/user/me",
  },

  // == Apps: /api/apps ==
  apps: {
    list: "/api/apps",
    userApps: "/api/user/apps",
  },

  // == Collection: /api/collection ==
  collection: {
    base: "/api/collection",
    dvds: "/api/collection/dvds",
    dvd: (id: string) => `/api/collection/dvds/${id}`,
  },

  // == Intello: /api/intello ==
  intello: {
    games: "/api/intello/games",
    models: "/api/intello/models",
    qcm: "/api/intello/qcm",
    qcmById: (id: string) => `/api/intello/qcm/${id}`,
    qcmGenerate: "/api/intello/qcm/generate",
    openQuestions: "/api/intello/open-questions",
    openQuestionsCheck: "/api/intello/open-questions/check",
    flashcards: "/api/intello/flashcards",
    trueFalse: "/api/intello/true-false",
    keywords: "/api/intello/keywords",
    orderPhrases: "/api/intello/order-phrases",
    fillBlanks: "/api/intello/fill-blanks",
  },
} as const;
