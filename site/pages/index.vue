<script setup lang="ts">
import { SITE_LOCALES } from '~/shared/locales'

const downloadUrl = 'https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe'
const repositoryUrl = 'https://github.com/AriesAlex/QuiTwin'
const siteUrl = 'https://ariesalex.github.io/QuiTwin/'
const runtimeConfig = useRuntimeConfig()
const baseURL = runtimeConfig.app.baseURL
const { locale, t } = useI18n()
const switchLocalePath = useSwitchLocalePath()
const i18nHead = useLocaleHead({ seo: true })

const flow = [
  { number: '01', key: 'stock' },
  { number: '02', key: 'twin' },
  { number: '03', key: 'equicord' },
  { number: '04', key: 'rebuild' },
] as const

const safeguards = [
  { key: 'process', size: 'wide' },
  { key: 'atomic', size: 'compact' },
  { key: 'native', size: 'tall' },
  { key: 'cleanup', size: 'wide' },
] as const

const activeLocale = computed(() =>
  SITE_LOCALES.find(item => item.code === locale.value) ?? SITE_LOCALES[0],
)
const localizedUrl = computed(() => {
  const path = switchLocalePath(locale.value).replace(/^\//, '')
  return new URL(path, siteUrl).href
})
const metaTitle = computed(() => t('meta.title'))
const metaDescription = computed(() => t('meta.description'))
const metaImageAlt = computed(() => t('meta.ogAlt'))

useSeoMeta({
  title: metaTitle,
  description: metaDescription,
  ogTitle: metaTitle,
  ogDescription: metaDescription,
  ogType: 'website',
  ogSiteName: 'QuiTwin',
  ogUrl: localizedUrl,
  ogImage: `${siteUrl}og.png`,
  ogImageAlt: metaImageAlt,
  twitterCard: 'summary_large_image',
  twitterTitle: metaTitle,
  twitterDescription: metaDescription,
  twitterImage: `${siteUrl}og.png`,
})

useHead(() => ({
  htmlAttrs: i18nHead.value.htmlAttrs,
  link: [
    ...(i18nHead.value.link ?? []),
    { rel: 'icon', type: 'image/x-icon', href: `${baseURL}favicon.ico` },
  ],
  meta: [
    ...(i18nHead.value.meta ?? []),
    { name: 'theme-color', content: '#08090d', media: '(prefers-color-scheme: dark)' },
    { name: 'theme-color', content: '#f4f2ed', media: '(prefers-color-scheme: light)' },
    { name: 'robots', content: 'index, follow' },
  ],
  script: [
    {
      type: 'application/ld+json',
      innerHTML: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'SoftwareApplication',
        name: 'QuiTwin',
        description: t('meta.description'),
        url: localizedUrl.value,
        downloadUrl,
        applicationCategory: 'UtilitiesApplication',
        operatingSystem: 'Windows 10, Windows 11',
        softwareVersion: runtimeConfig.public.projectVersion,
        inLanguage: activeLocale.value.language,
        author: {
          '@type': 'Person',
          name: 'ArieX',
          url: 'https://github.com/AriesAlex',
        },
        offers: {
          '@type': 'Offer',
          price: '0',
          priceCurrency: 'USD',
        },
        license: 'https://opensource.org/license/mit',
        codeRepository: repositoryUrl,
      }),
    },
  ],
}))
</script>

