import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { Link, useNavigate } from 'react-router-dom'
import { toast } from 'sonner'
import { login } from '../../api/auth'
import { getErrorMessage } from '../../api/client'
import { loginSchema, type LoginFormValues } from '../../lib/validation'
import { useAuthStore } from '../../store/authStore'
import styles from '../../pages/AuthPage/AuthPage.module.css'

export function LoginForm() {
  const navigate = useNavigate()
  const setSession = useAuthStore((state) => state.setSession)
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      username: '',
      password: '',
    },
  })

  const onSubmit = handleSubmit(async (values) => {
    try {
      const { data } = await login(values.username, values.password)
      setSession(data.access_token, data.user)
      navigate('/dashboard', { replace: true })
    } catch (error) {
      toast.error(getErrorMessage(error, 'Sign in failed'))
    }
  })

  return (
    <form className={styles.form} onSubmit={onSubmit} noValidate>
      <h1>Sign in</h1>

      <label>
        Username
        <input
          type="text"
          autoComplete="username"
          aria-invalid={Boolean(errors.username)}
          {...register('username')}
        />
        {errors.username ? (
          <span className={styles.fieldError}>{errors.username.message}</span>
        ) : null}
      </label>

      <label>
        Password
        <input
          type="password"
          autoComplete="current-password"
          aria-invalid={Boolean(errors.password)}
          {...register('password')}
        />
        {errors.password ? (
          <span className={styles.fieldError}>{errors.password.message}</span>
        ) : null}
      </label>

      <button type="submit" disabled={isSubmitting}>
        {isSubmitting ? 'Signing in...' : 'Sign in'}
      </button>

      <p className={styles.hint}>
        No account? <Link to="/auth/register">Sign up</Link>
      </p>
    </form>
  )
}
