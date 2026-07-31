import { readFileSync } from 'node:fs'

import { SITE_LOCALES } from './shared/locales'

const cargoManifest = readFileSync(new URL('../Cargo.toml', import.meta.url), 'utf8')
const projectVersion = cargoManifest.match(/^version = "([^"]+)"$/m)?.[1]
const appBaseURL = process.env.NUXT_APP_BASE_URL || '/'
const siteUrl = new URL(process.env.NUXT_PUBLIC_SITE_URL ?? 'https://quitwin.ariex.ru').href

if (!projectVersion)
  throw new Error('Could not read the QuiTwin version from Cargo.toml')

const localeBootstrap = `(() => {
  const root = ${JSON.stringify(appBaseURL)}
  if (location.pathname !== root) return

  document.documentElement.style.visibility = 'hidden'

  const supported = new Set(${JSON.stringify(SITE_LOCALES.map(locale => locale.code))})
  const saved = document.cookie
    .split('; ')
    .find(cookie => cookie.startsWith('quitwin_locale='))
    ?.split('=')[1]
  const browserLocale = navigator.languages
    .map(language => language.toLowerCase().split('-')[0])
    .find(language => supported.has(language))
  const target = saved && supported.has(saved) ? saved : browserLocale

  if (target && target !== 'en') {
    location.replace(root + target + '/' + location.search + location.hash)
    return
  }

  document.documentElement.style.removeProperty('visibility')
})()`

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
    baseURL: appBaseURL,
    head: {
      script: [
        {
          key: 'locale-bootstrap',
          innerHTML: localeBootstrap,
          tagPriority: 'critical',
        },
      ],
    },
  },
  runtimeConfig: {
    public: {
      projectVersion,
      siteUrl,
    },
  },
  css: [
    '@fontsource-variable/space-grotesk',
    '~/assets/styles/global.scss',
  ],
  experimental: {
    payloadExtraction: false,
  },
  i18n: {
    baseUrl: siteUrl,
    defaultLocale: 'en',
    strategy: 'prefix_except_default',
    locales: SITE_LOCALES.map(locale => ({ ...locale })),
    detectBrowserLanguage: false,
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
