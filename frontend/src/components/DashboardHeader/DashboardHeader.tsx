import { Link, useNavigate } from 'react-router-dom'
import { BrandMark } from '../BrandMark/BrandMark'
import { useAuthStore } from '../../store/authStore'
import styles from './DashboardHeader.module.css'

export function DashboardHeader() {
  const username = useAuthStore((state) => state.username)
  const logout = useAuthStore((state) => state.logout)
  const navigate = useNavigate()

  const handleLogout = () => {
    logout()
    localStorage.removeItem('url-shortener-auth')
    navigate('/')
  }

  return (
    <header className={styles.header}>
      <Link to="/" className={styles.logo}>
        <BrandMark />
      </Link>
      <div className={styles.actions}>
        <span className={styles.username}>{username ?? 'User'}</span>
        <button type="button" className={styles.logout} onClick={handleLogout}>
          Log out
        </button>
      </div>
    </header>
  )
}
