import { z } from 'zod'

const passwordSchema = z
  .string()
  .min(1, 'Password is required')
  .min(8, 'Password must be at least 8 characters')
  .regex(/[0-9]/, 'Password must contain a digit')
  .regex(/[a-z]/, 'Password must contain a lowercase letter')
  .regex(/[A-Z]/, 'Password must contain an uppercase letter')

export const loginSchema = z.object({
  username: z.string().min(1, 'Username is required'),
  password: z.string().min(1, 'Password is required'),
})

export const registerSchema = z
  .object({
    username: z.string().min(1, 'Username is required'),
    password: passwordSchema,
    repeatPassword: z.string().min(1, 'Repeat password is required'),
  })
  .refine((values) => values.password === values.repeatPassword, {
    message: 'Passwords do not match',
    path: ['repeatPassword'],
  })

export type LoginFormValues = z.infer<typeof loginSchema>
export type RegisterFormValues = z.infer<typeof registerSchema>
