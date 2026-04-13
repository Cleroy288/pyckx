/** Supabase auth HTTP client */

import type { Config } from '../config'
import type { SupabaseAuthResponse } from '../types'

const LOGIN_PATH = '/auth/v1/token?grant_type=password'
const LOGOUT_PATH = '/auth/v1/logout'

/** Login with email/password via Supabase Auth API */
export async function supabaseLogin(
  cfg: Config,
  email: string,
  password: string,
): Promise<SupabaseAuthResponse> {
  const url = `${cfg.supabaseUrl}${LOGIN_PATH}`

  const res = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      apikey: cfg.supabaseAnonKey,
    },
    body: JSON.stringify({ email, password }),
  })

  if (!res.ok) {
    const body = await res.text()
    throw new Error(
      `Supabase login failed (${res.status}): ${body}`,
    )
  }

  return res.json() as Promise<SupabaseAuthResponse>
}

/** Logout (invalidate token) via Supabase -- best effort */
export async function supabaseLogout(
  cfg: Config,
  accessToken: string,
): Promise<void> {
  const url = `${cfg.supabaseUrl}${LOGOUT_PATH}`

  try {
    await fetch(url, {
      method: 'POST',
      headers: {
        apikey: cfg.supabaseAnonKey,
        Authorization: `Bearer ${accessToken}`,
      },
    })
  } catch (error) {
    console.error(
      'Supabase logout failed (best-effort):',
      error,
    )
  }
}
