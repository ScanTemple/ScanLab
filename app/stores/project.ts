import type { ProcessingStage } from '~~/src-tauri/bindings/ProcessingStage'

export const useProjectStore = defineStore('project', () => {
  const projectStages = ref([] as ProcessingStage[])
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

  async function refreshStages() {
    projectStages.value = await useCommands().listStages()
    console.log('Stages refreshed:', projectStages.value)
  }

  return { stagesArchive, projectStages, refreshStages }
})
