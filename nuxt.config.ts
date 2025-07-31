import tailwindcss from '@tailwindcss/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    '@nuxt/fonts',
    '@nuxt/icon',
    '@nuxt/eslint',
    '@pinia/nuxt',
    '@vueuse/nuxt',
  ],
  ssr: false,
  imports: {
    presets: [{
      from: '@faker-js/faker',
      imports: ['faker'],
    }, {
      from: '@vueuse/core',
      imports: ['useImage'],
    }, {
      from: 'tailwind-variants',
      imports: ['tv'],
    }],
    dirs: [
      'services',
    ],
  },
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  ignore: ['**/src-tauri/**'],
  compatibilityDate: '2025-07-15',
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
    },
    plugins: [
      tailwindcss(),
    ],
  },
  eslint: {
    config: {
      stylistic: true,
    },
  },
  icon: { // for tailwind v4
    mode: 'css',
    cssLayer: 'base',
  },
})
