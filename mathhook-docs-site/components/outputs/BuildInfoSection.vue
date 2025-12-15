<template>
  <div
    v-scroll-animate="{ animation: 'fade-up', delay: 100 }"
    class="relative rounded-2xl p-8 border border-rust-core/30 bg-logic-navy-800/30 backdrop-blur-sm overflow-hidden"
  >
    <!-- Animated border glow -->
    <div
      class="absolute inset-0 rounded-2xl opacity-50"
      style="background: linear-gradient(90deg, transparent, rgba(230, 69, 36, 0.1), transparent); animation: shimmer 3s infinite;"
    ></div>

    <div class="relative z-10">
      <div class="flex items-center gap-4 mb-5">
        <div class="w-12 h-12 rounded-xl bg-rust-core/10 flex items-center justify-center animate-pulse-slow">
          <svg class="w-6 h-6 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
          </svg>
        </div>
        <div>
          <h3 class="text-xl font-semibold text-chalk">Build Your Own Outputs</h3>
          <p class="text-sm text-chalk-600">Generate documentation in any format</p>
        </div>
      </div>

      <p class="text-chalk-400 mb-6 max-w-2xl">
        All outputs are generated from YAML schemas using the MathHook KB CLI tool.
        You can build custom formats or regenerate existing ones.
      </p>

      <div class="relative rounded-xl bg-logic-navy-900/80 border border-logic-navy-700/50 p-5 font-mono text-sm overflow-hidden group">
        <code class="block text-chalk-300 leading-relaxed">
          <span class="text-chalk-600"># Generate all output formats from a schema</span><br/>
          <span class="text-solve-cyan">cd</span> <span class="text-chalk-400">/path/to/mathhook-kb</span><br/>
          <span class="text-solve-cyan">cargo run</span> <span class="text-rust-core">--release</span> <span class="text-rust-core">-p</span> <span class="text-amber-400">kb-cli</span> <span class="text-rust-core">--</span> build <span class="text-step-green">schemas/calculus/derivatives.yaml</span> <span class="text-rust-core">--generators</span> <span class="text-violet-400">all</span>
        </code>

        <!-- Copy button -->
        <button
          @click="copyCommand"
          class="absolute top-3 right-3 p-2 rounded-lg bg-logic-navy-700/50 hover:bg-logic-navy-700 text-chalk-500 hover:text-chalk transition-all duration-300 opacity-0 group-hover:opacity-100"
        >
          <svg v-if="!copied" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
          </svg>
          <svg v-else class="w-4 h-4 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
const copied = ref(false)

const copyCommand = async () => {
  const command = 'cd /path/to/mathhook-kb && cargo run --release -p kb-cli -- build schemas/calculus/derivatives.yaml --generators all'
  await navigator.clipboard.writeText(command)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
}

// Scroll animate directive
const vScrollAnimate = {
  mounted(el, binding) {
    const options = binding.value || {}
    const animation = options.animation || 'fade-up'
    const delay = options.delay || 0

    el.style.opacity = '0'
    if (animation === 'fade-up') {
      el.style.transform = 'translateY(40px)'
    }

    el.style.transition = `all 0.8s cubic-bezier(0.16, 1, 0.3, 1) ${delay}ms`

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            el.style.opacity = '1'
            el.style.transform = 'translateY(0)'
            observer.unobserve(el)
          }
        })
      },
      { threshold: 0.1, rootMargin: '0px 0px -50px 0px' }
    )

    observer.observe(el)
  }
}
</script>

<style scoped>
@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

@keyframes pulse-slow {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.7;
  }
}

.animate-pulse-slow {
  animation: pulse-slow 3s ease-in-out infinite;
}
</style>
