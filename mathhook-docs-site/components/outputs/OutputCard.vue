<template>
  <component
    :is="isExternalLink ? 'a' : linkUrl ? NuxtLink : 'div'"
    :to="!isExternalLink && linkUrl ? linkUrl : undefined"
    :href="isExternalLink ? linkUrl : undefined"
    :target="isExternalLink ? '_blank' : undefined"
    v-scroll-animate="{ animation: 'fade-up', delay: animationDelay }"
    class="group relative rounded-2xl p-6 border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500 overflow-hidden block cursor-pointer"
    :class="[
      linkUrl && !hasCustomAction ? 'hover:scale-[1.02] hover:shadow-2xl' : 'hover:shadow-2xl',
      shadowColorClass
    ]"
  >
    <!-- Hover glow effect -->
    <div
      class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none"
      :class="glowGradientClass"
    ></div>

    <div class="relative z-10">
      <div class="flex items-center justify-between gap-4 mb-4">
        <div class="flex items-center gap-4">
          <div
            class="w-14 h-14 rounded-xl flex items-center justify-center group-hover:scale-110 transition-all duration-300"
            :class="iconBgClass"
          >
            <slot name="icon">
              <component :is="icon" class="w-7 h-7" :class="iconColorClass" />
            </slot>
          </div>
          <div>
            <h3
              class="text-lg font-semibold transition-colors"
              :class="titleColorClass"
            >
              {{ title }}
            </h3>
            <p class="text-sm text-chalk-600">{{ subtitle }}</p>
          </div>
        </div>

        <!-- File count badge -->
        <div v-if="fileCount > 0" class="flex items-center gap-2">
          <span class="px-2 py-1 rounded-full text-xs font-medium" :class="badgeClass">
            {{ fileCount }} {{ fileCount === 1 ? 'file' : 'files' }}
          </span>
        </div>
      </div>

      <p class="text-chalk-400 mb-5">{{ description }}</p>

      <!-- Default link style (arrow) -->
      <div v-if="linkUrl && !hasCustomAction" class="flex items-center font-medium" :class="linkColorClass">
        {{ linkText }}
        <svg class="w-5 h-5 ml-2 group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
        </svg>
      </div>

      <!-- Custom action slot (for buttons, special cases) -->
      <slot name="action"></slot>
    </div>
  </component>
</template>

<script setup>
import { computed } from 'vue'
import { NuxtLink } from '#components'

const props = defineProps({
  title: { type: String, required: true },
  subtitle: { type: String, required: true },
  description: { type: String, required: true },
  linkUrl: { type: String, default: '' },
  linkText: { type: String, default: 'Open' },
  fileCount: { type: Number, default: 0 },
  color: {
    type: String,
    default: 'rust',
    validator: (v) => ['rust', 'amber', 'green', 'violet', 'cyan'].includes(v)
  },
  animationDelay: { type: Number, default: 100 },
  hasCustomAction: { type: Boolean, default: false },
  icon: { type: [Object, String], default: null }
})

const isExternalLink = computed(() => props.linkUrl?.startsWith('http'))

const colorClasses = {
  rust: {
    iconBg: 'bg-rust-core/10 group-hover:bg-rust-core/20',
    iconColor: 'text-rust-core',
    title: 'text-rust-core group-hover:text-rust-core-300',
    link: 'text-rust-core',
    glow: 'bg-gradient-to-br from-rust-core/10 via-transparent to-transparent',
    shadow: 'hover:shadow-rust-core/10',
    badge: 'bg-rust-core/10 text-rust-core'
  },
  amber: {
    iconBg: 'bg-amber-500/10 group-hover:bg-amber-500/20',
    iconColor: 'text-amber-500',
    title: 'text-amber-500 group-hover:text-amber-400',
    link: 'text-amber-500',
    glow: 'bg-gradient-to-br from-amber-500/10 via-transparent to-transparent',
    shadow: 'hover:shadow-amber-500/10',
    badge: 'bg-amber-500/10 text-amber-500'
  },
  green: {
    iconBg: 'bg-step-green/10 group-hover:bg-step-green/20',
    iconColor: 'text-step-green',
    title: 'text-step-green group-hover:text-step-green-300',
    link: 'text-step-green',
    glow: 'bg-gradient-to-br from-step-green/10 via-transparent to-transparent',
    shadow: 'hover:shadow-step-green/10',
    badge: 'bg-step-green/10 text-step-green'
  },
  violet: {
    iconBg: 'bg-violet-500/10 group-hover:bg-violet-500/20',
    iconColor: 'text-violet-500',
    title: 'text-violet-500 group-hover:text-violet-400',
    link: 'text-violet-500',
    glow: 'bg-gradient-to-br from-violet-500/10 via-transparent to-transparent',
    shadow: 'hover:shadow-violet-500/10',
    badge: 'bg-violet-500/10 text-violet-500'
  },
  cyan: {
    iconBg: 'bg-solve-cyan/10 group-hover:bg-solve-cyan/20',
    iconColor: 'text-solve-cyan',
    title: 'text-chalk group-hover:text-solve-cyan',
    link: 'text-solve-cyan',
    glow: 'bg-gradient-to-br from-solve-cyan/5 via-transparent to-transparent',
    shadow: 'hover:shadow-solve-cyan/10',
    badge: 'bg-solve-cyan/10 text-solve-cyan'
  }
}

const iconBgClass = computed(() => colorClasses[props.color].iconBg)
const iconColorClass = computed(() => colorClasses[props.color].iconColor)
const titleColorClass = computed(() => colorClasses[props.color].title)
const linkColorClass = computed(() => colorClasses[props.color].link)
const glowGradientClass = computed(() => colorClasses[props.color].glow)
const shadowColorClass = computed(() => colorClasses[props.color].shadow)
const badgeClass = computed(() => colorClasses[props.color].badge)

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
            el.style.transform = 'translateY(0) translateX(0) scale(1)'
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
