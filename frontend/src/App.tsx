import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom'
import { AuthPage } from './pages/AuthPage/AuthPage'
import { DashboardPage } from './pages/DashboardPage/DashboardPage'
import { LandingPage } from './pages/LandingPage/LandingPage'
import { LinkStatsPage } from './pages/LinkStatsPage/LinkStatsPage'

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/auth/:type" element={<AuthPage />} />
        <Route path="/dashboard" element={<DashboardPage />} />
        <Route path="/dashboard/:shortCode" element={<LinkStatsPage />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  )
}
