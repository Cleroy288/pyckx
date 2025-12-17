# API Configuration

This document describes how API endpoints are configured and managed in the PYCKX frontend application.

## Overview

All API endpoints are centralized in `lib/api/config.ts` and can be customized via environment variables in the `.env` file. This provides a single source of truth for all API URLs.

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│     .env        │────▶│  lib/api/config  │────▶│   API Calls     │
│  (endpoints)    │     │   (endpoints)    │     │  (fetch/axios)  │
└─────────────────┘     └──────────────────┘     └─────────────────┘
```

## Environment Variables

### Base URL

```bash
# Leave empty for same-origin (production)
# Set to backend URL for development
NEXT_PUBLIC_API_BASE_URL=
```

### Auth Endpoints

```bash
NEXT_PUBLIC_AUTH_LOGIN=/auth/login
NEXT_PUBLIC_AUTH_REGISTER=/auth/register
NEXT_PUBLIC_AUTH_LOGOUT=/auth/logout
```

### User Endpoints

```bash
NEXT_PUBLIC_USER_ME=/user/me
```

### Apps Endpoints

```bash
NEXT_PUBLIC_APPS_LIST=/api/apps
NEXT_PUBLIC_APPS_USER=/api/user/apps
```

### Collection Endpoints

```bash
NEXT_PUBLIC_COLLECTION_BASE=/app/collection
NEXT_PUBLIC_COLLECTION_ADD_DVD=/app/collection/add-dvd
NEXT_PUBLIC_COLLECTION_GET_USER_DVDS=/app/collection/get-user-dvd

# Dynamic endpoints (use {id} as placeholder)
NEXT_PUBLIC_COLLECTION_GET_DVD=/app/collection/dvd/{id}
NEXT_PUBLIC_COLLECTION_UPDATE_DVD=/app/collection/mod-dvd/{id}
NEXT_PUBLIC_COLLECTION_DELETE_DVD=/app/collection/del/{id}
```

### Development Proxy

```bash
# Server-side only (not exposed to browser)
BACKEND_URL=http://localhost:8080
```

## Config Module

### Location

`lib/api/config.ts`

### Usage

```typescript
import { endpoints } from "@/lib/api/config";

// Static endpoints
const loginUrl = endpoints.auth.login();
// → "/auth/login" or "http://localhost:8080/auth/login"

// Dynamic endpoints with ID
const dvdUrl = endpoints.collection.getDvd("123");
// → "/app/collection/dvd/123"
```

### Helper Functions

```typescript
// Build URL from env variable with fallback
function buildUrl(envPath: string | undefined, fallback: string): string

// Build dynamic URL with ID parameter replacement
function buildDynamicUrl(envPath: string | undefined, fallback: string, id: string): string
```

## Endpoint Reference

| Endpoint | Method | Function | Description |
|----------|--------|----------|-------------|
| `/auth/login` | POST | `endpoints.auth.login()` | User login |
| `/auth/register` | POST | `endpoints.auth.register()` | User registration |
| `/auth/logout` | POST | `endpoints.auth.logout()` | User logout |
| `/user/me` | GET | `endpoints.user.me()` | Get current user |
| `/api/apps` | GET | `endpoints.apps.list()` | List all apps |
| `/api/user/apps` | GET/POST/DELETE | `endpoints.apps.userApps()` | User's apps |
| `/app/collection/add-dvd` | POST | `endpoints.collection.addDvd()` | Add DVD |
| `/app/collection/get-user-dvd` | GET | `endpoints.collection.getUserDvds()` | Get user's DVDs |
| `/app/collection/dvd/{id}` | GET | `endpoints.collection.getDvd(id)` | Get single DVD |
| `/app/collection/mod-dvd/{id}` | PUT | `endpoints.collection.updateDvd(id)` | Update DVD |
| `/app/collection/del/{id}` | DELETE | `endpoints.collection.deleteDvd(id)` | Delete DVD |
| `/app/intello/models` | GET | `endpoints.intello.models()` | List available AI models |

## Development vs Production

### Development Setup

```bash
# .env
NEXT_PUBLIC_API_BASE_URL=
BACKEND_URL=http://localhost:8080
```

- Frontend runs on `localhost:3000`
- Backend runs on `localhost:8080`
- Next.js rewrites proxy API requests to backend

### Production Setup

```bash
# .env
NEXT_PUBLIC_API_BASE_URL=
# BACKEND_URL not needed - same origin
```

- Frontend and backend served from same origin
- No proxy needed
- All API calls go to same domain

## Development Proxy

The `next.config.mjs` configures rewrites for development:

```javascript
async rewrites() {
  const backendUrl = process.env.BACKEND_URL || "http://localhost:8080";
  
  return {
    beforeFiles: [
      { source: "/auth/:path*", destination: `${backendUrl}/auth/:path*` },
      { source: "/user/:path*", destination: `${backendUrl}/user/:path*` },
      { source: "/api/:path*", destination: `${backendUrl}/api/:path*` },
      { source: "/app/collection/:path*", destination: `${backendUrl}/app/collection/:path*` },
    ],
  };
}
```

## Adding New Endpoints

1. Add environment variable to `.env`:
   ```bash
   NEXT_PUBLIC_NEW_ENDPOINT=/api/new-endpoint
   ```

2. Update `lib/api/config.ts`:
   ```typescript
   export const endpoints = {
     // ...existing
     newFeature: {
       endpoint: () => buildUrl(process.env.NEXT_PUBLIC_NEW_ENDPOINT, "/api/new-endpoint"),
     },
   };
   ```

3. Update `.env.local.example` for documentation

4. If needed, add rewrite rule in `next.config.mjs`

## Related Documentation

- [Security Configuration](./security-configuration.md)
- [Authentication](./authentication.md)
- [Architecture](./architecture.md)
