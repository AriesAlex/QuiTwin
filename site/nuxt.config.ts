import { readFileSync } from 'node:fs'

import { SITE_LOCALES } from './shared/locales'

const cargoManifest = readFileSync(new URL('../Cargo.toml', import.meta.url), 'utf8')
const projectVersion = cargoManifest.match(/^version = "([^"]+)"$/m)?.[1]

if (!projectVersion)
  throw new Error('Could not read the QuiTwin version from Cargo.toml')

export default defineNuxtConfig({
  compatibilityDate: '2026-07-27',
  modules: [
    '@nuxtjs/i18n',
  ],
  devtools: {
    enabled: false,
  },
  ssr: true,
  app: {
    baseURL: process.env.NUXT_APP_BASE_URL || '/',
  },
  runtimeConfig: {
    public: {
      projectVersion,
    },
  },
  css: [
    '@fontsource-variable/space-grotesk',
    '~/assets/css/main.css',
  ],
  experimental: {
    payloadExtraction: false,
  },
  i18n: {
    baseUrl: 'https://ariesalex.github.io',
    defaultLocale: 'en',
    strategy: 'prefix_except_default',
    locales: SITE_LOCALES.map(locale => ({ ...locale })),
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'quitwin_locale',
      redirectOn: 'root',
      fallbackLocale: 'en',
    },
    experimental: {
      prerenderMessages: true,
    },
  },
  nitro: {
    prerender: {
      crawlLinks: false,
      routes: ['/', '/ru', '/sr', '/pl', '/tr', '/fr', '/ar', '/zh'],
    },
  },
  typescript: {
    strict: true,
  },
})
