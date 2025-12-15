<template>
  <div
    v-scroll-animate="{ animation: 'fade-up', delay: animationDelay }"
    class="group relative rounded-2xl border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm transition-all duration-500 overflow-hidden"
    :class="[
      shadowColorClass,
      isExpanded ? 'hover:shadow-2xl' : 'hover:scale-[1.02] hover:shadow-2xl'
    ]"
  >
    <!-- Hover glow effect -->
    <div
      class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none"
      :class="glowGradientClass"
    ></div>

    <!-- Card Header (always visible) -->
    <div class="relative z-10 p-6">
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
        <div v-if="files.length > 0" class="flex items-center gap-2">
          <span class="px-2 py-1 rounded-full text-xs font-medium" :class="badgeClass">
            {{ files.length }} {{ files.length === 1 ? 'file' : 'files' }}
          </span>
        </div>
      </div>

      <p class="text-chalk-400 mb-5">{{ description }}</p>

      <!-- Action buttons -->
      <div class="flex items-center gap-3 flex-wrap">
        <!-- Primary link (if not expandable or always show) -->
        <a
          v-if="linkUrl"
          :href="linkUrl"
          target="_blank"
          class="inline-flex items-center font-medium transition-colors"
          :class="linkColorClass"
        >
          {{ linkText }}
          <svg class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
        </a>

        <!-- External action (e.g., Open in Colab) -->
        <a
          v-if="externalActionUrl"
          :href="externalActionUrl"
          target="_blank"
          class="inline-flex items-center px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-200"
          :class="externalButtonClass"
        >
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
          </svg>
          {{ externalActionText }}
        </a>

        <!-- Expand/Collapse button -->
        <button
          v-if="files.length > 0"
          @click="toggleExpand"
          class="inline-flex items-center px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-200"
          :class="expandButtonClass"
        >
          <svg
            class="w-4 h-4 mr-2 transition-transform duration-200"
            :class="{ 'rotate-180': isExpanded }"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
          </svg>
          {{ isExpanded ? 'Hide Files' : 'Browse Files' }}
        </button>
      </div>
    </div>

    <!-- Expandable File Browser Section -->
    <Transition name="expand-content">
      <div
        v-if="isExpanded && files.length > 0"
        class="relative z-10 border-t border-logic-navy-700/50 bg-logic-navy-900/30"
      >
        <div class="p-6 pt-4">
          <OutputsFileBrowser
            :files="files"
            :color="color"
            @download="handleDownload"
            @download-multiple="handleMultipleDownload"
          />
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const props = defineProps({
  title: { type: String, required: true },
  subtitle: { type: String, required: true },
  description: { type: String, required: true },
  linkUrl: { type: String, default: '' },
  linkText: { type: String, default: 'Open' },
  externalActionUrl: { type: String, default: '' },
  externalActionText: { type: String, default: 'Open External' },
  color: {
    type: String,
    default: 'rust',
    validator: (v) => ['rust', 'amber', 'green', 'violet', 'cyan'].includes(v)
  },
  animationDelay: { type: Number, default: 100 },
  icon: { type: [Object, String], default: null },
  files: { type: Array, default: () => [] },
  defaultExpanded: { type: Boolean, default: false }
})

const emit = defineEmits(['expand', 'collapse', 'download'])

// State
const isExpanded = ref(props.defaultExpanded)

