import React from 'react'
import { Routes, Route } from "react-router-dom";

import HomePage from './pages/Home.js'
import PastePage from './pages/Paste.js'

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/:pasteId" element={<PastePage />} />
    </Routes>
  )
}
