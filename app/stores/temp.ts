import { path } from '@tauri-apps/api'
import { open } from '@tauri-apps/plugin-dialog'
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs'
import type { UUID } from 'crypto'

export type DataThumbnail = {
  uuid: UUID
  path: string
  cover: string
  name: string
  selected: boolean
}

export const useTempStore = defineStore('temp', () => {
  const thumbnails = ref([] as DataThumbnail[])
  const stage = crypto.randomUUID()
  const stageUpdatedAt = new Date()

  const imageService = useImageService()

  function addThumbnails(paths: string[]) {
    const files = paths.map(e => (reactive({
      uuid: crypto.randomUUID(),
      path: e,
      cover: '',
      name: '',
      selected: false,
    })))

    for (const file of files) {
      path.basename(file.path).then((e) => {
        file.name = e
      })
    }

    for (const file of files) {
      imageService.getThumbnail(file.path, 200, stage, stageUpdatedAt).then((thumbnail) => {
        file.cover = thumbnail.data.value
      })
      // imageService.getPreview(file.path, stage, stageUpdatedAt).then((image) => {
      //   file.cover = image.data.value
      // })
    }

    return files
  }

  async function selectDirectoryAndListFiles() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select a directory with images',
    })

    if (!selected) {
      return
    }

    const entries = await readDir(selected, { baseDir: BaseDirectory.AppLocalData })
    const names = entries.filter(e => /\.(png|jpe?g|tiff?)$/i.test(e.name))
    const paths = await Promise.all(names.map(e => path.join(selected, e.name)))

    thumbnails.value.push(...addThumbnails(paths))
  }

  return {
    thumbnails,
    selectDirectoryAndListFiles,
  }
})
