<template>
  <div class="min-h-screen bg-logic-navy-900">
    <NuxtPage />
  </div>
</template>

<script setup>
useHead({
  htmlAttrs: {
    lang: 'en',
    class: 'dark'
  },
  bodyAttrs: {
    class: 'font-sans antialiased'
  }
})
</script>
