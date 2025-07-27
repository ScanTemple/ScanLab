<script setup lang="ts">
defineProps<{
  image: string
}>()
</script>

<template>
  <section class="relative overflow-hidden">
    <img
      :src="image"
      class="object-cover w-full"
    >

    <div class="absolute top-0 right-0 left-0 bottom-0 bg-neutral-800/80 opacity-50 hover:opacity-100 transition-opacity flex items-center justify-evenly size-full">
      <button
        type="button"
        class="text-neutral-300 hover:text-neutral-300/30 transition-colors"
      >
        <Icon
          name="ic:baseline-crop"
          class="cursor-pointer text-4xl"
        />
      </button>
    </div>
  </section>
</template>
