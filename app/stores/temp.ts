import { invoke } from '@tauri-apps/api/core'
import { path } from '@tauri-apps/api'
import { open } from '@tauri-apps/plugin-dialog'
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs'

export type DataThumbnail = {
  cover: string
  name: string
  seelcted: boolean
}

export const useTempStore = defineStore('temp', () => {
  const files = ref([] as string[])
  const thumbnails = ref({} as Record<string, DataThumbnail>)

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

  async function dummy() {
    const selected = 'Q:\\Users\\sawic\\Pictures\\chaos-game-chapter-1.zip\\'

    const entries = await readDir(selected, { baseDir: BaseDirectory.AppLocalData })
    files.value = await Promise.all(entries
      .filter(e => e.name && /\.(png|jpe?g|tiff?)$/i.test(e.name))
      // .map(e => selected + '/' + e.name)
      .map(e => path.join(selected, e.name)))
  }

  dummy()

  // TODO: Fix UI thread blocking issue
  watch(files, async (newFiles) => {
    thumbnails.value = Object.fromEntries(newFiles.map(f => [f, { cover: '', name: '', seelcted: false } satisfies DataThumbnail]))

    for (const file of newFiles) {
      path.basename(file).then((e) => {
        thumbnails.value[file]!.name = e
      })
    }

    for (const file of newFiles) {
      getThumbnail(file).then((thumbnail) => {
        thumbnails.value[file]!.cover = thumbnail
      })
    }
  }, { immediate: true })

  return {
    files,
    thumbnails,
    selectDirectoryAndListFiles,
  }
})
