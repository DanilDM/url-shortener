import { Link } from 'react-router-dom'
import { BrandMark } from '../../components/BrandMark/BrandMark'
import styles from './LandingPage.module.css'

export function LandingPage() {
  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <Link to="/" className={styles.logo}>
          <BrandMark />
        </Link>
        <nav className={styles.nav}>
          <Link to="/auth/login">Sign in</Link>
          <Link to="/auth/register" className={styles.primaryLink}>
            Sign up
          </Link>
        </nav>
      </header>

      <main>
        <section className={styles.hero}>
          <h1>Short links. Clear stats.</h1>
          <p>
            Create short URLs, share them with anyone, and track how many times
            they are opened.
          </p>
          <Link to="/auth/register" className={styles.cta}>
            Get started
          </Link>
        </section>

        <section className={styles.features}>
          <h2>Features</h2>
          <ul>
            <li>Create short links from long URLs</li>
            <li>Track click statistics</li>
            <li>Manage active links in one place</li>
          </ul>
        </section>
      </main>

      <footer className={styles.footer}>
        <BrandMark />
        <p>Web service for shortening URLs and tracking clicks.</p>
      </footer>
    </div>
  )
}
