const counterId = 111206645

declare global {
  interface Window {
    ym?: (id: number, method: string, ...args: unknown[]) => void
  }
}

export default defineNuxtPlugin(() => {
  useHead({
    script: [
      {
        key: 'yandex-metrika',
        innerHTML: `(function(m,e,t,r,i,k,a){m[i]=m[i]||function(){(m[i].a=m[i].a||[]).push(arguments)};m[i].l=1*new Date();for(var j=0;j<document.scripts.length;j++){if(document.scripts[j].src===r){return}}k=e.createElement(t),a=e.getElementsByTagName(t)[0],k.async=1,k.src=r,a.parentNode.insertBefore(k,a)})(window,document,'script','https://mc.yandex.ru/metrika/tag.js?id=${counterId}','ym');ym(${counterId},'init',{ssr:true,webvisor:true,clickmap:true,ecommerce:'dataLayer',accurateTrackBounce:true,trackLinks:true});`,
      },
    ],
    noscript: [
      {
        key: 'yandex-metrika-noscript',
        innerHTML: `<div><img src="https://mc.yandex.ru/watch/${counterId}" style="position:absolute;left:-9999px" alt=""></div>`,
      },
    ],
  })

  if (import.meta.server)
    return

  const router = useRouter()
  let ready = false

  void router.isReady().then(() => {
    ready = true
  })

  router.afterEach((to, from, failure) => {
    if (!ready || failure)
      return

    window.ym?.(counterId, 'hit', to.fullPath, { referer: from.fullPath })
  })
})
