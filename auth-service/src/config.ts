/** Auth-service configuration from environment */

import {
  DEFAULT_PORT,
  DEFAULT_REDIS_URL,
  SECURE_HTTP_VALUE,
} from './constants'

export interface Config {
  port: number
  redisUrl: string
  supabaseUrl: string
  supabaseAnonKey: string
  secureHttp: boolean
}

export function loadConfig(): Config {
  const port = Number(Bun.env.AUTH_PORT ?? DEFAULT_PORT)
  const redisUrl = Bun.env.REDIS_URL ?? DEFAULT_REDIS_URL
  const supabaseUrl = requireEnv('SP_URL')
  const supabaseAnonKey = requireEnv('SP_ANON')
  const secureHttp = Bun.env.SECURE_HTTP === SECURE_HTTP_VALUE

  return {
    port,
    redisUrl,
    supabaseUrl,
    supabaseAnonKey,
    secureHttp,
  }
}

function requireEnv(key: string): string {
  const value = Bun.env[key]
  if (!value) throw new Error(`Missing env var: ${key}`)
  return value
}
