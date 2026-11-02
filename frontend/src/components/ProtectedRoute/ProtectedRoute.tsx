import { Navigate, Outlet } from 'react-router-dom'
import { useAuthStore } from '../../store/authStore'

export function ProtectedRoute() {
  const token = useAuthStore((state) => state.token)
  const isBootstrapping = useAuthStore((state) => state.isBootstrapping)

  if (isBootstrapping) {
    return <p style={{ padding: 24 }}>Loading...</p>
  }

  if (!token) {
    return <Navigate to="/auth/login" replace />
  }

  return <Outlet />
}
