export function buildShortUrl(code: string) {
  const apiBase = import.meta.env.VITE_API_URL ?? 'http://localhost:3000/api/v1'

  try {
    const origin = new URL(apiBase).origin
    return `${origin}/${code}`
  } catch {
    return `/${code}`
  }
}
