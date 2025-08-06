import { path } from '@tauri-apps/api'
import { open } from '@tauri-apps/plugin-dialog'
import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs'
import type { ProcessingStage } from '@/../src-tauri/bindings/ProcessingStage'
import { invoke } from '@tauri-apps/api/core'
import type { UUID } from 'crypto'
import type { OpenParams } from '~~/src-tauri/bindings/OpenParams'
import type { ImageInfo } from '~~/src-tauri/bindings/ImageInfo'

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
    try {
      await openImagesDialog()
      await loadImagesFromStage()
    } catch (error) {
      console.error('Failed to select and load images:', error)
      throw error
    }
  }

  async function openImagesDialog() {
    await invoke('open_images', { position: undefined })
  }

  async function loadImagesFromStage() {
    const stage = await invoke('get_stage', { index: 0 }) as ProcessingStage
    
    const imagePaths = stage.images.map((image) => {
      const imageInfo = image as ImageInfo<OpenParams>
      return imageInfo.params.file_path
    })
    
    if (imagePaths.length > 0) {
      const newThumbnails = addThumbnails(imagePaths)
      thumbnails.value.push(...newThumbnails)
    }
  }

  // async function selectDirectoryAndListFiles() {
  //   // const selected = await open({
  //   //   directory: true,
  //   //   multiple: false,
  //   //   title: 'Select a directory with images',
  //   // })

  //   // if (!selected) {
  //   //   return
  //   // }

  //   // const entries = await readDir(selected, { baseDir: BaseDirectory.AppLocalData })
  //   // const names = entries.filter(e => /\.(png|jpe?g|tiff?)$/i.test(e.name))
  //   // const paths = await Promise.all(names.map(e => path.join(selected, e.name)))

  //   // thumbnails.value.push(...addThumbnails(paths))
  //   // await
  //   await invoke('open_images', { position: undefined })
  //   .then(async () => {
  //     await invoke('get_stage', { index: 0 }).then((result) => {
  //       const stage = result as ProcessingStage
  //       stage.images.forEach((image) => {
  //         const path = (image as ImageInfo<OpenParams>).params.file_path
  //         thumbnails.value.push(...addThumbnails([path]))
  //       })
  //     })
  //   })
  // }

  return {
    thumbnails,
    selectDirectoryAndListFiles,
  }
})
