export type DataScan = {
  image: string
  uuid: UUID
  name: string
}

export const useScansStore = defineStore('scans', () => {
  const data = ref([] as DataScan[])
  const isLoading = ref(false)

  function dummy() {
    data.value.push(...Array.from({ length: 12 }, _ => ({
      uuid: faker.string.uuid() as UUID,
      name: faker.system.filePath(),
      image: faker.image.urlPicsumPhotos(faker.datatype.boolean()
        ? {
            width: 640,
            height: 480,
          }
        : {
            width: 480,
            height: 640,
          }),

    })),
    )
  }

  dummy()
  dummy()
  dummy()

  return { dummy, data, isLoading }
})
