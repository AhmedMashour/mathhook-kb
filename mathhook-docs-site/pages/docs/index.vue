<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk">
    <!-- Navigation -->
    <nav class="sticky top-0 z-50 bg-logic-navy-900/80 backdrop-blur-lg border-b border-logic-navy-700/50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <NuxtLink to="/" class="flex items-center gap-2">
            <svg class="w-7 h-7" viewBox="0 0 48 48" fill="none">
              <defs>
                <linearGradient id="nav-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#E64524"/>
                  <stop offset="100%" stop-color="#06B6D4"/>
                </linearGradient>
              </defs>
              <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#nav-gradient)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <span class="text-lg font-semibold">
              <span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span>
            </span>
          </NuxtLink>
          <div class="flex items-center gap-6">
            <NuxtLink to="/docs" class="text-solve-cyan font-medium">Docs</NuxtLink>
            <NuxtLink to="/outputs" class="nav-link">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="nav-link flex items-center gap-1">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <div class="max-w-5xl mx-auto px-4 py-12">
      <div class="mb-12">
        <h1 class="text-4xl md:text-5xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-rust-core to-solve-cyan">
          Documentation
        </h1>
        <p class="text-xl text-chalk-500">
          Explore comprehensive mathematical documentation with interactive examples
        </p>
      </div>

      <div class="space-y-8">
        <!-- Getting Started Section -->
        <section class="bg-gradient-to-r from-rust-core to-solve-cyan rounded-2xl p-1">
          <div class="bg-logic-navy-800 rounded-xl p-6">
            <h2 class="text-2xl font-semibold mb-4 text-chalk flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-rust-core/20 flex items-center justify-center">
                <svg class="w-5 h-5 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                </svg>
              </div>
              Getting Started
            </h2>
            <div class="space-y-3">
              <NuxtLink
                to="/docs/getting-started-quick-start"
                class="block p-4 bg-logic-navy-700/50 rounded-lg hover:bg-logic-navy-700 transition-colors group"
              >
                <h3 class="font-semibold text-lg text-solve-cyan group-hover:text-solve-cyan-300 transition-colors">
                  Quick Start Guide
                </h3>
                <p class="text-chalk-500 mt-1">
                  Get started with MathHook in Python, Rust, or JavaScript - installation, basic usage, and key features
                </p>
              </NuxtLink>
            </div>
          </div>
        </section>

        <!-- ODE Section -->
        <section class="card p-6">
          <h2 class="text-2xl font-semibold mb-4 text-chalk flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-solve-cyan/10 flex items-center justify-center">
              <svg class="w-5 h-5 text-solve-cyan" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z"/>
              </svg>
            </div>
            Ordinary Differential Equations
          </h2>
          <div class="space-y-3">
            <NuxtLink
              to="/docs/ode-first_order-separable"
              class="block p-4 bg-solve-cyan/5 rounded-lg hover:bg-solve-cyan/10 transition-colors group border border-solve-cyan/10 hover:border-solve-cyan/30"
            >
              <h3 class="font-semibold text-lg text-solve-cyan group-hover:text-solve-cyan-300 transition-colors">
                Separable Differential Equations
              </h3>
              <p class="text-chalk-500 mt-1">
                Solve first-order ODEs using separation of variables
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/ode-first_order-linear"
              class="block p-4 bg-solve-cyan/5 rounded-lg hover:bg-solve-cyan/10 transition-colors group border border-solve-cyan/10 hover:border-solve-cyan/30"
            >
              <h3 class="font-semibold text-lg text-solve-cyan group-hover:text-solve-cyan-300 transition-colors">
                Linear First-Order Differential Equations
              </h3>
              <p class="text-chalk-500 mt-1">
                Solve linear ODEs using the integrating factor method
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/ode-first_order-bernoulli"
              class="block p-4 bg-solve-cyan/5 rounded-lg hover:bg-solve-cyan/10 transition-colors group border border-solve-cyan/10 hover:border-solve-cyan/30"
            >
              <h3 class="font-semibold text-lg text-solve-cyan group-hover:text-solve-cyan-300 transition-colors">
                Bernoulli Differential Equations
              </h3>
              <p class="text-chalk-500 mt-1">
                Solve nonlinear ODEs using substitution methods
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/ode-first_order-exact"
              class="block p-4 bg-solve-cyan/5 rounded-lg hover:bg-solve-cyan/10 transition-colors group border border-solve-cyan/10 hover:border-solve-cyan/30"
            >
              <h3 class="font-semibold text-lg text-solve-cyan group-hover:text-solve-cyan-300 transition-colors">
                Exact Differential Equations
              </h3>
              <p class="text-chalk-500 mt-1">
                Solve exact equations using potential function methods
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/ode-first_order-homogeneous"
              class="block p-4 bg-solve-cyan/5 rounded-lg hover:bg-solve-cyan/10 transition-colors group border border-solve-cyan/10 hover:border-solve-cyan/30"
            >
              <h3 class="font-semibold text-lg text-solve-cyan group-hover:text-solve-cyan-300 transition-colors">
                Homogeneous Differential Equations
              </h3>
              <p class="text-chalk-500 mt-1">
                Solve homogeneous ODEs using v = y/x substitution
              </p>
            </NuxtLink>
          </div>
        </section>

        <!-- Calculus Section -->
        <section class="card p-6">
          <h2 class="text-2xl font-semibold mb-4 text-chalk flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-rust-core/10 flex items-center justify-center">
              <svg class="w-5 h-5 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
              </svg>
            </div>
            Calculus
          </h2>
          <div class="space-y-3">
            <NuxtLink
              to="/docs/calculus-derivatives"
              class="block p-4 bg-rust-core/5 rounded-lg hover:bg-rust-core/10 transition-colors group border border-rust-core/10 hover:border-rust-core/30"
            >
              <h3 class="font-semibold text-lg text-rust-core group-hover:text-rust-core-300 transition-colors">
                Symbolic Differentiation
              </h3>
              <p class="text-chalk-500 mt-1">
                Compute derivatives using power rule, chain rule, product rule, and more
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/calculus-integration"
              class="block p-4 bg-rust-core/5 rounded-lg hover:bg-rust-core/10 transition-colors group border border-rust-core/10 hover:border-rust-core/30"
            >
              <h3 class="font-semibold text-lg text-rust-core group-hover:text-rust-core-300 transition-colors">
                Symbolic Integration
              </h3>
              <p class="text-chalk-500 mt-1">
                Compute integrals using substitution, integration by parts, and more
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/calculus-limits"
              class="block p-4 bg-rust-core/5 rounded-lg hover:bg-rust-core/10 transition-colors group border border-rust-core/10 hover:border-rust-core/30"
            >
              <h3 class="font-semibold text-lg text-rust-core group-hover:text-rust-core-300 transition-colors">
                Limits and Continuity
              </h3>
              <p class="text-chalk-500 mt-1">
                Fundamental limit concepts, L'Hopital's rule, and continuity
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/calculus-series"
              class="block p-4 bg-rust-core/5 rounded-lg hover:bg-rust-core/10 transition-colors group border border-rust-core/10 hover:border-rust-core/30"
            >
              <h3 class="font-semibold text-lg text-rust-core group-hover:text-rust-core-300 transition-colors">
                Series and Sequences
              </h3>
              <p class="text-chalk-500 mt-1">
                Taylor series, Fourier series, convergence tests, and power series
              </p>
            </NuxtLink>
          </div>
        </section>

        <!-- Algebra Section -->
        <section class="card p-6">
          <h2 class="text-2xl font-semibold mb-4 text-chalk flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-amber-500/10 flex items-center justify-center">
              <svg class="w-5 h-5 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"/>
              </svg>
            </div>
            Algebra
          </h2>
          <div class="space-y-3">
            <NuxtLink
              to="/docs/algebra-simplify"
              class="block p-4 bg-amber-500/5 rounded-lg hover:bg-amber-500/10 transition-colors group border border-amber-500/10 hover:border-amber-500/30"
            >
              <h3 class="font-semibold text-lg text-amber-500 group-hover:text-amber-400 transition-colors">
                Expression Simplification
              </h3>
              <p class="text-chalk-500 mt-1">
                Simplify algebraic expressions - combine like terms, factor, expand, rationalize
              </p>
            </NuxtLink>
          </div>
        </section>

        <!-- PDE Section -->
        <section class="card p-6">
          <h2 class="text-2xl font-semibold mb-4 text-chalk flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-step-green/10 flex items-center justify-center">
              <svg class="w-5 h-5 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
              </svg>
            </div>
            Partial Differential Equations
          </h2>
          <div class="space-y-3">
            <NuxtLink
              to="/docs/pde-heat-equation"
              class="block p-4 bg-step-green/5 rounded-lg hover:bg-step-green/10 transition-colors group border border-step-green/10 hover:border-step-green/30"
            >
              <h3 class="font-semibold text-lg text-step-green group-hover:text-step-green-300 transition-colors">
                The Heat Equation
              </h3>
              <p class="text-chalk-500 mt-1">
                Parabolic PDE modeling heat diffusion and other diffusion processes
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/pde-wave-equation"
              class="block p-4 bg-step-green/5 rounded-lg hover:bg-step-green/10 transition-colors group border border-step-green/10 hover:border-step-green/30"
            >
              <h3 class="font-semibold text-lg text-step-green group-hover:text-step-green-300 transition-colors">
                The Wave Equation
              </h3>
              <p class="text-chalk-500 mt-1">
                Hyperbolic PDE describing wave propagation in vibrating strings, sound, and light
              </p>
            </NuxtLink>

            <NuxtLink
              to="/docs/pde-laplace-equation"
              class="block p-4 bg-step-green/5 rounded-lg hover:bg-step-green/10 transition-colors group border border-step-green/10 hover:border-step-green/30"
            >
              <h3 class="font-semibold text-lg text-step-green group-hover:text-step-green-300 transition-colors">
                The Laplace Equation
              </h3>
              <p class="text-chalk-500 mt-1">
                Elliptic PDE describing steady-state potential fields and equilibrium
              </p>
            </NuxtLink>
          </div>
        </section>

        <!-- Special Functions Section -->
        <section class="card p-6">
          <h2 class="text-2xl font-semibold mb-4 text-chalk flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-violet-500/10 flex items-center justify-center">
              <svg class="w-5 h-5 text-violet-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/>
              </svg>
            </div>
            Special Functions
          </h2>
          <div class="space-y-3">
            <NuxtLink
              to="/docs/special_functions-gamma"
              class="block p-4 bg-violet-500/5 rounded-lg hover:bg-violet-500/10 transition-colors group border border-violet-500/10 hover:border-violet-500/30"
            >
              <h3 class="font-semibold text-lg text-violet-500 group-hover:text-violet-400 transition-colors">
                The Gamma Function
              </h3>
              <p class="text-chalk-500 mt-1">
                Generalization of factorial to complex numbers
              </p>
            </NuxtLink>
          </div>
        </section>

        <!-- Coming Soon Section -->
        <section class="card p-6 border-dashed border-logic-navy-600">
          <h2 class="text-2xl font-semibold mb-4 text-chalk-500 flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-logic-navy-700 flex items-center justify-center">
              <svg class="w-5 h-5 text-chalk-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
              </svg>
            </div>
            Additional Topics Coming Soon
          </h2>
          <div class="grid md:grid-cols-2 gap-4">
            <div class="p-4 bg-logic-navy-800/50 rounded-lg">
              <h3 class="font-semibold text-chalk-400 mb-2">
                More Special Functions
              </h3>
              <ul class="text-sm text-chalk-600 space-y-1">
                <li>Beta Function</li>
                <li>Bessel Functions</li>
                <li>Error Functions</li>
              </ul>
            </div>

            <div class="p-4 bg-logic-navy-800/50 rounded-lg">
              <h3 class="font-semibold text-chalk-400 mb-2">
                More Calculus
              </h3>
              <ul class="text-sm text-chalk-600 space-y-1">
                <li>Limits and Continuity</li>
                <li>Series and Sequences</li>
                <li>Multivariable Calculus</li>
              </ul>
            </div>

            <div class="p-4 bg-logic-navy-800/50 rounded-lg">
              <h3 class="font-semibold text-chalk-400 mb-2">
                More Algebra
              </h3>
              <ul class="text-sm text-chalk-600 space-y-1">
                <li>Equation Solving</li>
                <li>Polynomial Operations</li>
                <li>Matrix Algebra</li>
              </ul>
            </div>
          </div>
          <p class="text-center text-chalk-600 mt-6 text-sm">
            These topics are being actively developed. Check back regularly for updates!
          </p>
        </section>
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
      content: 'Browse MathHook mathematical documentation covering calculus, algebra, ODEs, PDEs, and more. Symbolic Power. Educational Clarity. Native Speed.'
    }
  ]
})
</script>
