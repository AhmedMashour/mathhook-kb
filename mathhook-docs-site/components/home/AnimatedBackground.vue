<template>
  <div class="fixed inset-0 pointer-events-none overflow-hidden">
    <!-- Base dark layer -->
    <div class="absolute inset-0 bg-logic-navy-900"></div>

    <!-- Animated mesh gradient blobs -->
    <div class="absolute inset-0">
      <!-- Primary rust/orange blob - top left -->
      <div
        class="absolute w-[800px] h-[800px] rounded-full animate-blob-1"
        :style="{
          background: 'radial-gradient(circle at center, rgba(230, 69, 36, 0.45) 0%, rgba(230, 69, 36, 0.2) 40%, transparent 70%)',
          filter: 'blur(60px)',
          top: '-15%',
          left: '-10%',
          transform: `translate(${parallax.x * 0.03}px, ${parallax.y * 0.03}px)`
        }"
      ></div>

      <!-- Secondary cyan blob - bottom right -->
      <div
        class="absolute w-[700px] h-[700px] rounded-full animate-blob-2"
        :style="{
          background: 'radial-gradient(circle at center, rgba(6, 182, 212, 0.5) 0%, rgba(6, 182, 212, 0.2) 40%, transparent 70%)',
          filter: 'blur(60px)',
          bottom: '-10%',
          right: '-5%',
          transform: `translate(${parallax.x * -0.025}px, ${parallax.y * -0.025}px)`
        }"
      ></div>

      <!-- Tertiary purple/violet blob - center -->
      <div
        class="absolute w-[600px] h-[600px] rounded-full animate-blob-3"
        :style="{
          background: 'radial-gradient(circle at center, rgba(139, 92, 246, 0.4) 0%, rgba(139, 92, 246, 0.15) 40%, transparent 70%)',
          filter: 'blur(80px)',
          top: '30%',
          left: '40%',
          transform: `translate(${parallax.x * 0.02}px, ${parallax.y * -0.02}px)`
        }"
      ></div>

      <!-- Accent pink blob - top right -->
      <div
        class="absolute w-[500px] h-[500px] rounded-full animate-blob-4"
        :style="{
          background: 'radial-gradient(circle at center, rgba(236, 72, 153, 0.35) 0%, rgba(236, 72, 153, 0.1) 40%, transparent 70%)',
          filter: 'blur(70px)',
          top: '5%',
          right: '15%',
          transform: `translate(${parallax.x * -0.015}px, ${parallax.y * 0.015}px)`
        }"
      ></div>

      <!-- Accent green blob - bottom left -->
      <div
        class="absolute w-[450px] h-[450px] rounded-full animate-blob-5"
        :style="{
          background: 'radial-gradient(circle at center, rgba(34, 197, 94, 0.3) 0%, rgba(34, 197, 94, 0.1) 40%, transparent 70%)',
          filter: 'blur(60px)',
          bottom: '20%',
          left: '10%',
          transform: `translate(${parallax.x * 0.02}px, ${parallax.y * 0.02}px)`
        }"
      ></div>
    </div>

    <!-- Floating particles (client-only to avoid hydration mismatch) -->
    <ClientOnly>
      <div class="absolute inset-0">
        <div v-for="i in 30" :key="'particle-'+i"
             class="absolute rounded-full animate-float-particle"
             :style="{
               width: `${2 + Math.random() * 4}px`,
               height: `${2 + Math.random() * 4}px`,
               background: ['rgba(230, 69, 36, 0.6)', 'rgba(6, 182, 212, 0.6)', 'rgba(139, 92, 246, 0.5)', 'rgba(255, 255, 255, 0.4)'][Math.floor(Math.random() * 4)],
               left: `${Math.random() * 100}%`,
               top: `${Math.random() * 100}%`,
               animationDelay: `${Math.random() * 8}s`,
               animationDuration: `${8 + Math.random() * 12}s`
             }"
        ></div>
      </div>
    </ClientOnly>

    <!-- Subtle grid overlay -->
    <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:60px_60px]"></div>

    <!-- Top fade for readability -->
    <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900/50 via-transparent to-logic-navy-900/70"></div>
  </div>
</template>

<script setup>
const parallax = ref({ x: 0, y: 0 })

const handleMouseMove = (e) => {
  parallax.value = {
    x: (e.clientX - window.innerWidth / 2) / 50,
    y: (e.clientY - window.innerHeight / 2) / 50
  }
}

onMounted(() => {
  window.addEventListener('mousemove', handleMouseMove, { passive: true })
})

onUnmounted(() => {
  window.removeEventListener('mousemove', handleMouseMove)
})
</script>

<style scoped>
/* Blob animations */
@keyframes blob-1 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  25% { transform: translate(80px, -60px) scale(1.1); }
  50% { transform: translate(40px, 80px) scale(0.95); }
  75% { transform: translate(-60px, 40px) scale(1.05); }
}

@keyframes blob-2 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  25% { transform: translate(-70px, 50px) scale(1.05); }
  50% { transform: translate(60px, -40px) scale(1.1); }
  75% { transform: translate(30px, 70px) scale(0.95); }
}

@keyframes blob-3 {
  0%, 100% { transform: translate(0, 0) scale(1) rotate(0deg); }
  33% { transform: translate(50px, 60px) scale(1.08) rotate(10deg); }
  66% { transform: translate(-40px, -50px) scale(0.92) rotate(-10deg); }
}

@keyframes blob-4 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  20% { transform: translate(-50px, -30px) scale(1.12); }
  40% { transform: translate(40px, 50px) scale(0.9); }
  60% { transform: translate(70px, -20px) scale(1.05); }
  80% { transform: translate(-30px, 60px) scale(0.95); }
}

@keyframes blob-5 {
  0%, 100% { transform: translate(0, 0) scale(1) rotate(0deg); }
  25% { transform: translate(60px, 40px) scale(1.1) rotate(5deg); }
  50% { transform: translate(-50px, 70px) scale(0.95) rotate(-5deg); }
  75% { transform: translate(40px, -60px) scale(1.05) rotate(8deg); }
}

.animate-blob-1 { animation: blob-1 20s ease-in-out infinite; }
.animate-blob-2 { animation: blob-2 25s ease-in-out infinite; }
.animate-blob-3 { animation: blob-3 18s ease-in-out infinite; }
.animate-blob-4 { animation: blob-4 22s ease-in-out infinite; }
.animate-blob-5 { animation: blob-5 24s ease-in-out infinite; }

/* Floating particles animation */
@keyframes float-particle {
  0%, 100% {
    transform: translateY(0) translateX(0);
    opacity: 0;
  }
  10% { opacity: 1; }
  90% { opacity: 1; }
  100% {
    transform: translateY(-100vh) translateX(50px);
    opacity: 0;
  }
}

.animate-float-particle {
  animation: float-particle 15s ease-in-out infinite;
}
</style>
