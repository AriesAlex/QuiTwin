import { access, readFile, stat } from 'node:fs/promises'
import { join } from 'node:path'

const output = join(import.meta.dirname, '..', '.output', 'public')
const siteUrl = 'https://ariesalex.github.io/QuiTwin/'
const cargoManifest = await readFile(join(import.meta.dirname, '..', '..', 'Cargo.toml'), 'utf8')
const projectVersion = cargoManifest.match(/^version = "([^"]+)"$/m)?.[1]
const locales = [
  { route: '', language: 'en', dir: 'ltr' },
  { route: 'ru', language: 'ru', dir: 'ltr' },
  { route: 'sr', language: 'sr-Latn', dir: 'ltr' },
  { route: 'pl', language: 'pl', dir: 'ltr' },
  { route: 'tr', language: 'tr', dir: 'ltr' },
  { route: 'fr', language: 'fr', dir: 'ltr' },
  { route: 'ar', language: 'ar', dir: 'rtl' },
  { route: 'zh', language: 'zh-Hans', dir: 'ltr' },
]

if (!projectVersion)
  throw new Error('Could not read the QuiTwin version from Cargo.toml')

function requireMatch(html, pattern, expected, label) {
  const actual = html.match(pattern)?.[1]
  if (actual !== expected)
    throw new Error(`${label}: expected ${expected}, received ${actual ?? 'nothing'}`)
}

for (const locale of locales) {
  const page = locale.route
    ? join(output, locale.route, 'index.html')
    : join(output, 'index.html')
  const html = await readFile(page, 'utf8')
  const canonical = locale.route ? `${siteUrl}${locale.route}` : siteUrl

  requireMatch(html, /<html[^>]*\blang="([^"]+)"/, locale.language, `${locale.route || 'en'} lang`)
  requireMatch(html, /<html[^>]*\bdir="([^"]+)"/, locale.dir, `${locale.route || 'en'} dir`)
  requireMatch(html, /<link id="i18n-can" rel="canonical" href="([^"]+)"/, canonical, `${locale.route || 'en'} canonical`)

  if (html.includes('/QuiTwin/QuiTwin/'))
    throw new Error(`${locale.route || 'en'} contains a doubled GitHub Pages base path`)
  if (!html.includes(`${siteUrl}og.png`))
    throw new Error(`${locale.route || 'en'} is missing its social image`)
  if (!html.includes('application/ld+json'))
    throw new Error(`${locale.route || 'en'} is missing structured data`)
  if (!html.includes(`"softwareVersion":"${projectVersion}"`))
    throw new Error(`${locale.route || 'en'} does not use the Cargo package version`)
  if ((html.match(/rel="alternate"/g) ?? []).length !== 11)
    throw new Error(`${locale.route || 'en'} does not expose every hreflang alternate`)
}

for (const asset of ['favicon.ico', 'icon.png', 'og.png', 'robots.txt', 'sitemap.xml']) {
  const path = join(output, asset)
  await access(path)
  if ((await stat(path)).size === 0)
    throw new Error(`${asset} is empty`)
}

const sitemap = await readFile(join(output, 'sitemap.xml'), 'utf8')
for (const locale of locales) {
  const url = locale.route ? `${siteUrl}${locale.route}/` : siteUrl
  if (!sitemap.includes(`<loc>${url}</loc>`))
    throw new Error(`sitemap.xml is missing ${url}`)
}

console.log(`Verified ${locales.length} localized pages and production SEO metadata.`)
