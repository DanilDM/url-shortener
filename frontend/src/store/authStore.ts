import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import * as authApi from '../api/auth'
import { getErrorMessage } from '../api/client'
import type { User } from '../api/types'

type AuthState = {
  token: string | null
  user: User | null
  isBootstrapping: boolean
  setSession: (token: string, user: User) => void
  clearSession: () => void
  bootstrap: () => Promise<void>
  logout: () => Promise<void>
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      token: null,
      user: null,
      isBootstrapping: true,

      setSession: (token, user) => {
        set({ token, user })
      },

      clearSession: () => {
        set({ token: null, user: null })
      },

      bootstrap: async () => {
        const token = get().token
        if (!token) {
          set({ isBootstrapping: false, user: null })
          return
        }

        try {
          const { data } = await authApi.getMe()
          set({ user: data, isBootstrapping: false })
        } catch {
          set({ token: null, user: null, isBootstrapping: false })
        }
      },

      logout: async () => {
        try {
          if (get().token) {
            await authApi.logout()
          }
        } catch (error) {
          // Still clear local session even if request fails.
          console.error(getErrorMessage(error, 'Logout failed'))
        } finally {
          get().clearSession()
        }
      },
    }),
    {
      name: 'url-shortener-auth',
      partialize: (state) => ({
        token: state.token,
        user: state.user,
      }),
    },
  ),
)
