<template>
  <section class="py-24 px-4 relative z-10">
    <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900 via-logic-navy-800/50 to-logic-navy-900 pointer-events-none"></div>

    <div class="max-w-5xl mx-auto relative">
      <div class="text-center mb-16" v-scroll-animate="{ animation: 'fade-up' }">
        <span class="inline-block px-4 py-1.5 bg-solve-cyan/10 border border-solve-cyan/20 rounded-full text-xs text-solve-cyan font-medium mb-4">PERFORMANCE</span>
        <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Blazing Fast</h2>
        <p class="text-chalk-400 text-lg">Nanosecond-to-microsecond performance for symbolic operations</p>
      </div>

      <div class="grid md:grid-cols-2 gap-10 items-start">
        <!-- Benchmark Table -->
        <div
          class="bg-logic-navy-800/50 border border-logic-navy-700/50 rounded-2xl overflow-hidden backdrop-blur-sm"
          v-scroll-animate="{ animation: 'fade-right' }"
        >
          <div class="px-6 py-4 border-b border-logic-navy-700/50 bg-logic-navy-800/30">
            <h3 class="font-semibold text-chalk text-lg">Operation Benchmarks</h3>
          </div>
          <div class="divide-y divide-logic-navy-700/30">
            <div
              v-for="(benchmark, index) in benchmarks"
              :key="benchmark.name"
              class="flex justify-between items-center px-6 py-4 hover:bg-logic-navy-700/20 transition-colors"
              v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
            >
              <span class="text-chalk-400 text-sm">{{ benchmark.name }}</span>
              <span :class="`text-${benchmark.color} font-mono font-bold`">{{ benchmark.time }}</span>
            </div>
          </div>
          <div class="px-6 py-3 bg-logic-navy-900/50 text-xs text-chalk-600 flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
            Benchmarked with Criterion.rs
          </div>
        </div>

        <!-- Why So Fast -->
        <div class="space-y-5" v-scroll-animate="{ animation: 'fade-left' }">
          <h3 class="font-semibold text-chalk text-xl mb-6">Why So Fast?</h3>
          <div
            v-for="(reason, index) in performanceReasons"
            :key="reason.title"
            class="flex items-start gap-4 group"
            v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
          >
            <div
              class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 transition-all duration-300 group-hover:scale-110"
              :class="`bg-${reason.color}/10`"
            >
              <span :class="`text-${reason.color} text-sm font-bold`">{{ reason.icon }}</span>
            </div>
            <div>
              <div class="text-chalk font-medium group-hover:text-white transition-colors">{{ reason.title }}</div>
              <div class="text-chalk-500 text-sm group-hover:text-chalk-400 transition-colors">{{ reason.description }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
import { useHomeData } from '~/composables/useHomeData'

const { benchmarks, performanceReasons } = useHomeData()

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
