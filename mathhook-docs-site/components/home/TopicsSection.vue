<template>
  <section class="py-24 px-4 relative z-10">
    <div class="max-w-6xl mx-auto">
      <div class="text-center mb-16" v-scroll-animate="{ animation: 'fade-up' }">
        <span class="inline-block px-4 py-1.5 bg-step-green/10 border border-step-green/20 rounded-full text-xs text-step-green font-medium mb-4">DOCUMENTATION</span>
        <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Comprehensive Math Coverage</h2>
        <p class="text-chalk-400 text-lg">From basic algebra to advanced differential equations</p>
        <p v-if="categoryTopics.length > 0" class="text-xs text-chalk-600 mt-3">
          {{ totalTopicCount }} topics across {{ categoryTopics.length }} categories
        </p>
      </div>

      <div class="grid grid-cols-2 md:grid-cols-4 gap-5">
        <NuxtLink
          v-for="(cat, index) in categoryTopics"
          :key="cat.id"
          :to="cat.firstTopic ? `/docs/${cat.firstTopic}` : '/docs'"
          class="group relative p-6 bg-logic-navy-800/30 border border-logic-navy-700/50 rounded-2xl transition-all duration-500 hover:-translate-y-2 overflow-hidden"
          :class="`hover:border-${cat.color}/50`"
          v-scroll-animate="{ animation: 'fade-up', delay: index * 75 }"
        >
          <!-- Gradient overlay on hover -->
          <div
            class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none"
            :class="`bg-gradient-to-br from-${cat.color}/10 to-transparent`"
          ></div>

          <div class="relative z-10">
            <div class="text-3xl mb-3 transition-transform duration-300 group-hover:scale-110 group-hover:-rotate-6">{{ cat.icon }}</div>
            <div class="font-semibold mb-1 transition-colors" :class="`text-${cat.color} group-hover:text-white`">{{ cat.title }}</div>
            <div class="text-xs text-chalk-500 group-hover:text-chalk-400 transition-colors">{{ cat.description }}</div>
            <div class="text-xs text-chalk-600 mt-3 flex items-center gap-1">
              <span>{{ cat.count }} topics</span>
              <svg class="w-3 h-3 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
              </svg>
            </div>
          </div>
        </NuxtLink>
      </div>

      <div class="mt-12 text-center" v-scroll-animate="{ animation: 'fade-up', delay: 400 }">
        <NuxtLink to="/docs" class="group inline-flex items-center gap-2 px-6 py-3 bg-logic-navy-800/50 border border-logic-navy-600 rounded-xl text-chalk hover:bg-logic-navy-700 hover:border-logic-navy-500 transition-all duration-300 hover:scale-105">
          Explore all topics
          <svg class="w-4 h-4 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/></svg>
        </NuxtLink>
      </div>
    </div>
  </section>
</template>

<script setup>
import { useHomeData } from '~/composables/useHomeData'

const { categoryConfig } = useHomeData()

const categoryTopics = ref([])
const totalTopicCount = ref(0)

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

onMounted(async () => {
  try {
    const response = await fetch('/data/_topics.json')
    if (!response.ok) return

    const topics = await response.json()
    totalTopicCount.value = topics.length

    const grouped = {}

    for (const topic of topics) {
      for (const [catId, catConfig] of Object.entries(categoryConfig)) {
        const matches = catConfig.prefixes.some(prefix => topic.startsWith(prefix))
        if (matches) {
          if (!grouped[catId]) {
            grouped[catId] = { id: catId, ...catConfig, topics: [], firstTopic: null, count: 0 }
          }
          grouped[catId].topics.push(topic)
          grouped[catId].count++
          if (!grouped[catId].firstTopic) {
            grouped[catId].firstTopic = topic
          }
          break
        }
      }
    }

    categoryTopics.value = Object.values(grouped).sort((a, b) => a.order - b.order).slice(0, 8)
  } catch (e) {
    console.error('Failed to load topics:', e)
  }
})
</script>
