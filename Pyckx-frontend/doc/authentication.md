# Authentication Documentation

## Overview

Pyckx uses session-based authentication with HTTP-only cookies managed by the Rust/Actix-Web backend.

## Session Management

### Cookie
- Name: `session_id`
- Type: HTTP-only (not accessible via JavaScript)
- Set by backend on successful login
- Cleared on logout

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
// components/auth-guard.tsx
import { AuthGuard } from "@/components/auth-guard"

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
