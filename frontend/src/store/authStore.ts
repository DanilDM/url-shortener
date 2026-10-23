import { create } from 'zustand'
import { persist } from 'zustand/middleware'

type AuthState = {
  username: string | null
  setUsername: (username: string | null) => void
  logout: () => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      username: null,
      setUsername: (username) => set({ username }),
      logout: () => set({ username: null }),
    }),
    { name: 'url-shortener-auth' },
  ),
)
