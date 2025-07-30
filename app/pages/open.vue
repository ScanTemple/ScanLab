<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { path } from '@tauri-apps/api'
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs'

const files = ref<string[]>([])
const thumbnails = ref<Record<string, string>>({})

async function getThumbnail(filePath: string) {
  try {
    const base64 = await invoke('generate_thumbnail_from_path', {
      sourcePath: filePath,
      size: 200,
    })
    return `data:image/webp;base64,${base64}`
  }
  catch (e) {
    console.error(`Error generating thumbnail ${filePath}: ${e}`)
    return ''
  }
}

async function selectDirectoryAndListFiles() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'Select a directory with images',
  })
  if (typeof selected === 'string') {
    const entries = await readDir(selected, { baseDir: BaseDirectory.AppLocalData })
    files.value = await Promise.all(entries
      .filter(e => e.name && /\.(png|jpe?g|tiff?)$/i.test(e.name))
      // .map(e => selected + '/' + e.name)
      .map(e => path.join(selected, e.name)))
  }
}

// TODO: Fix UI thread blocking issue
watch(files, async (newFiles) => {
  thumbnails.value = Object.fromEntries(newFiles.map(f => [f, '']))
  for (const file of newFiles) {
    const thumbnail = await getThumbnail(file)
    if (thumbnail) {
      thumbnails.value[file] = thumbnail
    }
  }
}, { immediate: true })
</script>

<template>
  <section>
    <button @click="selectDirectoryAndListFiles">
      Select Directory
    </button>

    <div
      v-for="file in files"
      :key="file"
      class="inline-block m-2 border rounded flex items-center justify-center bg-gray-100 relative"
    >
      <template v-if="thumbnails[file]">
        <img
          :src="thumbnails[file]"
          alt="Thumbnail"
          class="object-cover rounded"
        >
      </template>
      <template v-else>
        <Icon
          name="ic:image"
          class="cursor-pointer text-4xl"
        />
      </template>
    </div>
  </section>
</template>
