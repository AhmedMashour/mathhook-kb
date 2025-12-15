<template>
  <section class="py-24 px-4 relative z-10">
    <div class="max-w-6xl mx-auto">
      <div class="text-center mb-16" v-scroll-animate="{ animation: 'fade-up' }">
        <span class="inline-block px-4 py-1.5 bg-rust-core/10 border border-rust-core/20 rounded-full text-xs text-rust-core font-medium mb-4">FEATURES</span>
        <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Key Features</h2>
        <p class="text-chalk-400 max-w-2xl mx-auto text-lg">A high-performance educational computer algebra system</p>
      </div>

      <div class="grid md:grid-cols-4 gap-5">
        <div
          v-for="(feature, index) in features"
          :key="feature.title"
          class="group bg-logic-navy-800/30 border border-logic-navy-700/50 rounded-2xl p-6 transition-all duration-500 hover:bg-logic-navy-800/50 hover:-translate-y-2 hover:shadow-xl cursor-default"
          :class="`hover:border-${feature.color}/50 hover:shadow-${feature.color}/10`"
          v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
        >
          <div
            class="w-12 h-12 rounded-xl flex items-center justify-center mb-4 transition-transform duration-300 group-hover:scale-110 group-hover:rotate-3"
            :class="`bg-gradient-to-br from-${feature.color}/20 to-${feature.color}/5`"
          >
            <span class="text-xl">{{ feature.icon }}</span>
          </div>
          <h3 class="text-lg font-semibold text-chalk mb-2 group-hover:text-white transition-colors">{{ feature.title }}</h3>
          <p class="text-chalk-500 text-sm leading-relaxed group-hover:text-chalk-400 transition-colors">
            {{ feature.description }}
          </p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
import { useHomeData } from '~/composables/useHomeData'

const { features } = useHomeData()

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
</script>
