export const useImagePreviewStore = defineStore('image-preview', () => {
  const data = reactive({
    src: '',
    alt: '',
  })

  function show(src: string, alt: string) {
    data.src = src
    data.alt = alt
  }

  return { ...toRefs(data), show }
})

// https://youtu.be/BiP0FpY88E4
