import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(), // 加入這一行
  ],
  server: {
    port: 8080,
    strictPort: true,
    host: true,
  }
})