<template>
  <a class="skip-link" href="#main">{{ t('a11y.skip') }}</a>

  <header class="site-header">
    <nav class="site-nav" :aria-label="t('a11y.primaryNav')">
      <a class="wordmark" href="#top" :aria-label="t('a11y.home')" translate="no">
        <img :src="`${baseURL}icon.png`" alt="" width="40" height="40">
        <span>QuiTwin</span>
      </a>

      <div class="nav-links">
        <a href="#how-it-works">{{ t('nav.how') }}</a>
        <a :href="repositoryUrl" target="_blank" rel="noopener noreferrer">{{ t('nav.source') }} ↗</a>

        <details class="language-picker">
          <summary :aria-label="t('nav.language')">
            <span>{{ activeLocale.short }}</span>
            <span aria-hidden="true">+</span>
          </summary>
          <div class="language-menu">
            <NuxtLink
              v-for="item in SITE_LOCALES"
              :key="item.code"
              :to="switchLocalePath(item.code)"
              :hreflang="item.language"
              :lang="item.language"
              :aria-current="item.code === locale ? 'page' : undefined"
            >
              <span>{{ item.short }}</span>
              <span>{{ item.name }}</span>
            </NuxtLink>
          </div>
        </details>

        <a class="nav-download" :href="downloadUrl">{{ t('nav.download') }}</a>
      </div>
    </nav>
  </header>

  <main id="main">
    <section id="top" class="hero" aria-labelledby="hero-title">
      <div class="hero-copy">
        <h1 id="hero-title">
          {{ t('hero.titleFirst') }}<br>
          <span>{{ t('hero.titleSecond') }}</span>
        </h1>
        <p class="hero-lede">{{ t('hero.lede') }}</p>
        <div class="hero-actions">
          <a class="button button-primary" :href="downloadUrl">
            <span>{{ t('hero.download') }}</span>
            <small>{{ t('hero.platform') }}</small>
          </a>
          <a class="text-link" href="#how-it-works">{{ t('hero.mechanism') }} ↓</a>
        </div>
        <p class="hero-fineprint">{{ t('hero.fineprint') }}</p>
      </div>

      <div class="hero-visual" :aria-label="t('a11y.productFacts')">
        <div class="brand-field">
          <span class="brand-index">QT / 01</span>
          <img
            class="hero-mark"
            :src="`${baseURL}icon.png`"
            :alt="t('a11y.logoAlt')"
            width="720"
            height="720"
            fetchpriority="high"
          >
          <span class="brand-caption">{{ t('hero.brandLineOne') }}<br>{{ t('hero.brandLineTwo') }}</span>
        </div>
        <dl class="hero-stats">
          <div>
            <dt>1</dt>
            <dd>{{ t('hero.statExe') }}</dd>
          </div>
          <div>
            <dt>0</dt>
            <dd>{{ t('hero.statServices') }}</dd>
          </div>
          <div>
            <dt>NTFS</dt>
            <dd>{{ t('hero.statRuntime') }}</dd>
          </div>
        </dl>
      </div>
    </section>

    <aside class="trust-strip" :aria-label="t('a11y.status')">
      <span>{{ t('trust.license') }}</span>
      <span>{{ t('trust.source') }}</span>
      <span>{{ t('trust.channels') }}</span>
      <span>{{ t('trust.windows') }}</span>
    </aside>

    <section class="problem reveal" aria-labelledby="problem-title">
      <div class="section-number" aria-hidden="true">01</div>
      <div class="problem-heading">
        <h2 id="problem-title">{{ t('problem.title') }}</h2>
      </div>
      <div class="problem-copy">
        <p>{{ t('problem.copyOne') }}</p>
        <p>{{ t('problem.copyTwo') }}</p>
      </div>
    </section>

    <section id="how-it-works" class="mechanism reveal" aria-labelledby="mechanism-title">
      <div class="mechanism-intro">
        <h2 id="mechanism-title">
          {{ t('mechanism.titleFirst') }}<br>{{ t('mechanism.titleSecond') }}
        </h2>
        <p>{{ t('mechanism.intro') }}</p>
      </div>

      <ol class="flow-list">
        <li v-for="step in flow" :key="step.number">
          <span class="flow-number">{{ step.number }}</span>
          <div>
            <h3>{{ t(`mechanism.steps.${step.key}.title`) }}</h3>
            <p>{{ t(`mechanism.steps.${step.key}.copy`) }}</p>
          </div>
        </li>
      </ol>
    </section>

    <section class="safeguards reveal" aria-labelledby="safeguards-title">
      <div class="safeguards-heading">
        <h2 id="safeguards-title">{{ t('safeguards.title') }}</h2>
      </div>

      <div class="safeguard-grid">
        <article
          v-for="item in safeguards"
          :key="item.key"
          class="safeguard"
          :class="`safeguard-${item.size}`"
        >
          <h3>{{ t(`safeguards.items.${item.key}.title`) }}</h3>
          <p>{{ t(`safeguards.items.${item.key}.copy`) }}</p>
        </article>
      </div>
    </section>

    <section class="proof reveal" aria-labelledby="proof-title">
      <div class="proof-copy">
        <h2 id="proof-title">{{ t('proof.title') }}</h2>
        <p>{{ t('proof.copy') }}</p>
      </div>

      <div class="proof-ledger" :aria-label="t('a11y.verified')">
        <div>
          <span>{{ t('proof.host') }}</span>
          <strong>{{ t('proof.completed') }}</strong>
        </div>
        <div>
          <span>{{ t('proof.restart') }}</span>
          <strong>{{ t('proof.survived') }}</strong>
        </div>
        <div>
          <span>{{ t('proof.portable') }}</span>
          <strong>{{ t('proof.deleted') }}</strong>
        </div>
        <div>
          <span>{{ t('proof.resident') }}</span>
          <strong>{{ t('proof.none') }}</strong>
        </div>
      </div>
    </section>

    <section class="install reveal" aria-labelledby="install-title">
      <div>
        <h2 id="install-title">
          {{ t('install.lineOne') }}<br>{{ t('install.lineTwo') }}<br>{{ t('install.lineThree') }}
        </h2>
      </div>
      <div class="install-action">
        <p>{{ t('install.copy') }}</p>
        <a class="button button-light" :href="downloadUrl">
          <span>{{ t('install.button') }}</span>
          <small>{{ t('install.latest') }}</small>
        </a>
        <p class="risk-note">{{ t('install.risk') }}</p>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="footer-brand">
      <img :src="`${baseURL}icon.png`" alt="" width="34" height="34">
      <span>{{ t('footer.by') }}</span>
    </div>
    <div class="footer-links">
      <a :href="repositoryUrl" target="_blank" rel="noopener noreferrer">GitHub ↗</a>
      <a href="https://github.com/AriesAlex/QuiTwin/releases/latest" target="_blank" rel="noopener noreferrer">{{ t('footer.releases') }} ↗</a>
      <a href="https://github.com/AriesAlex/QuiTwin/blob/main/LICENSE" target="_blank" rel="noopener noreferrer">{{ t('footer.license') }} ↗</a>
    </div>
    <p>{{ t('footer.independent') }}</p>
  </footer>
</template>
