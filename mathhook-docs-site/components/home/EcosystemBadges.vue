<template>
  <section class="py-12 px-4 relative z-10" v-scroll-animate="{ animation: 'fade-up', delay: 0 }">
    <div class="max-w-4xl mx-auto">
      <div class="flex flex-wrap items-center justify-center gap-4">
        <a
          v-for="(badge, index) in ecosystemBadges"
          :key="badge.name"
          :href="badge.url"
          target="_blank"
          class="inline-flex items-center gap-2 px-5 py-2.5 bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl text-sm text-chalk-300 hover:border-opacity-100 transition-all duration-300 hover:scale-105 hover:-translate-y-1 backdrop-blur-sm"
          :class="`hover:border-${badge.color}/50 hover:shadow-lg hover:shadow-${badge.color}/10`"
          :style="{ transitionDelay: `${index * 50}ms` }"
        >
          <span class="text-lg">{{ badge.emoji }}</span>
          {{ badge.name }}
        </a>
      </div>
    </div>
  </section>
</template>

<script setup>
import { useHomeData } from '~/composables/useHomeData'

const { ecosystemBadges } = useHomeData()

// Scroll animation directive
const vScrollAnimate = {
  mounted(el, binding) {
    const options = binding.value || {}
    const delay = options.delay || 0

    el.style.opacity = '0'
    el.style.transform = 'translateY(30px)'
    el.style.transition = `all 0.8s cubic-bezier(0.16, 1, 0.3, 1) ${delay}ms`

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          setTimeout(() => {
            el.style.opacity = '1'
            el.style.transform = 'translateY(0)'
          }, 50)
          observer.unobserve(el)
        }
      })
    }, { threshold: 0.1, rootMargin: '50px' })

    observer.observe(el)
  }
}
</script>
