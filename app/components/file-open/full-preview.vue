<script lang="ts" setup>
import { useToggle } from '@vueuse/core'

defineProps<{
  src: string
  alt: string
}>()

const [state, toggle] = useToggle(false)
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
      @click="() => state = false"
    >
      <Icon
        name="ic:baseline-close"
        class="text-4xl"
      />
    </button>
  </section>

  <slot v-bind="{ state, toggle }" />

  <!-- https://youtu.be/5TQjhckMcr4 -->
</template>
