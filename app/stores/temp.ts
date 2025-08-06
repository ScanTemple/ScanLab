import { path } from '@tauri-apps/api'
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
  const stage1 = crypto.randomUUID()
  const stageUpdatedAt = new Date()

  const imageService = useImageService()

  const commands = useCommands()

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
      imageService.getThumbnail(file.path, 200, stage1, stageUpdatedAt).then((thumbnail) => {
        file.cover = thumbnail.data.value
      })
    }

    return files
  }

  async function loadImagesFromStage() {
    const stage = await commands.getStage(0)
    if (!stage) {
      return
    }

    const imagePaths = stage.images.map((image) => {
      const imageInfo = image as ImageInfo<OpenParams>
      return imageInfo.params.file_path
    })

    if (imagePaths.length > 0) {
      const newThumbnails = addThumbnails(imagePaths)
      thumbnails.value.push(...newThumbnails)
    }
  }

  loadImagesFromStage()

  return {
    thumbnails,
  }
})