// Color classes
const colorClasses = {
  rust: {
    iconBg: 'bg-rust-core/10 group-hover:bg-rust-core/20',
    iconColor: 'text-rust-core',
    title: 'text-rust-core group-hover:text-rust-core-300',
    link: 'text-rust-core hover:text-rust-core-300',
    glow: 'bg-gradient-to-br from-rust-core/10 via-transparent to-transparent',
    shadow: 'hover:shadow-rust-core/10',
    badge: 'bg-rust-core/10 text-rust-core',
    expandBtn: 'bg-logic-navy-700/50 text-chalk hover:bg-logic-navy-600/50 border border-logic-navy-600/50',
    externalBtn: 'bg-rust-core/10 text-rust-core hover:bg-rust-core/20 border border-rust-core/30'
  },
  amber: {
    iconBg: 'bg-amber-500/10 group-hover:bg-amber-500/20',
    iconColor: 'text-amber-500',
    title: 'text-amber-500 group-hover:text-amber-400',
    link: 'text-amber-500 hover:text-amber-400',
    glow: 'bg-gradient-to-br from-amber-500/10 via-transparent to-transparent',
    shadow: 'hover:shadow-amber-500/10',
    badge: 'bg-amber-500/10 text-amber-500',
    expandBtn: 'bg-logic-navy-700/50 text-chalk hover:bg-logic-navy-600/50 border border-logic-navy-600/50',
    externalBtn: 'bg-amber-500/10 text-amber-500 hover:bg-amber-500/20 border border-amber-500/30'
  },
  green: {
    iconBg: 'bg-step-green/10 group-hover:bg-step-green/20',
    iconColor: 'text-step-green',
    title: 'text-step-green group-hover:text-step-green-300',
    link: 'text-step-green hover:text-step-green-300',
    glow: 'bg-gradient-to-br from-step-green/10 via-transparent to-transparent',
    shadow: 'hover:shadow-step-green/10',
    badge: 'bg-step-green/10 text-step-green',
    expandBtn: 'bg-logic-navy-700/50 text-chalk hover:bg-logic-navy-600/50 border border-logic-navy-600/50',
    externalBtn: 'bg-step-green/10 text-step-green hover:bg-step-green/20 border border-step-green/30'
  },
  violet: {
    iconBg: 'bg-violet-500/10 group-hover:bg-violet-500/20',
    iconColor: 'text-violet-500',
    title: 'text-violet-500 group-hover:text-violet-400',
    link: 'text-violet-500 hover:text-violet-400',
    glow: 'bg-gradient-to-br from-violet-500/10 via-transparent to-transparent',
    shadow: 'hover:shadow-violet-500/10',
    badge: 'bg-violet-500/10 text-violet-500',
    expandBtn: 'bg-logic-navy-700/50 text-chalk hover:bg-logic-navy-600/50 border border-logic-navy-600/50',
    externalBtn: 'bg-violet-500/10 text-violet-500 hover:bg-violet-500/20 border border-violet-500/30'
  },
  cyan: {
    iconBg: 'bg-solve-cyan/10 group-hover:bg-solve-cyan/20',
    iconColor: 'text-solve-cyan',
    title: 'text-chalk group-hover:text-solve-cyan',
    link: 'text-solve-cyan hover:text-solve-cyan-300',
    glow: 'bg-gradient-to-br from-solve-cyan/5 via-transparent to-transparent',
    shadow: 'hover:shadow-solve-cyan/10',
    badge: 'bg-solve-cyan/10 text-solve-cyan',
    expandBtn: 'bg-logic-navy-700/50 text-chalk hover:bg-logic-navy-600/50 border border-logic-navy-600/50',
    externalBtn: 'bg-solve-cyan/10 text-solve-cyan hover:bg-solve-cyan/20 border border-solve-cyan/30'
  }
}

const iconBgClass = computed(() => colorClasses[props.color].iconBg)
const iconColorClass = computed(() => colorClasses[props.color].iconColor)
const titleColorClass = computed(() => colorClasses[props.color].title)
const linkColorClass = computed(() => colorClasses[props.color].link)
const glowGradientClass = computed(() => colorClasses[props.color].glow)
const shadowColorClass = computed(() => colorClasses[props.color].shadow)
const badgeClass = computed(() => colorClasses[props.color].badge)
const expandButtonClass = computed(() => colorClasses[props.color].expandBtn)
const externalButtonClass = computed(() => colorClasses[props.color].externalBtn)

// Methods
function toggleExpand() {
  isExpanded.value = !isExpanded.value
  emit(isExpanded.value ? 'expand' : 'collapse')
}

function handleDownload(file) {
  emit('download', file)
}

function handleMultipleDownload(files) {
  emit('download', files)
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

<style scoped>
.expand-content-enter-active,
.expand-content-leave-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  overflow: hidden;
}

.expand-content-enter-from,
.expand-content-leave-to {
  opacity: 0;
  max-height: 0;
}

.expand-content-enter-to,
.expand-content-leave-from {
  opacity: 1;
  max-height: 800px;
}
</style>
