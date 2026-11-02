import { Link, useNavigate } from 'react-router-dom'
import { toast } from 'sonner'
import { BrandMark } from '../BrandMark/BrandMark'
import { useAuthStore } from '../../store/authStore'
import styles from './DashboardHeader.module.css'

export function DashboardHeader() {
  const username = useAuthStore((state) => state.user?.username)
  const logout = useAuthStore((state) => state.logout)
  const navigate = useNavigate()

  const handleLogout = async () => {
    try {
      await logout()
      navigate('/', { replace: true })
    } catch {
      toast.error('Could not log out')
      navigate('/', { replace: true })
    }
  }

  return (
    <header className={styles.header}>
      <Link to="/" className={styles.logo}>
        <BrandMark />
      </Link>
      <div className={styles.actions}>
        <span className={styles.username}>{username ?? 'User'}</span>
        <button type="button" className={styles.logout} onClick={() => void handleLogout()}>
          Log out
        </button>
      </div>
    </header>
  )
}
