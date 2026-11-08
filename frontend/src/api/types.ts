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
