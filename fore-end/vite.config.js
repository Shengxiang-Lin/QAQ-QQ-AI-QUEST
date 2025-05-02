import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'


import fs from 'fs/promises';
import path from 'path';
async function loadJS() {
  const configPath = path.resolve('../config.json');
  const data = await fs.readFile(configPath, 'utf-8');
  return JSON.parse(data);
}

const parameters = await loadJS();
let port = parameters.vue_port.value
let host_port = parameters.rust_port.value
// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: `@use "@/assets/main.scss" as *;`,
      },
    },
  },
  server: {
    port: port, 
  },
  define:{
    __HOST_PORT__: host_port,
  }
})
