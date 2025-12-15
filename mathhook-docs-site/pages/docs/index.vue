<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden">
    <!-- Vibrant Animated Mesh Gradient Background -->
    <div class="fixed inset-0 pointer-events-none overflow-hidden">
      <!-- Base dark layer -->
      <div class="absolute inset-0 bg-logic-navy-900"></div>

      <!-- Animated mesh gradient blobs -->
      <div class="absolute inset-0">
        <!-- Primary rust/orange blob - top left -->
        <div
          class="absolute w-[600px] h-[600px] rounded-full animate-blob-1"
          :style="{
            background: 'radial-gradient(circle at center, rgba(230, 69, 36, 0.35) 0%, rgba(230, 69, 36, 0.15) 40%, transparent 70%)',
            filter: 'blur(60px)',
            top: '-10%',
            left: '-10%',
            transform: `translate(${parallax.x * 0.03}px, ${parallax.y * 0.03}px)`
          }"
        ></div>

        <!-- Secondary cyan blob - bottom right -->
        <div
          class="absolute w-[500px] h-[500px] rounded-full animate-blob-2"
          :style="{
            background: 'radial-gradient(circle at center, rgba(6, 182, 212, 0.4) 0%, rgba(6, 182, 212, 0.15) 40%, transparent 70%)',
            filter: 'blur(60px)',
            bottom: '-5%',
            right: '-5%',
            transform: `translate(${parallax.x * -0.025}px, ${parallax.y * -0.025}px)`
          }"
        ></div>

        <!-- Tertiary purple blob - center right -->
        <div
          class="absolute w-[450px] h-[450px] rounded-full animate-blob-3"
          :style="{
            background: 'radial-gradient(circle at center, rgba(139, 92, 246, 0.3) 0%, rgba(139, 92, 246, 0.1) 40%, transparent 70%)',
            filter: 'blur(80px)',
            top: '40%',
            right: '20%',
            transform: `translate(${parallax.x * 0.02}px, ${parallax.y * -0.02}px)`
          }"
        ></div>

        <!-- Accent green blob - bottom left -->
        <div
          class="absolute w-[350px] h-[350px] rounded-full animate-blob-4"
          :style="{
            background: 'radial-gradient(circle at center, rgba(34, 197, 94, 0.25) 0%, rgba(34, 197, 94, 0.08) 40%, transparent 70%)',
            filter: 'blur(60px)',
            bottom: '20%',
            left: '15%',
            transform: `translate(${parallax.x * 0.02}px, ${parallax.y * 0.02}px)`
          }"
        ></div>
      </div>

      <!-- Floating particles -->
      <div class="absolute inset-0">
        <div v-for="i in 20" :key="'particle-'+i"
             class="absolute rounded-full animate-float-particle"
             :style="{
               width: `${2 + Math.random() * 3}px`,
               height: `${2 + Math.random() * 3}px`,
               background: ['rgba(230, 69, 36, 0.5)', 'rgba(6, 182, 212, 0.5)', 'rgba(139, 92, 246, 0.4)', 'rgba(255, 255, 255, 0.3)'][Math.floor(Math.random() * 4)],
               left: `${Math.random() * 100}%`,
               top: `${Math.random() * 100}%`,
               animationDelay: `${Math.random() * 8}s`,
               animationDuration: `${10 + Math.random() * 15}s`
             }"
        ></div>
      </div>

      <!-- Subtle grid overlay -->
      <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:48px_48px]"></div>

      <!-- Readability overlay -->
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900/40 via-transparent to-logic-navy-900/60"></div>
    </div>

    <!-- Navigation -->
    <nav
      class="sticky top-0 z-50 transition-all duration-500"
      :class="scrollY > 20 ? 'bg-logic-navy-900/95 backdrop-blur-xl border-b border-logic-navy-700/50 shadow-lg shadow-black/10' : 'bg-logic-navy-900/80 backdrop-blur-lg border-b border-logic-navy-700/50'"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <NuxtLink to="/" class="flex items-center gap-2 group">
            <svg class="w-7 h-7 transition-transform duration-300 group-hover:scale-110" viewBox="0 0 48 48" fill="none">
              <defs>
                <linearGradient id="nav-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#E64524"/>
                  <stop offset="100%" stop-color="#06B6D4"/>
                </linearGradient>
              </defs>
              <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#nav-gradient)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <span class="text-lg font-semibold">
              <span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span>
            </span>
          </NuxtLink>
          <div class="flex items-center gap-6">
            <NuxtLink to="/docs" class="text-solve-cyan font-medium">Docs</NuxtLink>
            <NuxtLink to="/outputs" class="text-chalk-500 hover:text-chalk transition-all duration-300 hover:-translate-y-0.5">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-all duration-300 flex items-center gap-1 hover:-translate-y-0.5">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <div class="max-w-5xl mx-auto px-4 py-12 relative z-10">
      <!-- Header with Animation -->
      <div class="mb-12">
        <div
          :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-6': !mounted }"
          style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 0.1s"
        >
          <span class="inline-block px-4 py-1.5 bg-solve-cyan/10 border border-solve-cyan/20 rounded-full text-xs text-solve-cyan font-medium mb-4">DOCUMENTATION</span>
        </div>
        <h1
          class="text-4xl md:text-5xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-rust-core to-solve-cyan"
          :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-6': !mounted }"
          style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 0.2s"
        >
          Documentation
        </h1>
        <p
          class="text-xl text-chalk-400"
          :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-4': !mounted }"
          style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 0.3s"
        >
          Explore comprehensive mathematical documentation with interactive examples
        </p>
        <p
          class="text-sm text-chalk-600 mt-2"
          :class="{ 'opacity-100': mounted, 'opacity-0': !mounted }"
          style="transition: opacity 0.5s; transition-delay: 0.5s"
        >
          {{ Object.values(categories).flat().length }} topics available
        </p>
      </div>

      <!-- Search with Animation -->
      <div
        class="mb-10"
        :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-4': !mounted }"
        style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 0.4s"
      >
        <div class="relative group">
          <div class="absolute -inset-0.5 bg-gradient-to-r from-rust-core/20 to-solve-cyan/20 rounded-xl blur opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
          <div class="relative">
            <svg class="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-chalk-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search documentation..."
              class="w-full pl-12 pr-4 py-4 bg-logic-navy-800/80 border border-logic-navy-700 rounded-xl text-chalk placeholder-chalk-500 focus:outline-none focus:border-solve-cyan/50 focus:ring-2 focus:ring-solve-cyan/20 transition-all duration-300 backdrop-blur-sm"
            />
          </div>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="space-y-6">
        <div v-for="i in 3" :key="i" class="animate-pulse">
          <div class="h-8 bg-logic-navy-700/50 rounded-lg w-48 mb-4"></div>
          <div class="space-y-3">
            <div class="h-20 bg-logic-navy-800/50 rounded-xl"></div>
            <div class="h-20 bg-logic-navy-800/50 rounded-xl"></div>
          </div>
        </div>
      </div>

      <!-- Categories -->
      <div v-else class="space-y-10">
        <section
          v-for="(items, category, categoryIndex) in filteredCategories"
          :key="category"
          class="relative"
          v-scroll-animate="{ animation: 'fade-up', delay: categoryIndex * 100 }"
        >
          <!-- Category Card -->
          <div class="bg-logic-navy-800/30 border border-logic-navy-700/50 rounded-2xl p-6 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500">
            <!-- Category Header -->
            <div class="flex items-center gap-4 mb-6">
              <div
                class="w-12 h-12 rounded-xl flex items-center justify-center transition-transform duration-300 hover:scale-110 hover:rotate-3"
                :class="getCategoryStyle(category).bg"
              >
                <span class="text-2xl">{{ getCategoryStyle(category).icon }}</span>
              </div>
              <div>
                <h2 class="text-xl font-semibold text-chalk">{{ getCategoryStyle(category).title }}</h2>
                <p class="text-sm text-chalk-500">{{ items.length }} topics</p>
              </div>
            </div>

            <!-- Topic Items -->
            <div class="space-y-3">
              <NuxtLink
                v-for="(item, itemIndex) in items"
                :key="item.topic"
                :to="`/docs/${item.topic}`"
                class="block p-4 rounded-xl transition-all duration-300 group border hover:-translate-y-1 hover:shadow-lg"
                :class="getCategoryStyle(category).itemClass"
                :style="{ transitionDelay: `${itemIndex * 50}ms` }"
              >
                <div class="flex items-start justify-between gap-4">
                  <div class="flex-1">
                    <h3
                      class="font-semibold text-lg transition-colors flex items-center gap-2"
                      :class="getCategoryStyle(category).titleClass"
                    >
                      {{ item.title }}
                      <svg class="w-4 h-4 opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                      </svg>
                    </h3>
                    <p class="text-chalk-500 mt-1 text-sm leading-relaxed group-hover:text-chalk-400 transition-colors">
                      {{ item.description }}
                    </p>
                  </div>
                </div>
              </NuxtLink>
            </div>
          </div>
        </section>

        <!-- No results -->
        <div
          v-if="Object.keys(filteredCategories).length === 0"
          class="text-center py-16"
          v-scroll-animate="{ animation: 'fade-up' }"
        >
          <div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-logic-navy-800 flex items-center justify-center">
            <svg class="w-8 h-8 text-chalk-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
          </div>
          <p class="text-chalk-400 text-lg mb-2">No documentation found</p>
          <p class="text-chalk-600">Try a different search term for "{{ searchQuery }}"</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const searchQuery = ref('')
