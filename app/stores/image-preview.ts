export const useImagePreviewStore = defineStore('image-preview', () => {
  const data = reactive({
    src: '',
    alt: '',
    state: false,
  })

  function show(src: string, alt: string) {
    data.src = src
    data.alt = alt
    data.state = true
  }

  function hide() {
    data.state = false
  }

  return { ...toRefs(data), show, hide }
})

// https://youtu.be/BiP0FpY88E4
