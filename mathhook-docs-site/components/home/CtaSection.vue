<template>
  <section class="py-32 px-4 relative z-10 overflow-hidden">
    <!-- Animated background -->
    <div class="absolute inset-0 pointer-events-none">
      <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-gradient-conic from-rust-core/20 via-solve-cyan/20 to-step-green/20 rounded-full blur-[100px] animate-spin-slow"></div>
    </div>

    <div class="max-w-3xl mx-auto text-center relative" v-scroll-animate="{ animation: 'fade-up' }">
      <h2 class="text-4xl md:text-6xl font-bold text-chalk mb-6">Ready to Solve?</h2>
      <p class="text-chalk-400 mb-10 text-xl">Join developers using MathHook for high-performance symbolic math</p>
      <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
        <NuxtLink to="/docs" class="group relative px-10 py-4 bg-gradient-to-r from-rust-core via-solve-cyan to-step-green text-white font-semibold rounded-xl transition-all duration-300 hover:scale-105 hover:shadow-xl hover:shadow-solve-cyan/25 overflow-hidden">
          <span class="relative z-10">Read the Docs</span>
          <div class="absolute inset-0 bg-gradient-to-r from-step-green via-solve-cyan to-rust-core opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
        </NuxtLink>
        <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="group px-10 py-4 bg-logic-navy-800/80 border border-logic-navy-600 text-chalk font-semibold rounded-xl hover:bg-logic-navy-700 hover:border-logic-navy-500 transition-all duration-300 hover:scale-105 backdrop-blur-sm">
          View on GitHub
        </a>
      </div>
    </div>
  </section>
</template>

<script setup>
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

<style scoped>
/* Conic gradient */
.bg-gradient-conic {
  background: conic-gradient(from 0deg, var(--tw-gradient-stops));
}

/* Slow spin for CTA background */
@keyframes spin-slow {
  from { transform: translate(-50%, -50%) rotate(0deg); }
  to { transform: translate(-50%, -50%) rotate(360deg); }
}

.animate-spin-slow {
  animation: spin-slow 30s linear infinite;
}
</style>