const categories = ref({})
const loading = ref(true)
const mounted = ref(false)
const scrollY = ref(0)
const parallax = ref({ x: 0, y: 0 })

// Category configuration
const categoryConfig = {
  'getting-started': { title: 'Getting Started', icon: '🚀', color: 'rust-core', order: 1 },
  'introduction': { title: 'Introduction', icon: '📖', color: 'solve-cyan', order: 2 },
  'core': { title: 'Core Concepts', icon: '⚙️', color: 'amber-500', order: 3 },
  'operations': { title: 'Operations', icon: '🔧', color: 'solve-cyan', order: 4 },
  'calculus': { title: 'Calculus', icon: '📈', color: 'rust-core', order: 5 },
  'algebra': { title: 'Algebra', icon: '🎨', color: 'amber-500', order: 6 },
  'ode': { title: 'Ordinary Differential Equations', icon: '📐', color: 'solve-cyan', order: 7 },
  'pde': { title: 'Partial Differential Equations', icon: '🌊', color: 'step-green', order: 8 },
  'polynomial': { title: 'Polynomials', icon: '📊', color: 'violet-500', order: 9 },
  'special_functions': { title: 'Special Functions', icon: '⭐', color: 'violet-500', order: 10 },
  'parser': { title: 'Parsing & Formatting', icon: '📝', color: 'amber-500', order: 11 },
  'educational': { title: 'Educational Features', icon: '🎓', color: 'step-green', order: 12 },
  'evaluation': { title: 'Evaluation', icon: '⚡', color: 'rust-core', order: 13 },
  'api': { title: 'API Reference', icon: '📚', color: 'solve-cyan', order: 14 },
  'bindings': { title: 'Language Bindings', icon: '🔗', color: 'step-green', order: 15 },
  'architecture': { title: 'Architecture', icon: '🏗️', color: 'violet-500', order: 16 },
  'performance': { title: 'Performance', icon: '⚡', color: 'rust-core', order: 17 },
  'advanced': { title: 'Advanced Topics', icon: '🔬', color: 'violet-500', order: 18 },
  'advanced-pde': { title: 'Advanced PDEs', icon: '🌊', color: 'step-green', order: 18.5 },
  'contributing': { title: 'Contributing', icon: '🤝', color: 'step-green', order: 19 },
  'examples': { title: 'Examples', icon: '💡', color: 'amber-500', order: 20 },
  'dev': { title: 'Developer Guide', icon: '💻', color: 'pink-500', order: 21 },
  'internal': { title: 'Internal Documentation', icon: '🔒', color: 'chalk-500', order: 22 },
  'appendix': { title: 'Appendix', icon: '📎', color: 'chalk-500', order: 23 },
}

