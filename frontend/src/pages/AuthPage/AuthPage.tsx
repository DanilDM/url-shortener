import { Link, Navigate, useParams } from 'react-router-dom'
import { BrandMark } from '../../components/BrandMark/BrandMark'
import { LoginForm } from '../../components/LoginForm/LoginForm'
import { RegisterForm } from '../../components/RegisterForm/RegisterForm'
import { useAuthStore } from '../../store/authStore'
import styles from './AuthPage.module.css'

type AuthType = 'login' | 'register'

function isAuthType(value: string | undefined): value is AuthType {
  return value === 'login' || value === 'register'
}

export function AuthPage() {
  const { type } = useParams<{ type: string }>()
  const token = useAuthStore((state) => state.token)
  const isBootstrapping = useAuthStore((state) => state.isBootstrapping)

  if (!isAuthType(type)) {
    return <Navigate to="/auth/login" replace />
  }

  if (!isBootstrapping && token) {
    return <Navigate to="/dashboard" replace />
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

        {isLogin ? <LoginForm /> : <RegisterForm />}
      </div>
    </div>
  )
}
