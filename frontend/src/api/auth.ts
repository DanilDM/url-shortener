import { api } from './client'
import type { LoginResponse, User } from './types'

export function register(username: string, password: string) {
  return api.post<User>('/auth/register', { username, password })
}

export function login(username: string, password: string) {
  return api.post<LoginResponse>('/auth/login', { username, password })
}

export function getMe() {
  return api.get<User>('/auth/me')
}

export function logout() {
  return api.post('/auth/logout')
}
