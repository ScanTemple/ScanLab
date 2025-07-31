export type DataStage = {
  icon: string
  label: string
  url: string

  // TODO: enum for backend
  uuid: UUID
}

export const useStagesStore = defineStore('stages', () => {
  const data = ref([
    {
      icon: 'ic:baseline-image',
      label: 'Pick',
      url: '/',
      uuid: crypto.randomUUID(),
    },
    {
      icon: 'ic:outline-file-open',
      label: 'Open',
      url: '/open',
      uuid: crypto.randomUUID(),
    },
    {
      icon: 'ic:baseline-crop',
      label: 'Crop',
      url: '/crop',
      uuid: crypto.randomUUID(),
    },
    {
      icon: 'ic:baseline-rotate-90-degrees-ccw',
      label: 'Rotate',
      url: '/rotate',
      uuid: crypto.randomUUID(),
    },
    {
      icon: 'ic:baseline-save',
      label: 'Save',
      url: '/save',
      uuid: crypto.randomUUID(),
    },
  ] satisfies ReadonlyArray<DataStage> as ReadonlyArray<DataStage>)

  return { data }
})

// https://www.youtube.com/watch?v=EDnIEWyVIlE
