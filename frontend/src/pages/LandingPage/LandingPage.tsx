import { Link } from 'react-router-dom'
import { BrandMark } from '../../components/BrandMark/BrandMark'
import { useAuthStore } from '../../store/authStore'
import styles from './LandingPage.module.css'

const features = [
  {
    title: 'Create short links',
    text: 'Turn long URLs into compact links that are easy to share.',
  },
  {
    title: 'Track click stats',
    text: 'See how many times each short link was opened.',
  },
  {
    title: 'Manage active links',
    text: 'Keep your links organized and remove the ones you no longer need.',
  },
]

export function LandingPage() {
  const token = useAuthStore((state) => state.token)
  const isLoggedIn = Boolean(token)

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <Link to="/" className={styles.logo}>
          <BrandMark />
        </Link>
        <nav className={styles.nav}>
          {isLoggedIn ? (
            <Link to="/dashboard" className={styles.primaryLink}>
              Dashboard
            </Link>
          ) : (
            <>
              <Link to="/auth/login">Sign in</Link>
              <Link to="/auth/register" className={styles.primaryLink}>
                Sign up
              </Link>
            </>
          )}
        </nav>
      </header>

      <main>
        <section className={styles.hero}>
          <div className={styles.heroContent}>
            <p className={styles.eyebrow}>URL Shortener</p>
            <h1>Long links, short and trackable.</h1>
            <p>
              Create short URLs in seconds, share them with anyone, and follow
              click statistics from your dashboard.
            </p>
            <div className={styles.ctaGroup}>
              {isLoggedIn ? (
                <Link to="/dashboard" className={styles.cta}>
                  Open dashboard
                </Link>
              ) : (
                <Link to="/auth/register" className={styles.cta}>
                  Get started
                </Link>
              )}
              {!isLoggedIn ? (
                <Link to="/auth/login" className={styles.secondaryCta}>
                  Sign in
                </Link>
              ) : null}
            </div>
          </div>
        </section>

        <section className={styles.features}>
          <h2>Features</h2>
          <p className={styles.featuresLead}>
            Everything you need to shorten links and keep them under control.
          </p>
          <ul className={styles.featureGrid}>
            {features.map((feature) => (
              <li key={feature.title} className={styles.featureCard}>
                <h3>{feature.title}</h3>
                <p>{feature.text}</p>
              </li>
            ))}
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
