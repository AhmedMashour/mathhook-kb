// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  modules: [
    '@nuxtjs/tailwindcss',
    '@nuxtjs/google-fonts',
  ],

  // Google Fonts Configuration - Engineering Typography Baseline
  // Inter: Clean, highly legible, excellent for technical content
  // JetBrains Mono: Purpose-built for code, ligatures optional
  googleFonts: {
    families: {
      Inter: [400, 500, 600, 700, 800],
      'JetBrains Mono': [400, 500, 600, 700],
    },
    display: 'swap',
    prefetch: true,
    preconnect: true,
    preload: true,
  },

  // Tailwind CSS Configuration
  tailwindcss: {
    configPath: 'tailwind.config.ts',
    cssPath: '~/assets/css/main.css',
    exposeConfig: true,
  },

  app: {
    head: {
      title: 'MathHook - Symbolic Power. Educational Clarity. Native Speed.',
      htmlAttrs: {
        lang: 'en',
      },
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        {
          name: 'description',
          content: 'MathHook: High-performance computer algebra system in Rust. Symbolic mathematics, equation solving, calculus, ODEs, PDEs with mathematical correctness guaranteed.'
        },
        // Open Graph
        { property: 'og:type', content: 'website' },
        { property: 'og:title', content: 'MathHook - High-Performance CAS for Rust, Python, and Node' },
        { property: 'og:description', content: 'Symbolic Power. Educational Clarity. Native Speed. The high-performance computer algebra system.' },
        { property: 'og:image', content: '/og-image.png' },
        // Twitter Card
        { name: 'twitter:card', content: 'summary_large_image' },
        { name: 'twitter:title', content: 'MathHook CAS' },
        { name: 'twitter:description', content: 'Solve Fast. Understand Deeply.' },
        // Theme color
        { name: 'theme-color', content: '#0F172A' },
      ],
      link: [
        { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' },
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
        // KaTeX CSS for math rendering
        {
          rel: 'stylesheet',
          href: 'https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css'
        }
      ],
      script: [
        // KaTeX JS for math rendering
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
  },

  // Route rules for static generation
  routeRules: {
    '/': { prerender: true },
    '/docs': { prerender: true },
    '/outputs': { prerender: true },
  },

  // Nitro server configuration for Railway deployment
  nitro: {
    preset: 'node-server',
  },

  // Runtime config for environment variables
  runtimeConfig: {
    public: {
      siteUrl: process.env.NUXT_PUBLIC_SITE_URL || 'https://mathook.org',
    }
  },
})
