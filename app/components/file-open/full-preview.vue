<script lang="ts" setup>
defineProps<{
  src: string
  alt: string
  state: boolean
}>()

defineEmits<{
  (e: 'toggle', value: boolean): void
}>()
</script>

<template>
  <section
    v-if="state"
    class="fixed inset-0 z-10 bg-neutral-800/80 backdrop-blur-md"
  >
    <img
      :src="src"
      :alt="alt"
      class="h-full w-full p-12 object-contain"
    >

    <button
      type="button"
      class="fixed top-2 right-2 text-neutral-300 hover:text-neutral-300/30 transition-colors cursor-pointer"
      @click="$emit('toggle', false)"
    >
      <Icon
        name="ic:baseline-close"
        class="text-4xl"
      />
    </button>
  </section>

  <slot />

  <!-- https://youtu.be/5TQjhckMcr4 -->
</template>
