/** POST /api/auth/logout */

import { Hono } from 'hono'
import { getCookie, setCookie } from 'hono/cookie'

import type { Config } from '../config'
import {
  MSG_LOGGED_OUT,
  SESSION_COOKIE_NAME,
  sessionCookieOptions,
} from '../constants'
import { SessionService } from '../services/session'
import { supabaseLogout } from '../services/supabase'

export function logoutRoute(
  cfg: Config,
  sessions: SessionService,
) {
  const app = new Hono()

  app.post('/logout', async (c) => {
    const sessionId = getCookie(c, SESSION_COOKIE_NAME)

    if (sessionId) {
      const user = await sessions.deleteSession(sessionId)
      if (user) {
        await supabaseLogout(cfg, user.access_token)
      }
    }

    setCookie(c, SESSION_COOKIE_NAME, '', {
      ...sessionCookieOptions(cfg),
      maxAge: 0,
    })

    return c.json({ message: MSG_LOGGED_OUT }, 200)
  })

  return app
}
