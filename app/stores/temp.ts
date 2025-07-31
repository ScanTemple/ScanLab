import { invoke } from '@tauri-apps/api/core'
import { path } from '@tauri-apps/api'
import { open } from '@tauri-apps/plugin-dialog'
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs'

export type DataThumbnail = {
  cover: string
  name: string
  selected: boolean
}

export const useTempStore = defineStore('temp', () => {
  const files = ref([] as string[])
  const thumbnails = ref({} as Record<string, DataThumbnail>)
  const stage = crypto.randomUUID()
  const stageUpdatedAt = new Date()

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

  const imageService = useImageService()

  async function dummy() {
    const selected = 'Q:\\Users\\sawic\\Pictures\\chaos-game-chapter-1.zip\\'

    const entries = await readDir(selected, { baseDir: BaseDirectory.AppLocalData })
    files.value = await Promise.all(entries
      .filter(e => e.name && /\.(png|jpe?g|tiff?)$/i.test(e.name))
      // .map(e => selected + '/' + e.name)
      .map(e => path.join(selected, e.name)))
  }

  dummy()

  watch(files, async (newFiles) => {
    thumbnails.value = Object.fromEntries(newFiles.map(f => [f, { cover: '', name: '', selected: false } satisfies DataThumbnail]))

    // TODO: cache value on startup
    // TODO: use this to limit number of thumbnails generated at once
    const avalaibleCPUs = await invoke('get_cpus') as number
    console.log(`Available CPU cores: ${avalaibleCPUs}`)

    for (const file of newFiles) {
      path.basename(file).then((e) => {
        thumbnails.value[file]!.name = e
      })
    }

    for (const file of newFiles) {
      imageService.getThumbnail(file, 200, stage, stageUpdatedAt).then((thumbnail) => {
        thumbnails.value[file]!.cover = thumbnail.data.value
      })
    }
  }, { immediate: true })

  return {
    files,
    thumbnails,
    selectDirectoryAndListFiles,
  }
})
