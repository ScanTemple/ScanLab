export const useScanStore = defineStore('scan', () => {
  const scans = useScansStore()

  const _uuid = ref(undefined as undefined | UUID)

  function pick(uuid: UUID) {
    _uuid.value = uuid
  }

  const data = computed(() => _uuid.value ? scans.data.find(e => e.uuid === _uuid.value) : undefined)

  // todo: won't trigger if same value setted twice... maybe
  //       but _uuid need to be checked
  watch(data, (value) => {
    // scan has been removed
    if (value?.uuid !== _uuid.value) {
      _uuid.value = undefined
    }
  })

  return { pick, data }
})
