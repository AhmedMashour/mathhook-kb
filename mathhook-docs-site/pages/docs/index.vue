<template>
  <div class="min-h-screen bg-slate-50 dark:bg-slate-900">
    <!-- Header -->
    <header class="bg-white dark:bg-slate-800 shadow-sm">
      <div class="container mx-auto px-4 py-6">
        <div class="flex items-center justify-between">
          <NuxtLink to="/" class="flex items-center space-x-2">
            <span class="text-2xl font-bold text-blue-600">MathHook</span>
          </NuxtLink>
          <nav class="flex gap-6">
            <NuxtLink to="/docs" class="text-gray-600 dark:text-gray-300 hover:text-blue-600">
              Docs
            </NuxtLink>
            <a href="https://github.com/mathhook/mathhook" target="_blank" class="text-gray-600 dark:text-gray-300 hover:text-blue-600">
              GitHub
            </a>
          </nav>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <div class="container mx-auto px-4 py-12">
      <div class="max-w-4xl mx-auto">
        <h1 class="text-4xl font-bold mb-8 text-gray-800 dark:text-white">
          Documentation
        </h1>

        <div class="space-y-6">
          <!-- ODE Section -->
          <section class="bg-white dark:bg-slate-800 rounded-lg shadow-md p-6">
            <h2 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-white">
              Ordinary Differential Equations
            </h2>
            <div class="space-y-3">
              <NuxtLink
                to="/docs/ode-first_order-separable"
                class="block p-4 bg-blue-50 dark:bg-slate-700 rounded-lg hover:bg-blue-100 dark:hover:bg-slate-600 transition-colors"
              >
                <h3 class="font-semibold text-lg text-blue-600 dark:text-blue-400">
                  Separable Differential Equations
                </h3>
                <p class="text-gray-600 dark:text-gray-300 mt-1">
                  Solve first-order ODEs using separation of variables
                </p>
              </NuxtLink>
            </div>
          </section>

          <!-- More sections can be added here -->
          <section class="bg-gray-50 dark:bg-slate-700 rounded-lg p-6 text-center">
            <p class="text-gray-600 dark:text-gray-300">
              More topics will be added soon. Check back regularly for updates!
            </p>
          </section>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
useHead({
  title: 'Documentation - MathHook Knowledge Base',
  meta: [
    {
      name: 'description',
      content: 'Browse MathHook mathematical documentation covering calculus, algebra, ODEs, and more'
    }
  ]
})
</script>
