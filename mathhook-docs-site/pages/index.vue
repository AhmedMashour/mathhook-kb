<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden">
    <!-- Animated Background -->
    <HomeAnimatedBackground />

    <!-- Navigation -->
    <HomeAppNavbar />

    <!-- Hero Section -->
    <HomeHeroSection />

    <!-- Ecosystem Badges -->
    <HomeEcosystemBadges />

    <!-- Key Features -->
    <HomeFeaturesSection />

    <!-- Performance Benchmarks -->
    <HomePerformanceSection />

    <!-- Topics Coverage -->
    <HomeTopicsSection />

    <!-- Use Cases -->
    <HomeUseCasesSection />

    <!-- Quick Install -->
    <HomeInstallSection />

    <!-- CTA Section -->
    <HomeCtaSection />

    <!-- Footer -->
    <HomeAppFooter />
  </div>
</template>

<script setup>
useHead({
  title: 'MathHook - High-Performance Computer Algebra System',
  meta: [
    { name: 'description', content: 'MathHook: High-performance CAS in Rust with Python and Node.js bindings. Symbolic Power. Educational Clarity. Native Speed.' }
  ]
})
</script>
