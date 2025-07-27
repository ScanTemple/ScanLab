<script setup lang="ts">
const props = defineProps<{
  image: string
}>()

const _image = useImage({ src: props.image })
const isBadRotate = computed(() => {
  const width = _image.state?.value?.width
  if (!width) {
    return false
  }

  const height = _image.state?.value?.height
  if (!height) {
    return false
  }

  return width > height
})
</script>

<template>
  <section class="relative overflow-hidden">
    <img
      :src="image"
      class="object-cover w-full"
    >

    <div class="absolute top-0 right-0 left-0 bottom-0 bg-neutral-800/80 opacity-50 hover:opacity-100 transition-opacity">
      <div class="absolute flex items-center justify-evenly size-full">
        <button
          type="button"
          class="text-neutral-300 hover:text-neutral-300/30 transition-colors"
        >
          <Icon
            name="ic:baseline-rotate-left"
            class="cursor-pointer text-4xl"
          />
        </button>

        <button
          type="button"
          class="text-neutral-300 hover:text-neutral-300/30 transition-colors"
        >
          <Icon
            name="ic:baseline-screen-rotation-alt"
            class="cursor-pointer text-4xl"
          />
        </button>

        <button
          type="button"
          class="text-neutral-300 hover:text-neutral-300/30 transition-colors"
        >
          <Icon
            name="ic:baseline-rotate-right"
            class="cursor-pointer text-4xl"
          />
        </button>
      </div>

      <div
        v-if="isBadRotate"
        class="absolute bottom-1 right-1 px-2 py-1 rounded-md text-center bg-red-800 shadow-md"
      >
        bad rotation
      </div>
    </div>
  </section>
</template>
