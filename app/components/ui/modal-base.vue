<script lang="ts" setup>
import { onClickOutside, useMagicKeys } from '@vueuse/core'
import type { WatchHandle } from 'vue'

const modal = reactive(useProvideModalState())

const { escape } = useMagicKeys({ passive: true })

function createWatch() {
  return watch(() => escape?.value, (value) => {
    if (value) {
      modal?.hide()
    }
  })
}

let cancel = undefined as WatchHandle | undefined

watch(() => modal.state, (value) => {
  cancel?.()

  if (value) {
    cancel = createWatch()
  }
}, { immediate: true })

const elementRef = useTemplateRef('elementRef')
onClickOutside(elementRef, () => modal.hide())
</script>

<template>
  <slot
    name="trigger"
    v-bind="modal"
  />

  <section
    v-if="modal.state"
    class="fixed inset-0 z-10 bg-neutral-800/80 backdrop-blur-md"
  >
    <section class="flex items-center justify-center size-full p-8">
      <section ref="elementRef">
        <slot v-bind="modal" />
      </section>
    </section>
  </section>
  <!-- https://youtu.be/5TQjhckMcr4 -->
</template>