const getCategoryStyle = (category) => {
  const config = categoryConfig[category] || { title: category, icon: '📄', color: 'chalk-500' }
  return {
    title: config.title,
    icon: config.icon,
    bg: `bg-${config.color}/10`,
    itemClass: `bg-${config.color}/5 hover:bg-${config.color}/10 border-${config.color}/10 hover:border-${config.color}/30 hover:shadow-${config.color}/5`,
    titleClass: `text-${config.color} group-hover:text-white`
  }
}

// Scroll animation directive
const vScrollAnimate = {
  mounted(el, binding) {
    const options = binding.value || {}
    const animation = options.animation || 'fade-up'
    const delay = options.delay || 0

    el.style.opacity = '0'
    el.style.transform = animation === 'fade-up' ? 'translateY(30px)' :
                         animation === 'fade-left' ? 'translateX(30px)' :
                         animation === 'fade-right' ? 'translateX(-30px)' : 'translateY(30px)'
    el.style.transition = `all 0.8s cubic-bezier(0.16, 1, 0.3, 1) ${delay}ms`

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          setTimeout(() => {
            el.style.opacity = '1'
            el.style.transform = 'translateY(0) translateX(0)'
          }, 50)
          observer.unobserve(el)
        }
      })
    }, { threshold: 0.1, rootMargin: '50px' })

    observer.observe(el)
  }
}

// Scroll handler
const handleScroll = () => {
  scrollY.value = window.scrollY
}

