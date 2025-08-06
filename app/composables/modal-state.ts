import { createInjectionState, useToggle } from '@vueuse/core'

const injectionKey = 'modal-state'

export const [useProvideModalState, useModalState] = createInjectionState(() => {
  const [state, toggle] = useToggle(false)

  return {
    state,
    toggle,

    hide: () => toggle(false),
    show: () => toggle(true),
  }
}, { injectionKey })
