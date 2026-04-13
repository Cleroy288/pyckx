/** Auth microservice -- Hono on Bun */

import { Hono } from 'hono'

import { loadConfig } from './config'
import { loginRoute } from './routes/login'
import { logoutRoute } from './routes/logout'
import { meRoute } from './routes/me'
import { SessionService } from './services/session'

const cfg = loadConfig()
const sessions = new SessionService(cfg.redisUrl)

const app = new Hono()

app.route('/api/auth', loginRoute(cfg, sessions))
app.route('/api/auth', logoutRoute(cfg, sessions))
app.route('/api/user', meRoute(sessions))
app.get('/health', (c) => c.json({ status: 'ok' }))

console.log(`Auth service running on port ${cfg.port}`)

export default {
  port: cfg.port,
  fetch: app.fetch,
}
