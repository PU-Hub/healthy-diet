import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css' // 確保 Tailwind CSS 被載入
import App from './App.jsx' // 引入您的主程式 (注意這裡的 A 是大寫，需與您的檔名一致)

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
