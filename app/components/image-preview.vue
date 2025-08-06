<script lang="ts" setup>
const preview = useImagePreviewStore()
</script>

<template>
  <ui-modal-base>
    <img
      ref="previewRef"
      :src="preview.src"
      :alt="preview.alt"
      class="h-full w-auto"
    >
  </ui-modal-base>
</template>
