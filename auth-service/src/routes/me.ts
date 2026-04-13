/** GET /api/user/me */

import { Hono } from 'hono'
import { getCookie } from 'hono/cookie'

import { SESSION_COOKIE_NAME } from '../constants'
import { SessionService } from '../services/session'
import { toAuthResponse } from '../types'

export function meRoute(sessions: SessionService) {
  const app = new Hono()

  app.get('/me', async (c) => {
    const sessionId = getCookie(c, SESSION_COOKIE_NAME)
    if (!sessionId) return c.body(null, 401)

    const user = await sessions.getUser(sessionId)
    if (!user) return c.body(null, 401)

    return c.json(toAuthResponse(user), 200)
  })

  return app
}
