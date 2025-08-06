import type { UUID } from 'crypto'

export type DataStageLayerSettings = never

export type DataStageLayer = {
  uuid: UUID

  // metainfo of current stage
  reference: DataStage

  // stage updated at
  // e.g. changes on back stages, or changes in configs of current stage
  actualAt: Date

  // todo: stage settings
  data: DataStageLayerSettings

  items: Record<UUID, {
    // todo: path, base64, arraybuffer
    preview: string

    // todo: scan overide settings for current stage
    data: DataStageLayerSettings
  }>
}

export const useStageLayersStore = defineStore('stage-layers', () => {
  const user = ref([] as DataStageLayer[])

  const stages = useStagesStore()
  const now = new Date()

  // todo: for test purposes
  onMounted(() => {
    user.value.push(...stages.data.slice(1, stages.data.length - 1).map(e => ({
      uuid: crypto.randomUUID(),
      reference: e,
      actualAt: now,
      items: {},
    } as DataStageLayer satisfies DataStageLayer)))
  })

  const pick = {
    uuid: crypto.randomUUID(),
    reference: stages.data[0],
    actualAt: now,
    items: {},
  } as DataStageLayer satisfies DataStageLayer

  const save = {
    uuid: crypto.randomUUID(),
    reference: stages.data[stages.data.length - 1],
    actualAt: now,
    items: {},
  } as DataStageLayer satisfies DataStageLayer

  const defined = computed(() => {
    return [
      pick,
      ...user.value,
      save,
    ] satisfies ReadonlyArray<DataStageLayer> as ReadonlyArray<DataStageLayer>
  })

  const route = useRoute()
  const active = computed(() => defined.value.find((e) => {
    console.log(route.query, '/', e.uuid)
    return e.uuid === route.query.uuid
  }))

  return { active, defined, user }
})
