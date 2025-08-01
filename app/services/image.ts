import { invoke } from '@tauri-apps/api/core'
import type { UUID } from 'crypto'

export const useImageService = () => {
  return {
    getThumbnail: (path: string, size: number, stage: UUID, validAt: Date) => {
      const key = `getThumbnail:${path}:${size}:${stage}:${validAt}`

      return useAsyncData<string>(key, async () => invoke('generate_thumbnail_from_path', {
        sourcePath: path,
        size,
      }), {
        default: () => '',

        lazy: true,
      })
    },

    getPreview: (path: string, stage: UUID, validAt: Date) => {
      const key = `getPreview:${path}:${stage}:${validAt}`

      return useAsyncData<string>(key, async () => invoke('show_image', {
        sourcePath: path,
      }), {
        default: () => '',

        lazy: true,
      })
    },
  }
}
