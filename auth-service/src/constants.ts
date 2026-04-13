/** Shared constants for auth-service */

import type { Config } from './config'

/** Cookie name for session identifier */
export const SESSION_COOKIE_NAME = 'session_id'

/** Session fallback TTL when token expiry is in the past */
export const FALLBACK_TTL_SECS = 3600

/** Validation limits */
export const MAX_EMAIL_LENGTH = 255
export const MIN_PASSWORD_LENGTH = 6
export const MAX_PASSWORD_LENGTH = 128

/** Config defaults */
export const DEFAULT_PORT = '3001'
export const DEFAULT_REDIS_URL = 'redis://127.0.0.1:6379'
export const SECURE_HTTP_VALUE = 'true'

/** Error messages */
export const ERR_INVALID_CREDENTIALS = 'Invalid email or password'
export const ERR_AUTH_FAILED = 'Invalid credentials'

/** Response messages */
export const MSG_LOGGED_OUT = 'Logged out successfully'

/** Build cookie options for session cookie */
export function sessionCookieOptions(cfg: Config) {
  return {
    httpOnly: true,
    secure: cfg.secureHttp,
    sameSite: 'Lax' as const,
    path: '/',
  }
}
