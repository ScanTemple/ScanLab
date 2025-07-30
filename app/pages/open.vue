<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'

const thumbnailSrc = ref('')

async function getThumbnail(filePath: string) {
  try {
    const base64 = await invoke('generate_thumbnail_from_path', {
      sourcePath: filePath,
      size: 200,
    })
    return `data:image/avif;base64,${base64}`
  }
  catch (e) {
    console.error('Error generating thumbnail:', e)
    return ''
  }
}

onMounted(async () => {
  thumbnailSrc.value = await getThumbnail('/home/etsune/Pictures/scans/2022-12-05-0049.tif')
})
</script>

<template>
  <section>
    <img
      :src="thumbnailSrc"
      alt="Thumbnail"
    >
  </section>
</template>
