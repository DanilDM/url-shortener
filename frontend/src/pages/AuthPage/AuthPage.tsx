import { Link, Navigate, useParams } from 'react-router-dom'
import { BrandMark } from '../../components/BrandMark/BrandMark'
import styles from './AuthPage.module.css'

type AuthType = 'login' | 'register'

function isAuthType(value: string | undefined): value is AuthType {
  return value === 'login' || value === 'register'
}

export function AuthPage() {
  const { type } = useParams<{ type: string }>()

  if (!isAuthType(type)) {
    return <Navigate to="/auth/login" replace />
  }

  const isLogin = type === 'login'

  return (
    <div className={styles.page}>
      <Link to="/" className={styles.logo}>
        <BrandMark />
      </Link>

      <div className={styles.card}>
        <div className={styles.tabs} role="tablist">
          <Link
            to="/auth/login"
            role="tab"
            aria-selected={isLogin}
            className={isLogin ? styles.tabActive : styles.tab}
          >
            Sign in
          </Link>
          <Link
            to="/auth/register"
            role="tab"
            aria-selected={!isLogin}
            className={!isLogin ? styles.tabActive : styles.tab}
          >
            Sign up
          </Link>
        </div>

        {isLogin ? (
          <form className={styles.form} onSubmit={(event) => event.preventDefault()}>
            <h1>Sign in</h1>
            <label>
              Username
              <input name="username" type="text" autoComplete="username" />
            </label>
            <label>
              Password
              <input name="password" type="password" autoComplete="current-password" />
            </label>
            <button type="submit">Sign in</button>
            <p className={styles.hint}>
              No account? <Link to="/auth/register">Sign up</Link>
            </p>
          </form>
        ) : (
          <form className={styles.form} onSubmit={(event) => event.preventDefault()}>
            <h1>Sign up</h1>
            <label>
              Username
              <input name="username" type="text" autoComplete="username" />
            </label>
            <label>
              Password
              <input name="password" type="password" autoComplete="new-password" />
            </label>
            <label>
              Repeat password
              <input name="repeatPassword" type="password" autoComplete="new-password" />
            </label>
            <button type="submit">Sign up</button>
            <p className={styles.hint}>
              Already have an account? <Link to="/auth/login">Sign in</Link>
            </p>
          </form>
        )}
      </div>
    </div>
  )
}
