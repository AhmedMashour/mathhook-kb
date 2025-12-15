<template>
  <section class="py-24 px-4 relative z-10">
    <div class="max-w-4xl mx-auto">
      <div class="text-center mb-14" v-scroll-animate="{ animation: 'fade-up' }">
        <span class="inline-block px-4 py-1.5 bg-amber-500/10 border border-amber-500/20 rounded-full text-xs text-amber-400 font-medium mb-4">INSTALLATION</span>
        <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Get Started in Seconds</h2>
        <p class="text-chalk-400 text-lg">Choose your language and start computing</p>
      </div>

      <div class="grid md:grid-cols-3 gap-6">
        <div
          v-for="(install, index) in installCommands"
          :key="install.lang"
          class="group bg-logic-navy-800/50 border border-logic-navy-700/50 rounded-2xl p-6 transition-all duration-500 hover:-translate-y-2 hover:shadow-xl backdrop-blur-sm"
          :class="`hover:border-${install.color}/50 hover:shadow-${install.color}/10`"
          v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
        >
          <div class="flex items-center gap-3 mb-5">
            <span class="text-3xl transition-transform duration-300 group-hover:scale-110">{{ install.emoji }}</span>
            <span :class="`text-${install.color} font-semibold text-lg`">{{ install.lang }}</span>
          </div>
          <div
            class="relative bg-logic-navy-900/80 rounded-xl p-4 font-mono text-sm cursor-pointer group/code"
            @click="copyCommand(install.command)"
          >
            <code class="text-chalk-300">{{ install.command }}</code>
            <div class="absolute right-3 top-1/2 -translate-y-1/2 opacity-0 group-hover/code:opacity-100 transition-opacity">
              <svg v-if="!copiedCommand || copiedCommand !== install.command" class="w-5 h-5 text-chalk-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
              </svg>
              <svg v-else class="w-5 h-5 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
              </svg>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
import { useHomeData } from '~/composables/useHomeData'

const { installCommands } = useHomeData()
const copiedCommand = ref(null)

const copyCommand = async (command) => {
  await navigator.clipboard.writeText(command)
  copiedCommand.value = command
  setTimeout(() => {
    copiedCommand.value = null
  }, 2000)
}

// Scroll animation directive
const vScrollAnimate = {
  mounted(el, binding) {
    const options = binding.value || {}
    const animation = options.animation || 'fade-up'
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
