<script setup lang="ts">
defineProps<{
  to: string
}>()
</script>

<template>
  <NuxtLink
    :to="to"
    class="[&.router-link-active]:underline [&.router-link-active]:underline-offset-2 [&.router-link-active]:pointer-events-none not-[&.router-link-active]:text-neutral-500 not-[&.router-link-active]:hover:text-neutral-300 transition-colors"
  >
    <slot />
  </NuxtLink>
</template>
