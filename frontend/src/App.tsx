import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom'
import { Toaster } from 'sonner'
import { AuthBootstrap } from './components/AuthBootstrap/AuthBootstrap'
import { ProtectedRoute } from './components/ProtectedRoute/ProtectedRoute'
import { AuthPage } from './pages/AuthPage/AuthPage'
import { DashboardPage } from './pages/DashboardPage/DashboardPage'
import { LandingPage } from './pages/LandingPage/LandingPage'
import { LinkStatsPage } from './pages/LinkStatsPage/LinkStatsPage'

export default function App() {
  return (
    <AuthBootstrap>
      <BrowserRouter>
        <Toaster position="top-right" richColors closeButton />
        <Routes>
          <Route path="/" element={<LandingPage />} />
          <Route path="/auth/:type" element={<AuthPage />} />
          <Route element={<ProtectedRoute />}>
            <Route path="/dashboard" element={<DashboardPage />} />
            <Route path="/dashboard/:shortCode" element={<LinkStatsPage />} />
          </Route>
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
    </AuthBootstrap>
  )
}
