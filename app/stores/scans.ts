export const useScansStore = defineStore('scans', () => {
  const data = ref([] as string[])
  const isLoading = ref(false)

  function dummy() {
    data.value.push(...Array.from({ length: 12 }, _ => faker.image.urlPicsumPhotos(faker.datatype.boolean()
      ? {
          width: 640,
          height: 480,
        }
      : {
          width: 480,
          height: 640,
        })),
    )
  }

  return { dummy, data, isLoading }
})
