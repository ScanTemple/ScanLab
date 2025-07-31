<script lang="ts" setup>
import { onClickOutside, useMagicKeys } from '@vueuse/core'

const preview = useImagePreviewStore()

const { escape } = useMagicKeys({ passive: true })

watch(() => escape?.value, (value) => {
  if (value) {
    preview.hide()
  }
})

const previewRef = useTemplateRef('previewRef')
onClickOutside(previewRef, () => preview.hide())
</script>

<template>
  <section
    v-if="preview.state"
    class="fixed inset-0 z-10 bg-neutral-800/80 backdrop-blur-md"
  >
    <div class="flex items-center justify-center size-full p-8">
      <img
        ref="previewRef"
        :src="preview.src"
        :alt="preview.alt"
        class="h-full w-auto"
      >
    </div>
  </section>
  <!-- https://youtu.be/5TQjhckMcr4 -->
</template>
