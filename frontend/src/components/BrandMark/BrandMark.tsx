import styles from './BrandMark.module.css'

type BrandMarkProps = {
  className?: string
  showText?: boolean
}

export function BrandMark({ className, showText = true }: BrandMarkProps) {
  return (
    <span className={[styles.root, className].filter(Boolean).join(' ')}>
      <svg
        className={styles.mark}
        viewBox="0 0 32 32"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true"
      >
        <rect width="32" height="32" rx="8" fill="#5B5FEF" />
        <g
          fill="none"
          stroke="#fff"
          strokeWidth="2.25"
          strokeLinecap="round"
          strokeLinejoin="round"
          transform="translate(4 4)"
        >
          <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
          <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
        </g>
      </svg>
      {showText ? <span className={styles.text}>URL Shortener</span> : null}
    </span>
  )
}
