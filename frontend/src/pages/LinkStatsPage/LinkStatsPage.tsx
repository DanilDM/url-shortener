import { Link, useParams } from 'react-router-dom'
import { DashboardHeader } from '../../components/DashboardHeader/DashboardHeader'
import styles from './LinkStatsPage.module.css'

export function LinkStatsPage() {
  const { shortCode } = useParams<{ shortCode: string }>()

  return (
    <div className={styles.page}>
      <DashboardHeader />
      <main className={styles.main}>
        <Link to="/dashboard" className={styles.back}>
          Back to Dashboard
        </Link>
        <section className={styles.card}>
          <h1>Link statistics</h1>
          <p className={styles.meta}>
            shortCode: <code>{shortCode}</code>
          </p>
          <p className={styles.placeholder}>
            Detailed link stats will appear here.
          </p>
        </section>
      </main>
    </div>
  )
}
