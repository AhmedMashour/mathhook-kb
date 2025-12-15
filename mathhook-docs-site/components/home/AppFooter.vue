<template>
  <footer class="py-16 px-4 border-t border-logic-navy-800/50 relative z-10">
    <div class="max-w-6xl mx-auto">
      <div class="flex flex-col md:flex-row items-center justify-between gap-8">
        <div class="flex items-center gap-3">
          <svg class="w-8 h-8" viewBox="0 0 48 48" fill="none">
            <defs>
              <linearGradient id="footer-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stop-color="#E64524"/>
                <stop offset="100%" stop-color="#06B6D4"/>
              </linearGradient>
            </defs>
            <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#footer-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
          </svg>
          <div>
            <span class="text-chalk-400 text-sm">
              <span class="text-rust-core font-semibold">Math</span><span class="text-solve-cyan font-semibold">Hook</span>
            </span>
            <p class="text-chalk-600 text-xs mt-0.5">Symbolic Power. Educational Clarity. Native Speed.</p>
          </div>
        </div>
        <div class="flex items-center gap-8 text-sm">
          <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">GitHub</a>
          <a href="https://crates.io/crates/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">Crates.io</a>
          <a href="https://pypi.org/project/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">PyPI</a>
          <a href="https://www.npmjs.com/package/mathhook-node" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">npm</a>
        </div>
      </div>
      <div class="mt-10 pt-8 border-t border-logic-navy-800/50 text-center text-xs text-chalk-600">
        Built with Rust. Open source under MIT OR Apache-2.0.
      </div>
    </div>
  </footer>
</template>
