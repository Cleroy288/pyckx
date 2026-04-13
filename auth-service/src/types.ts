/** Shared types for auth-service */

/** User stored in Redis session (matches Rust User struct) */
export interface SessionUser {
  id: string
  email: string
  username: string
  role: string
  access_token: string
  refresh_token: string
  expires_at: number
}

/** Response sent to frontend (no tokens) */
export interface AuthResponse {
  username: string
  email: string
  role: string
}

/** Supabase auth response shape */
export interface SupabaseAuthResponse {
  access_token: string
  refresh_token: string
  expires_at: number
  user: {
    id: string
    email: string
    user_metadata: {
      username?: string
    }
    role: string
  }
}

/** Convert Supabase response to session user */
export function toSessionUser(
  res: SupabaseAuthResponse,
): SessionUser {
  return {
    id: res.user.id,
    email: res.user.email ?? '',
    username: res.user.user_metadata?.username ?? '',
    role: res.user.role ?? 'user',
    access_token: res.access_token,
    refresh_token: res.refresh_token,
    expires_at: res.expires_at,
  }
}

/** Convert session user to auth response (safe for frontend) */
export function toAuthResponse(
  user: SessionUser,
): AuthResponse {
  return {
    username: user.username,
    email: user.email,
    role: user.role,
  }
}
