<template>
  <section class="grid scans-grid gap-2 content-start">
    <slot />
  </section>
</template>

<style scoped>
.scans-grid {
  grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr));
}
</style>
