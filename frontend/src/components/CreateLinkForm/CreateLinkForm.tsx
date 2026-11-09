import { zodResolver } from '@hookform/resolvers/zod'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { toast } from 'sonner'
import { createLink } from '../../api/urls'
import { getErrorMessage } from '../../api/client'
import type { CreateLinkResponse } from '../../api/types'
import { buildShortUrl } from '../../lib/shortUrl'
import { createLinkSchema, type CreateLinkFormValues } from '../../lib/validation'
import styles from './CreateLinkForm.module.css'

function toIsoExpiresAt(value?: string) {
  if (!value) return undefined
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return undefined
  return date.toISOString()
}

export function CreateLinkForm() {
  const [created, setCreated] = useState<CreateLinkResponse | null>(null)
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<CreateLinkFormValues>({
    resolver: zodResolver(createLinkSchema),
    defaultValues: {
      url: '',
      expiresAt: '',
    },
  })

  const onSubmit = handleSubmit(async (values) => {
    try {
      const payload = {
        url: values.url.trim(),
        ...(values.expiresAt
          ? { expires_at: toIsoExpiresAt(values.expiresAt) }
          : {}),
      }
      const { data } = await createLink(payload)
      setCreated(data)
      reset({ url: '', expiresAt: '' })
      toast.success('Short link created')
    } catch (error) {
      toast.error(getErrorMessage(error, 'Could not create short link'))
    }
  })

  const handleCopy = async () => {
    if (!created) return
    const shortUrl = buildShortUrl(created.code)
    try {
      await navigator.clipboard.writeText(shortUrl)
      toast.success('Copied to clipboard')
    } catch {
      toast.error('Could not copy link')
    }
  }

  return (
    <div>
      <form className={styles.form} onSubmit={onSubmit} noValidate>
        <div className={styles.row}>
          <label className={styles.label}>
            Original URL
            <input
              className={styles.input}
              type="url"
              placeholder="https://example.com/very/long/path"
              aria-invalid={Boolean(errors.url)}
              {...register('url')}
            />
            {errors.url ? (
              <span className={styles.fieldError}>{errors.url.message}</span>
            ) : null}
          </label>

          <button className={styles.submit} type="submit" disabled={isSubmitting}>
            {isSubmitting ? 'Shortening...' : 'Shorten'}
          </button>
        </div>

        <label className={styles.label}>
          Expires at (optional)
          <input className={styles.input} type="datetime-local" {...register('expiresAt')} />
          {errors.expiresAt ? (
            <span className={styles.fieldError}>{errors.expiresAt.message}</span>
          ) : null}
        </label>
      </form>

      {created ? (
        <div className={styles.result}>
          <p className={styles.resultTitle}>New short link</p>
          <div className={styles.resultRow}>
            <span className={styles.shortUrl}>{buildShortUrl(created.code)}</span>
            <button className={styles.copy} type="button" onClick={() => void handleCopy()}>
              Copy
            </button>
          </div>
          <p className={styles.meta}>Original: {created.original_url}</p>
          {created.expires_at ? (
            <p className={styles.meta}>
              Expires: {new Date(created.expires_at).toLocaleString()}
            </p>
          ) : (
            <p className={styles.meta}>No expiration</p>
          )}
        </div>
      ) : null}
    </div>
  )
}
