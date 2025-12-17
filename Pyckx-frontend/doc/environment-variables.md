# Environment Variables

Complete reference for all environment variables used in the PYCKX frontend application.

## Quick Reference

| Variable | Required | Default | Exposed to Browser |
|----------|----------|---------|-------------------|
| `NEXT_PUBLIC_API_BASE_URL` | No | `""` | ✅ Yes |
| `NEXT_PUBLIC_AUTH_*` | No | See defaults | ✅ Yes |
| `NEXT_PUBLIC_USER_*` | No | See defaults | ✅ Yes |
| `NEXT_PUBLIC_APPS_*` | No | See defaults | ✅ Yes |
| `NEXT_PUBLIC_COLLECTION_*` | No | See defaults | ✅ Yes |
| `BACKEND_URL` | No | `http://localhost:8080` | ❌ No |
| `SECURITY_MODE` | No | `development` | ❌ No |

## Files

- `.env` - Default environment variables (committed to repo)
- `.env.local` - Local overrides (not committed, gitignored)
- `.env.local.example` - Template for `.env.local`

## Variable Details

### API Base URL

```bash
NEXT_PUBLIC_API_BASE_URL=
```

- **Purpose:** Prefix for all API endpoint URLs
- **Default:** Empty string (same-origin)
- **Development:** Leave empty (proxy handles routing)
- **Production:** Leave empty (same-origin deployment)

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
NEXT_PUBLIC_COLLECTION_GET_DVD=/app/collection/dvd/{id}
NEXT_PUBLIC_COLLECTION_UPDATE_DVD=/app/collection/mod-dvd/{id}
NEXT_PUBLIC_COLLECTION_DELETE_DVD=/app/collection/del/{id}
```

**Note:** Dynamic endpoints use `{id}` placeholder, replaced at runtime.

### Backend URL (Development Only)

```bash
BACKEND_URL=http://localhost:8080
```

- **Purpose:** Target URL for development proxy rewrites
- **Used by:** `next.config.mjs` rewrites
- **Not exposed to browser** (server-side only)

### Security Mode

```bash
SECURITY_MODE=development
```

- **Purpose:** Controls security header behavior
- **Values:**
  - `development` - No HSTS, safe for local dev
  - `production` - Enables HSTS (requires HTTPS)
- **Used by:** `next.config.mjs` headers

## Environment Precedence

Next.js loads environment variables in this order (later overrides earlier):

1. `.env` (default values)
2. `.env.local` (local overrides)
3. `.env.development` / `.env.production` (mode-specific)
4. `.env.development.local` / `.env.production.local` (mode-specific local)

## Browser Exposure

Variables prefixed with `NEXT_PUBLIC_` are:
- ✅ Available in browser JavaScript
- ✅ Bundled into client-side code
- ⚠️ Visible to end users (don't put secrets here!)

Variables without `NEXT_PUBLIC_` prefix are:
- ❌ Not available in browser
- ✅ Only available server-side (API routes, `next.config.mjs`)
- ✅ Safe for sensitive configuration

## Example Configurations

### Local Development

```bash
# .env.local
NEXT_PUBLIC_API_BASE_URL=
BACKEND_URL=http://localhost:8080
SECURITY_MODE=development
```

### Production (Same Origin)

```bash
# .env.production.local
NEXT_PUBLIC_API_BASE_URL=
SECURITY_MODE=production
```

### Production (Separate API Domain)

```bash
# .env.production.local
NEXT_PUBLIC_API_BASE_URL=https://api.example.com
SECURITY_MODE=production
```

## Troubleshooting

### Variables Not Loading

1. Restart the dev server after changing `.env` files
2. Ensure variable names are spelled correctly
3. Check that `.env.local` exists (copy from `.env.local.example`)

### Browser Can't Access Variable

1. Ensure variable has `NEXT_PUBLIC_` prefix
2. Rebuild the application (`bun run build`)
3. Clear browser cache

### Server-Side Variable Undefined

1. Check variable is defined in `.env` or `.env.local`
2. Ensure no `NEXT_PUBLIC_` prefix for server-only vars
3. Verify file is in project root (not in `src/`)

## Related Documentation

- [API Configuration](./api-configuration.md)
- [Security Configuration](./security-configuration.md)
