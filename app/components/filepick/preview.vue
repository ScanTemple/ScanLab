<script lang="ts" setup>
defineProps<{
  image: string
  name?: string
}>()
</script>

<template>
  <section class="relative overflow-hidden">
    <img
      :src="image"
      class="object-cover w-full"
    >

    <section class="absolute right-0 left-0 bottom-0 top-0 flex items-center justify-center bg-neutral-800/80 opacity-50 hover:opacity-100 transition-opacity">
      {{ name }}
    </section>
  </section>
</template>
