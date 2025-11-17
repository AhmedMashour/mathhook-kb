// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  modules: ['@nuxtjs/tailwindcss'],

  app: {
    head: {
      title: 'MathHook Knowledge Base',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        {
          name: 'description',
          content: 'Interactive mathematical documentation powered by MathHook CAS'
        }
      ],
      link: [
        {
          rel: 'stylesheet',
          href: 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css'
        }
      ],
      script: [
        {
          src: 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js',
          defer: true
        }
      ]
    }
  },

  ssr: true,

  experimental: {
    payloadExtraction: false
  }
})
