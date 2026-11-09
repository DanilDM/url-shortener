import { api } from './client'
import type { CreateLinkRequest, CreateLinkResponse } from './types'

export function createLink(payload: CreateLinkRequest) {
  return api.post<CreateLinkResponse>('/urls/new', payload)
}
