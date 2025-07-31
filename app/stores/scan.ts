import type { UUID } from 'crypto'

export const useScanStore = defineStore('scan', () => {
  const temp = useTempStore()

  const _uuid = ref(undefined as undefined | UUID)

  function pick(uuid: UUID) {
    _uuid.value = uuid
  }

  const data = computed(() => {
    if (_uuid.value) {
      return temp.thumbnails.find(e => e.uuid === _uuid.value)
    }

    return undefined
  })

  return { pick, data }
})
