export type User = {
  id: string
  username: string
}

export type LoginResponse = {
  access_token: string
  token_type: string
  expires_in: number
  user: User
}

export type ApiErrorBody = {
  error: string
}

export type CreateLinkRequest = {
  url: string
  expires_at?: string
}

export type CreateLinkResponse = {
  id: string
  code: string
  original_url: string
  created_at: string
  expires_at: string | null
}

