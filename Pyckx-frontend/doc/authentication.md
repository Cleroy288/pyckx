# Authentication Documentation

## Overview

Pyckx uses session-based authentication with HTTP-only cookies managed by the Rust/Actix-Web backend.

## Access Control

> **Important:** This platform restricts access to pre-authorized users only. If you receive an "User not authorized to access this platform" error (HTTP 403), contact the administrator to be added to the allowed users list.

## Session Management

### Storage
- **Persistence:** Stored in Supabase `sessions` table (migrated from CSV).
- **Caching:** In-memory `HashMap` for fast lookup on every request.
- **Lifetime:** Sessions persist across server restarts.

### Cookie
- Name: `session_id`
- Value: UUIDv4
- Type: HTTP-only (not accessible via JavaScript)
- Security: `Secure` (prod), `SameSite::Lax`
- Set by backend on successful login
- Cleared on logout

### Session Behavior
- **Single Session:** One valid session per user.
- **Reuse:** Login reuses existing valid session if available (prevents session proliferation).
- **Invalidation:** Logout clears session from both memory and Supabase.

### Credentials
All API requests include `credentials: "include"` to send cookies.

## Auth Context

### Provider
```typescript
// lib/auth-context.tsx
import { useAuth } from "@/lib/auth-context"

const {
  user,           // User | null
  isLoading,      // boolean
  isAuthenticated,// boolean
  login,          // (email, password) => Promise
  register,       // (data) => Promise
  logout,         // () => Promise
  checkAuth,      // () => Promise - verify session
} = useAuth()
```

### User Type
```typescript
interface User {
  email: string
  username: string
  role: string
}
```

## API Endpoints

| Method | Endpoint | Body | Response |
|--------|----------|------|----------|
| POST | `/auth/login` | `{ email, password }` | `{ user }` + Set-Cookie |
| POST | `/auth/register` | `{ email, password, username, phone_country_code?, phone_number? }` | `{ user }` + Set-Cookie |
| POST | `/auth/logout` | - | Clear cookie |
| GET | `/user/me` | - | `{ user }` or 401 |

## Auth Guard Component

Protects routes requiring authentication:

```tsx
// components/auth/auth-guard.tsx
import { AuthGuard } from "@/components/auth/auth-guard"

export default function ProtectedPage() {
  return (
    <AuthGuard>
      <YourContent />
    </AuthGuard>
  )
}
```

### Behavior
1. Shows loading spinner while checking auth
2. Redirects to `/login` if not authenticated
3. Renders children if authenticated

## Login Flow

```typescript
// In login page
const { login } = useAuth()

const handleSubmit = async (e) => {
  e.preventDefault()
  try {
    await login(email, password)
    router.push("/") // Redirect to dashboard
  } catch (err) {
    setError(err.message)
  }
}
```

## Registration Flow

```typescript
// In register page
const { register } = useAuth()

const handleSubmit = async (e) => {
  e.preventDefault()
  try {
    await register({
      email,
      password,
      username,
      phone_country_code, // optional
      phone_number,       // optional
    })
    router.push("/") // Auto-logged in after register
  } catch (err) {
    setError(err.message)
  }
}
```

## Error Handling

Backend returns errors in format:
```json
{
  "code": "INVALID_CREDENTIALS",
  "message": "Invalid email or password",
  "field": "email"  // optional
}
```

Common error codes:
- `INVALID_CREDENTIALS` - Wrong email or password
- `AUTH_NOT_ALLOWED` - User not in allowed list (HTTP 403)

Frontend extracts and displays the `message` field.

## Provider Setup

In `app/layout.tsx`:
```tsx
<AuthProvider>
  <ThemeProvider>
    <PaletteProvider>
      {children}
    </PaletteProvider>
  </ThemeProvider>
</AuthProvider>
```

AuthProvider should wrap everything to make auth state available globally.

## Protected vs Public Routes

| Route | Protection |
|-------|------------|
| `/` | AuthGuard |
| `/collection` | AuthGuard |
| `/login` | Public (redirects if logged in) |
| `/register` | Public (redirects if logged in) |
