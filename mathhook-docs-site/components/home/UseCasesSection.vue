<template>
  <section class="py-24 px-4 relative z-10">
    <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900 via-logic-navy-800/30 to-logic-navy-900 pointer-events-none"></div>

    <div class="max-w-6xl mx-auto relative">
      <div class="text-center mb-16" v-scroll-animate="{ animation: 'fade-up' }">
        <span class="inline-block px-4 py-1.5 bg-violet-500/10 border border-violet-500/20 rounded-full text-xs text-violet-400 font-medium mb-4">USE CASES</span>
        <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Built For</h2>
        <p class="text-chalk-400 text-lg">From research to production</p>
      </div>

      <div class="grid md:grid-cols-4 gap-8">
        <div
          v-for="(useCase, index) in useCases"
          :key="useCase.title"
          class="text-center group cursor-default"
          v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
        >
          <div
            class="w-16 h-16 mx-auto mb-5 rounded-2xl flex items-center justify-center transition-all duration-500 group-hover:scale-110 group-hover:rotate-6 group-hover:shadow-xl"
            :class="`bg-${useCase.color}/10 group-hover:bg-${useCase.color}/20 group-hover:shadow-${useCase.color}/20`"
          >
            <svg :class="`w-8 h-8 text-${useCase.color}`" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="useCase.iconPath"/>
            </svg>
          </div>
          <h3 class="font-semibold text-chalk mb-2 group-hover:text-white transition-colors">{{ useCase.title }}</h3>
          <p class="text-sm text-chalk-500 group-hover:text-chalk-400 transition-colors">{{ useCase.description }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
import { useHomeData } from '~/composables/useHomeData'

const { useCases } = useHomeData()

// Scroll animation directive
const vScrollAnimate = {
  mounted(el, binding) {
    const options = binding.value || {}
    const animation = options.animation || 'fade-up'
    const delay = options.delay || 0

    el.style.opacity = '0'
    el.style.transform = animation === 'fade-up' ? 'translateY(30px)' : 'translateY(30px)'
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