// Mouse move for parallax
const handleMouseMove = (e) => {
  parallax.value = {
    x: (e.clientX - window.innerWidth / 2) / 100,
    y: (e.clientY - window.innerHeight / 2) / 100
  }
}

// Load all documentation metadata
onMounted(async () => {
  mounted.value = true
  window.addEventListener('scroll', handleScroll, { passive: true })
  window.addEventListener('mousemove', handleMouseMove, { passive: true })

  try {
    const topicsResponse = await fetch('/data/_topics.json')
    if (!topicsResponse.ok) {
      throw new Error('Failed to load topics index')
    }
    const topics = await topicsResponse.json()

    const grouped = {}

    const getCategory = (topicName) => {
      if (topicName.startsWith('getting-started-')) return 'getting-started'
      if (topicName.startsWith('advanced-pde-')) return 'advanced-pde'
      return topicName.split('-')[0]
    }

    for (const topic of topics) {
      try {
        const response = await fetch(`/data/${topic}.json`)
        if (response.ok) {
          const data = await response.json()
          const category = getCategory(topic)

          if (!grouped[category]) {
            grouped[category] = []
          }

          grouped[category].push({
            topic,
            title: data.title || topic,
            description: data.description ? data.description.substring(0, 150) + (data.description.length > 150 ? '...' : '') : ''
          })
        }
      } catch (e) {
        // Skip topics that fail to load
      }
    }

    const sortedCategories = {}
    Object.keys(grouped)
      .sort((a, b) => (categoryConfig[a]?.order || 99) - (categoryConfig[b]?.order || 99))
      .forEach(key => {
        sortedCategories[key] = grouped[key]
      })

    categories.value = sortedCategories
  } catch (err) {
    console.error('Failed to load documentation:', err)
  } finally {
    loading.value = false
  }
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
  window.removeEventListener('mousemove', handleMouseMove)
})

// Filter categories based on search
const filteredCategories = computed(() => {
  if (!searchQuery.value) return categories.value

  const query = searchQuery.value.toLowerCase()
  const filtered = {}

  for (const [category, items] of Object.entries(categories.value)) {
    const matchingItems = items.filter(item =>
      item.title.toLowerCase().includes(query) ||
      item.description.toLowerCase().includes(query) ||
      item.topic.toLowerCase().includes(query)
    )
    if (matchingItems.length > 0) {
      filtered[category] = matchingItems
    }
  }

  return filtered
})

useHead({
  title: 'Documentation - MathHook Knowledge Base',
  meta: [
    {
      name: 'description',
      content: 'Browse MathHook mathematical documentation covering calculus, algebra, ODEs, PDEs, and more. Symbolic Power. Educational Clarity. Native Speed.'
    }
  ]
})
</script>

<style scoped>
/* Blob animations - organic flowing movement */
@keyframes blob-1 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  25% { transform: translate(60px, -40px) scale(1.1); }
  50% { transform: translate(30px, 60px) scale(0.95); }
  75% { transform: translate(-40px, 30px) scale(1.05); }
}

@keyframes blob-2 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  25% { transform: translate(-50px, 40px) scale(1.05); }
  50% { transform: translate(40px, -30px) scale(1.1); }
  75% { transform: translate(20px, 50px) scale(0.95); }
}

@keyframes blob-3 {
  0%, 100% { transform: translate(0, 0) scale(1) rotate(0deg); }
  33% { transform: translate(40px, 50px) scale(1.08) rotate(8deg); }
  66% { transform: translate(-30px, -40px) scale(0.92) rotate(-8deg); }
}

@keyframes blob-4 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  20% { transform: translate(-40px, -25px) scale(1.1); }
  40% { transform: translate(35px, 40px) scale(0.92); }
  60% { transform: translate(55px, -15px) scale(1.05); }
  80% { transform: translate(-25px, 50px) scale(0.95); }
}

.animate-blob-1 { animation: blob-1 20s ease-in-out infinite; }
.animate-blob-2 { animation: blob-2 25s ease-in-out infinite; }
.animate-blob-3 { animation: blob-3 18s ease-in-out infinite; }
.animate-blob-4 { animation: blob-4 22s ease-in-out infinite; }

/* Floating particles animation */
@keyframes float-particle {
  0%, 100% { transform: translateY(0) translateX(0); opacity: 0; }
  10% { opacity: 1; }
  90% { opacity: 1; }
  100% { transform: translateY(-100vh) translateX(40px); opacity: 0; }
}

.animate-float-particle {
  animation: float-particle 15s ease-in-out infinite;
}
</style>
