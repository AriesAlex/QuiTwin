<template>
  <div class="page">
    <header>
      <a class="wordmark" :href="baseURL" :aria-label="t('a11y.home')" translate="no">
        <img :src="`${baseURL}icon.png`" alt="" width="38" height="38">
        <span>QuiTwin</span>
      </a>

      <details class="language">
        <summary :aria-label="t('nav.language')">
          <PhGlobeSimple :size="20" weight="bold" />
          <span>{{ activeLocale.short }}</span>
        </summary>
        <nav>
          <NuxtLink
            v-for="item in SITE_LOCALES"
            :key="item.code"
            :to="switchLocalePath(item.code)"
            :hreflang="item.language"
            :lang="item.language"
            :aria-current="item.code === locale ? 'page' : undefined"
            @click="rememberLocale(item.code)"
          >
            <span>{{ item.name }}</span>
            <small>{{ item.short }}</small>
          </NuxtLink>
        </nav>
      </details>
    </header>

    <main>
      <section>
        <h1>{{ t('hero.title') }}</h1>
        <p class="copy">{{ t('hero.copy') }}</p>

        <div class="actions">
          <a class="download" :href="downloadUrl">
            <PhDownloadSimple :size="22" weight="bold" />
            <span>
              <strong>{{ t('hero.download') }}</strong>
              <small>{{ t('hero.platform') }}</small>
            </span>
          </a>
          <a class="github" :href="repositoryUrl" target="_blank" rel="noopener noreferrer">
            <PhGithubLogo :size="23" weight="fill" />
            <span>{{ t('hero.github') }}</span>
          </a>
        </div>

        <p class="after">{{ t('hero.after') }}</p>

        <p class="credits">ArieX · MIT</p>
      </section>

      <figure>
        <img
          :src="`${baseURL}icon.png`"
          :alt="t('a11y.logoAlt')"
          width="720"
          height="720"
          fetchpriority="high"
        >
      </figure>
    </main>
  </div>
</template>

<script setup lang="ts">
import { PhDownloadSimple, PhGithubLogo, PhGlobeSimple } from '@phosphor-icons/vue'

import { SITE_LOCALES } from '~/shared/locales'

const downloadUrl = 'https://github.com/AriesAlex/QuiTwin/releases/latest/download/QuiTwin.exe'
const repositoryUrl = 'https://github.com/AriesAlex/QuiTwin'
const siteUrl = 'https://ariesalex.github.io/QuiTwin/'
const runtimeConfig = useRuntimeConfig()
const baseURL = runtimeConfig.app.baseURL
const { locale, t } = useI18n()
const switchLocalePath = useSwitchLocalePath()
const i18nHead = useLocaleHead({ seo: true })
const localeCookie = useCookie('quitwin_locale', {
  path: '/',
  sameSite: 'lax',
  maxAge: 60 * 60 * 24 * 365,
})

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

function rememberLocale(code: string) {
  localeCookie.value = code
}

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
    { name: 'theme-color', content: '#08090d' },
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

<style scoped lang="scss">
@use '../assets/styles/tokens' as *;

.page {
  width: min(100%, 1280px);
  height: 100svh;
  min-height: 100svh;
  padding: clamp(22px, 3.5vw, 48px);
  margin: 0 auto;
  display: flex;
  flex-direction: column;
}

