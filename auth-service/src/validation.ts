/** Request validation schemas */

import { z } from 'zod'

import {
  MAX_EMAIL_LENGTH,
  MAX_PASSWORD_LENGTH,
  MIN_PASSWORD_LENGTH,
} from './constants'

export const loginSchema = z.object({
  email: z.string().email().max(MAX_EMAIL_LENGTH),
  password: z
    .string()
    .min(MIN_PASSWORD_LENGTH)
    .max(MAX_PASSWORD_LENGTH),
})

export type LoginBody = z.infer<typeof loginSchema>
