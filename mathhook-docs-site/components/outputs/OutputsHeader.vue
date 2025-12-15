<template>
  <div class="mb-14" v-scroll-animate="{ animation: 'fade-up' }">
    <div class="relative inline-block mb-4">
      <h1 class="text-4xl md:text-5xl font-bold">
        <span class="relative">
          <span
            v-for="(char, i) in 'Documentation'.split('')"
            :key="'doc-' + i"
            class="inline-block animate-text-reveal"
            :style="{ animationDelay: `${i * 0.03}s` }"
          >{{ char }}</span>
        </span>
        <span class="relative ml-2">
          <span
            v-for="(char, i) in 'Outputs'.split('')"
            :key="'out-' + i"
            class="inline-block bg-clip-text text-transparent bg-gradient-to-r from-rust-core to-solve-cyan animate-text-reveal"
            :style="{ animationDelay: `${0.4 + i * 0.03}s` }"
          >{{ char }}</span>
        </span>
      </h1>
      <!-- Animated underline -->
      <div class="absolute -bottom-2 left-0 h-1 bg-gradient-to-r from-rust-core via-violet-500 to-solve-cyan rounded-full animate-expand-width"></div>
    </div>
    <p class="text-xl text-chalk-400 max-w-2xl animate-fade-in-delayed">
      Access MathHook documentation in your preferred format — from interactive notebooks to searchable books
    </p>
  </div>
</template>

<script setup>
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
@keyframes text-reveal {
  0% {
    opacity: 0;
    transform: translateY(20px) rotateX(-20deg);
  }
  100% {
    opacity: 1;
    transform: translateY(0) rotateX(0);
  }
}

.animate-text-reveal {
  animation: text-reveal 0.6s ease-out forwards;
  opacity: 0;
}

@keyframes expand-width {
  0% {
    width: 0;
    opacity: 0;
  }
  100% {
    width: 100%;
    opacity: 1;
  }
}

.animate-expand-width {
  animation: expand-width 1s ease-out 0.8s forwards;
  width: 0;
}

@keyframes fade-in-delayed {
  0% {
    opacity: 0;
    transform: translateY(10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in-delayed {
  animation: fade-in-delayed 0.8s ease-out 0.6s forwards;
  opacity: 0;
}
</style>
