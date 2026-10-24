import { DashboardHeader } from '../../components/DashboardHeader/DashboardHeader'
import styles from './DashboardPage.module.css'

export function DashboardPage() {
  return (
    <div className={styles.page}>
      <DashboardHeader />
      <main className={styles.main}>
        <section className={styles.section}>
          <h1>Create short link</h1>
          <p className={styles.placeholder}>CreateLinkForm — coming next</p>
        </section>
        <section className={styles.section}>
          <h2>Your links</h2>
          <p className={styles.placeholder}>LinksList — coming next</p>
        </section>
      </main>
    </div>
  )
}
