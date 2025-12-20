# API Configuration

All API endpoints are defined in `lib/api/config.ts`.

## Endpoint Pattern

All routes follow: `/api/{service}/{feature}/{id}`

## Usage

```typescript
import { endpoints } from "@/lib/api/config";

// Static endpoints (plain strings)
fetch(endpoints.auth.login, { method: "POST", ... })
fetch(endpoints.user.me, { credentials: "include" })
fetch(endpoints.collection.dvds, { method: "GET" })

// Dynamic endpoints (functions that take ID)
fetch(endpoints.collection.dvd(id), { method: "DELETE" })
fetch(endpoints.intello.qcmById(id), { method: "GET" })
```

## Endpoint Reference

| Service | Endpoint | Type |
|---------|----------|------|
| **Auth** | `endpoints.auth.login` | string |
| **Auth** | `endpoints.auth.logout` | string |
| **User** | `endpoints.user.me` | string |
| **Apps** | `endpoints.apps.list` | string |
| **Apps** | `endpoints.apps.userApps` | string |
| **Collection** | `endpoints.collection.dvds` | string |
| **Collection** | `endpoints.collection.dvd(id)` | function |
| **Intello** | `endpoints.intello.qcm` | string |
| **Intello** | `endpoints.intello.qcmById(id)` | function |
| **Intello** | `endpoints.intello.models` | string |
| **Intello** | `endpoints.intello.flashcards` | string |
| **Intello** | `endpoints.intello.keywords` | string |
| **Intello** | `endpoints.intello.trueFalse` | string |
| **Intello** | `endpoints.intello.orderPhrases` | string |
| **Intello** | `endpoints.intello.fillBlanks` | string |

## Adding New Endpoints

1. Add to `lib/api/config.ts`:
   ```typescript
   export const endpoints = {
     myService: {
       list: "/api/my-service",              // static
       byId: (id: string) => `/api/my-service/${id}`,  // dynamic
     },
   } as const;
   ```

2. Use in components:
   ```typescript
   const res = await fetch(endpoints.myService.list);
   const item = await fetch(endpoints.myService.byId("123"));
   ```

## Related Docs

- [Architecture](./architecture.md)
- [Authentication](./authentication.md)


