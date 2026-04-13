/** Redis session CRUD */

import Redis from 'ioredis'

import { FALLBACK_TTL_SECS } from '../constants'
import type { SessionUser } from '../types'

const SESSION_PREFIX = 'session:'
const USER_SESSION_PREFIX = 'user_session:'

export class SessionService {
  private redis: Redis

  constructor(redisUrl: string) {
    this.redis = new Redis(redisUrl)
  }

  /** Get user from session */
  async getUser(
    sessionId: string,
  ): Promise<SessionUser | null> {
    const json = await this.redis.get(
      `${SESSION_PREFIX}${sessionId}`,
    )
    if (!json) return null
    return JSON.parse(json) as SessionUser
  }

  /** Create session -- always overwrites (new login = fresh token) */
  async createSession(
    user: SessionUser,
  ): Promise<string> {
    const sessionId = crypto.randomUUID()
    const ttl = this.computeTtl(user)
    await this.storeSession(sessionId, user, ttl)
    return sessionId
  }

  /** Delete session */
  async deleteSession(
    sessionId: string,
  ): Promise<SessionUser | null> {
    const user = await this.getUser(sessionId)
    if (!user) return null

    await this.redis.del(
      `${SESSION_PREFIX}${sessionId}`,
      `${USER_SESSION_PREFIX}${user.id}`,
    )
    return user
  }

  /** Store session + user index with TTL */
  private async storeSession(
    sessionId: string,
    user: SessionUser,
    ttlSecs: number,
  ): Promise<void> {
    const json = JSON.stringify(user)
    const pipeline = this.redis.pipeline()

    pipeline.set(
      `${SESSION_PREFIX}${sessionId}`,
      json,
      'EX',
      ttlSecs,
    )
    pipeline.set(
      `${USER_SESSION_PREFIX}${user.id}`,
      sessionId,
      'EX',
      ttlSecs,
    )

    await pipeline.exec()
  }

  /** Compute TTL from token expiry */
  private computeTtl(user: SessionUser): number {
    const now = Math.floor(Date.now() / 1000)
    const remaining = user.expires_at - now
    return remaining > 0 ? remaining : FALLBACK_TTL_SECS
  }
}
