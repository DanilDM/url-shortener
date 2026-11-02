import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import { Link, useNavigate } from 'react-router-dom'
import { toast } from 'sonner'
import { register as registerUser } from '../../api/auth'
import { getErrorMessage } from '../../api/client'
import { registerSchema, type RegisterFormValues } from '../../lib/validation'
import styles from '../../pages/AuthPage/AuthPage.module.css'

export function RegisterForm() {
  const navigate = useNavigate()
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<RegisterFormValues>({
    resolver: zodResolver(registerSchema),
    defaultValues: {
      username: '',
      password: '',
      repeatPassword: '',
    },
  })

  const onSubmit = handleSubmit(async (values) => {
    try {
      await registerUser(values.username, values.password)
      toast.success('Account created. Please sign in.')
      navigate('/auth/login', { replace: true })
    } catch (error) {
      toast.error(getErrorMessage(error, 'Sign up failed'))
    }
  })

  return (
    <form className={styles.form} onSubmit={onSubmit} noValidate>
      <h1>Sign up</h1>

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
          autoComplete="new-password"
          aria-invalid={Boolean(errors.password)}
          {...register('password')}
        />
        {errors.password ? (
          <span className={styles.fieldError}>{errors.password.message}</span>
        ) : null}
      </label>

      <label>
        Repeat password
        <input
          type="password"
          autoComplete="new-password"
          aria-invalid={Boolean(errors.repeatPassword)}
          {...register('repeatPassword')}
        />
        {errors.repeatPassword ? (
          <span className={styles.fieldError}>{errors.repeatPassword.message}</span>
        ) : null}
      </label>

      <button type="submit" disabled={isSubmitting}>
        {isSubmitting ? 'Signing up...' : 'Sign up'}
      </button>

      <p className={styles.hint}>
        Already have an account? <Link to="/auth/login">Sign in</Link>
      </p>
    </form>
  )
}