header {
  position: relative;
  z-index: 3;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.wordmark {
  display: inline-flex;
  align-items: center;
  gap: 11px;
  font-size: 1.08rem;
  font-weight: 680;
  letter-spacing: -0.035em;

  img {
    width: 38px;
    height: 38px;
  }
}

.language {
  position: relative;

  summary {
    min-height: 42px;
    padding: 0 14px;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    list-style: none;
    border-radius: 999px;
    background: $surface;
    color: $muted;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 680;
    transition: color 160ms ease, background 160ms ease;

    &::-webkit-details-marker {
      display: none;
    }

    &:hover {
      background: $surface-hover;
      color: $text;
    }
  }

  &[open] summary {
    background: $surface-hover;
    color: $text;
  }

  nav {
    position: absolute;
    top: calc(100% + 10px);
    inset-inline-end: 0;
    width: 210px;
    padding: 8px;
    display: flex;
    flex-direction: column;
    border-radius: 18px;
    background: $menu;
    box-shadow: 0 24px 70px rgba(0, 0, 0, 0.46);

    a {
      min-height: 40px;
      padding: 0 12px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 16px;
      border-radius: 12px;
      color: $muted;
      font-size: 0.88rem;
      transition: color 140ms ease, background 140ms ease;

      &:hover,
      &[aria-current='page'] {
        background: $surface;
        color: $text;
      }
    }

    small {
      color: $faint;
      font-size: 0.68rem;
      font-weight: 700;
    }
  }
}

main {
  flex: 1;
  min-height: 0;
  padding: clamp(46px, 7vh, 92px) 0 clamp(20px, 3vh, 40px);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: clamp(40px, 7vw, 110px);
}

section {
  position: relative;
  z-index: 1;
  flex: 1 1 660px;
  max-width: 720px;
}

h1 {
  max-width: 11ch;
  font-size: clamp(4rem, 7.6vw, 7.4rem);
  font-weight: 620;
  line-height: 0.9;
  letter-spacing: -0.075em;
  text-wrap: balance;
}

.copy {
  max-width: 600px;
  margin-top: clamp(24px, 4vh, 38px);
  color: $muted;
  font-size: clamp(1rem, 1.45vw, 1.2rem);
  line-height: 1.55;
  letter-spacing: -0.02em;
}

.actions {
  margin-top: clamp(28px, 4.5vh, 42px);
  display: flex;
  align-items: center;
  gap: 12px;

  a {
    min-height: 58px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 999px;
    font-weight: 650;
    transition: transform 160ms ease, background 160ms ease;

    &:hover {
      transform: translateY(-2px);
    }
  }
}

.download {
  min-width: min(100%, 250px);
  padding: 0 22px;
  gap: 13px;
  background: $accent;
  color: white;
  box-shadow: 0 14px 36px rgba($accent, 0.2);

  &:hover {
    background: $accent-hover;
  }

  span {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  strong {
    font-size: 0.95rem;
    font-weight: 700;
  }

  small {
    color: rgba(255, 255, 255, 0.7);
    font-size: 0.67rem;
    font-weight: 550;
  }
}

.github {
  min-width: 132px;
  padding: 0 20px;
  gap: 9px;
  background: $surface;
  color: $text;

  &:hover {
    background: $surface-hover;
  }
}

.after {
  margin-top: 16px;
  color: $faint;
  font-size: 0.78rem;
}

.credits {
  margin-top: clamp(34px, 6vh, 68px);
  color: $faint;
  font-size: 0.72rem;
}

figure {
  flex: 0 1 450px;
  display: flex;
  align-items: center;
  justify-content: center;

  img {
    width: min(34vw, 460px);
    min-width: 260px;
    filter: drop-shadow(0 34px 58px rgba($accent, 0.2));
  }
}

:global(html[dir='rtl']) {
  h1,
  .copy,
  .after,
  .credits {
    letter-spacing: 0;
  }

  .download span {
    align-items: flex-start;
  }
}

@media (max-width: 780px) {
  .page {
    height: auto;
    padding: 20px;
    overflow: hidden;
  }

  main {
    padding: 54px 0 26px;
  }

  section {
    max-width: 620px;
  }

  h1 {
    max-width: 10ch;
    font-size: clamp(3.45rem, 15vw, 5.3rem);
  }

  .copy {
    max-width: 540px;
    font-size: 1rem;
  }

  figure {
    position: absolute;
    inset-inline-end: 0;
    bottom: 0;
    z-index: 0;
    width: 48vw;
    height: 55vh;
    align-items: flex-end;
    justify-content: flex-start;
    overflow: hidden;
    opacity: 0.13;
    pointer-events: none;

    img {
      width: 70vw;
      min-width: 0;
      max-width: none;
      filter: none;
    }
  }

  .credits {
    max-width: 37ch;
  }
}

@media (max-width: 500px) {
  .actions {
    a {
      min-width: 0;
    }
  }

  .download {
    flex: 1;
    padding-inline: 14px;
  }

  .github {
    flex: 0 0 112px;
    padding-inline: 12px;
  }

  .credits {
    margin-top: 26px;
  }

  :lang(ru) h1,
  :lang(sr) h1,
  :lang(pl) h1,
  :lang(tr) h1,
  :lang(fr) h1 {
    font-size: clamp(2.85rem, 12.5vw, 4.15rem);
    line-height: 0.96;
  }

  :lang(ar) h1 {
    font-size: clamp(3rem, 13vw, 4.3rem);
    line-height: 1.05;
  }

  :lang(zh) h1 {
    font-size: clamp(3.15rem, 14vw, 4.5rem);
    line-height: 1.05;
  }
}

@media (max-width: 350px) {
  .actions {
    align-items: stretch;
    flex-direction: column;

    a {
      width: 100%;
    }
  }

  .github {
    flex-basis: auto;
  }
}

@media (prefers-reduced-motion: reduce) {
  .actions a {
    transition: none;

    &:hover {
      transform: none;
    }
  }
}
</style>
