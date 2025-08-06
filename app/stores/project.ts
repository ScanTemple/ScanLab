import type { UUID } from 'crypto'

export type DataStage2 = {
  icon: string
  label: string
  url: string
}

export type DataStage3 = {
  uuid: UUID
} & DataStage2 & ProcessingStage

const types = {
  open: {
    icon: 'ic:outline-file-open',
    label: 'Open',
    url: '/open',
  },
} as Record<string, DataStage2>

// [{
//   icon: 'ic:baseline-image',
//   label: 'Pick',
//   url: '/',
// }, {
//   icon: 'ic:outline-file-open',
//   label: 'Open',
//   url: '/open',
// }, {
//   icon: 'ic:baseline-crop',
//   label: 'Crop',
//   url: '/crop',
// }, {
//   icon: 'ic:baseline-rotate-90-degrees-ccw',
//   label: 'Rotate',
//   url: '/rotate',
// }, {
//   icon: 'ic:baseline-save',
//   label: 'Save',
//   url: '/save',
// }]

export const useProjectStore = defineStore('project', () => {
  const stagesArchive = readonly([{
    category: '',
    stages: [
      { name: 'Rotate', value: 'rotate', icon: 'ic:baseline-rotate-right' },
      { name: 'Crop', value: 'crop', icon: 'ic:baseline-crop' },
      { name: 'Resize', value: 'resize', icon: 'ic:baseline-resize' },
    ],
  }, {
    category: 'Basic',
    stages: [
      { name: 'Flip', value: 'flip', icon: 'ic:baseline-flip' },
      { name: 'Filter', value: 'filter', icon: 'ic:baseline-filter' },
    ],
  }, {
    category: 'Advanced', // just for example
    stages: [
      { name: 'Color Correction', value: 'color-correction', icon: 'ic:baseline-colorize' },
      { name: 'Sharpen', value: 'sharpen', icon: 'ic:baseline-sharpness' },
      { name: 'Noise Reduction', value: 'noise-reduction', icon: 'ic:baseline-noise-control' },
      { name: 'HDR Merge', value: 'hdr-merge', icon: 'ic:baseline-hdr-strong' },
    ],
  }])

  const _projectStages = ref([] as ProcessingStage[])

  const projectStages = computed(() => {
    return _projectStages.value.map(e => ({
      uuid: crypto.randomUUID(),
      ...types[e.type] || {},
      ...e,
    } as DataStage3))
  })

  async function refreshStages() {
    _projectStages.value = await useCommands().listStages()
    console.log('Stages refreshed:', _projectStages.value)
  }

  return { stagesArchive, projectStages, refreshStages }
})
