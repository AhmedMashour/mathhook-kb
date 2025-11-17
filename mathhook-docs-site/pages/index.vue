<template>
  <div class="min-h-screen">
    <!-- Hero Section -->
    <header class="bg-gradient-to-r from-blue-600 to-purple-600 text-white py-20">
      <div class="container mx-auto px-4">
        <div class="max-w-4xl mx-auto text-center">
          <h1 class="text-5xl font-bold mb-6">
            MathHook Knowledge Base
          </h1>
          <p class="text-xl mb-8 text-blue-100">
            Interactive mathematical documentation powered by a high-performance computer algebra system
          </p>
          <div class="flex gap-4 justify-center">
            <NuxtLink
              to="/docs"
              class="bg-white text-blue-600 px-8 py-3 rounded-lg font-semibold hover:bg-blue-50 transition-colors"
            >
              Browse Documentation
            </NuxtLink>
            <a
              href="https://github.com/mathhook/mathhook"
              target="_blank"
              class="bg-blue-700 text-white px-8 py-3 rounded-lg font-semibold hover:bg-blue-800 transition-colors"
            >
              View on GitHub
            </a>
          </div>
        </div>
      </div>
    </header>

    <!-- Features Section -->
    <section class="py-16 container mx-auto px-4">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl font-bold text-center mb-12 text-gray-800 dark:text-white">
          Features
        </h2>
        <div class="grid md:grid-cols-3 gap-8">
          <div class="bg-white dark:bg-slate-800 p-6 rounded-lg shadow-lg">
            <div class="text-4xl mb-4">🎓</div>
            <h3 class="text-xl font-semibold mb-3 text-gray-800 dark:text-white">
              Educational
            </h3>
            <p class="text-gray-600 dark:text-gray-300">
              Step-by-step explanations with interactive examples in Rust, Python, and JavaScript
            </p>
          </div>

          <div class="bg-white dark:bg-slate-800 p-6 rounded-lg shadow-lg">
            <div class="text-4xl mb-4">⚡</div>
            <h3 class="text-xl font-semibold mb-3 text-gray-800 dark:text-white">
              High Performance
            </h3>
            <p class="text-gray-600 dark:text-gray-300">
              Written in Rust for blazing-fast symbolic computation and equation solving
            </p>
          </div>

          <div class="bg-white dark:bg-slate-800 p-6 rounded-lg shadow-lg">
            <div class="text-4xl mb-4">🔬</div>
            <h3 class="text-xl font-semibold mb-3 text-gray-800 dark:text-white">
              Comprehensive
            </h3>
            <p class="text-gray-600 dark:text-gray-300">
              Covers calculus, algebra, ODEs, PDEs, and more with rigorous mathematical correctness
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Topics Preview -->
    <section class="py-16 bg-white dark:bg-slate-800">
      <div class="container mx-auto px-4">
        <div class="max-w-6xl mx-auto">
          <h2 class="text-3xl font-bold text-center mb-12 text-gray-800 dark:text-white">
            Available Topics
          </h2>
          <div class="grid md:grid-cols-2 gap-6">
            <NuxtLink
              to="/docs/ode-first_order-separable"
              class="group bg-gradient-to-br from-blue-50 to-purple-50 dark:from-slate-700 dark:to-slate-600 p-6 rounded-lg hover:shadow-xl transition-all"
            >
              <h3 class="text-xl font-semibold mb-2 text-gray-800 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400">
                Separable Differential Equations
              </h3>
              <p class="text-gray-600 dark:text-gray-300">
                Learn to solve ODEs using separation of variables
              </p>
            </NuxtLink>

            <div class="bg-gradient-to-br from-gray-50 to-gray-100 dark:from-slate-700 dark:to-slate-600 p-6 rounded-lg opacity-75">
              <h3 class="text-xl font-semibold mb-2 text-gray-600 dark:text-gray-400">
                More topics coming soon...
              </h3>
              <p class="text-gray-500 dark:text-gray-400">
                Additional mathematical topics are being added regularly
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
useHead({
  title: 'Home - MathHook Knowledge Base',
  meta: [
    {
      name: 'description',
      content: 'Interactive mathematical documentation powered by MathHook CAS - high-performance symbolic mathematics in Rust'
    }
  ]
})
</script>
