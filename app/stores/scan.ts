export const useScanStore = defineStore('scan', () => {
  const scans = useScansStore()

  const _uuid = ref(undefined as undefined | ReturnType<typeof crypto.randomUUID>)

  function pick(uuid: ReturnType<typeof crypto.randomUUID>) {
    _uuid.value = uuid
  }

  const data = computed(() => _uuid.value ? scans.data.find(e => e.uuid === _uuid.value) : undefined)

  // todo: won't trigger if same value setted twice... maybe
  watch(data, (value) => {
    // scan has been removed
    if (value?.uuid !== _uuid.value) {
      _uuid.value = undefined
    }
  })

  return { pick, data }
})
