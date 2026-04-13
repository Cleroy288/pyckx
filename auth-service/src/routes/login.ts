/** POST /api/auth/login */

import { Hono } from 'hono'
import { setCookie } from 'hono/cookie'

import type { Config } from '../config'
import {
  ERR_AUTH_FAILED,
  ERR_INVALID_CREDENTIALS,
  SESSION_COOKIE_NAME,
  sessionCookieOptions,
} from '../constants'
import { SessionService } from '../services/session'
import { supabaseLogin } from '../services/supabase'
import { toAuthResponse, toSessionUser } from '../types'
import { loginSchema } from '../validation'

export function loginRoute(
  cfg: Config,
  sessions: SessionService,
) {
  const app = new Hono()

  app.post('/login', async (c) => {
    const body = await parseBody(c)
    if (!body) {
      return c.json(
        { error: ERR_INVALID_CREDENTIALS },
        400,
      )
    }

    const parsed = loginSchema.safeParse(body)
    if (!parsed.success) {
      return c.json(
        { error: ERR_INVALID_CREDENTIALS },
        400,
      )
    }

    try {
      return await handleLogin(c, cfg, sessions, parsed.data)
    } catch (error) {
      console.error('Login failed:', error)
      return c.json({ error: ERR_AUTH_FAILED }, 401)
    }
  })

  return app
}

async function parseBody(c: any): Promise<unknown | null> {
  try {
    return await c.req.json()
  } catch {
    return null
  }
}

async function handleLogin(
  c: any,
  cfg: Config,
  sessions: SessionService,
  credentials: { email: string; password: string },
) {
  const supabaseRes = await supabaseLogin(
    cfg,
    credentials.email,
    credentials.password,
  )

  const user = toSessionUser(supabaseRes)
  const sessionId = await sessions.createSession(user)

  setCookie(
    c,
    SESSION_COOKIE_NAME,
    sessionId,
    sessionCookieOptions(cfg),
  )

  return c.json(toAuthResponse(user), 200)
}
