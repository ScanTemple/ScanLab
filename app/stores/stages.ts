import type { UUID } from 'crypto'

export type DataStage = {
  uuid: UUID
  icon: string
  label: string
  url: string
}

export const useStagesStore = defineStore('stages', () => {
  const data = ref([{
    uuid: `88bfd403-27e9-4af0-ac92-21ea5595147a`,
    icon: 'ic:baseline-image',
    label: 'Pick',
    url: '/',
  }, {
    uuid: `afc4a3b3-8fb2-4155-885e-7684af599a70`,
    icon: 'ic:outline-file-open',
    label: 'Open',
    url: '/open',
  }, {
    uuid: `142f891c-78a9-4743-8400-2bacfcfa4750`,
    icon: 'ic:baseline-crop',
    label: 'Crop',
    url: '/crop',
  }, {
    uuid: `82fee5c0-144a-4c01-a170-e96660890ee3`,
    icon: 'ic:baseline-rotate-90-degrees-ccw',
    label: 'Rotate',
    url: '/rotate',
  }, {
    uuid: `0f0c3d75-dcab-44de-8d3e-0c87c48718c6`,
    icon: 'ic:baseline-save',
    label: 'Save',
    url: '/save',
  }] satisfies ReadonlyArray<DataStage> as ReadonlyArray<DataStage>)

  return { data }
})

// https://www.youtube.com/watch?v=EDnIEWyVIlE
