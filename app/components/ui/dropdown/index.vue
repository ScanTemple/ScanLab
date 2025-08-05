<script lang="ts" setup>
import { onClickOutside, useToggle } from '@vueuse/core'

const [state, toggle] = useToggle(false)

defineProps<{
  disabled?: boolean
}>()

onClickOutside(useTemplateRef('elementRef'), () => state.value = false)
</script>

<template>
  <section
    ref="elementRef"
    class="relative"
  >
    <ui-button
      type="button"
      :active="state"
      :disabled="disabled"
      @click="toggle()"
    >
      <icon
        v-if="state"
        name="ic:baseline-arrow-drop-up"
      />

      <icon
        v-else
        name="ic:baseline-arrow-drop-down"
      />

      <span>
        <slot name="header" />
      </span>
    </ui-button>

    <ul
      v-if="state"
      class="absolute top-full left-0 w-full bg-zinc-800 shadow-md z-10"
    >
      <slot name="body" />
    </ul>
  </section>
</template>
