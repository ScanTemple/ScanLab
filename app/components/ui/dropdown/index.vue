<script lang="ts" setup>
import { onClickOutside, useToggle } from '@vueuse/core'

const [state, toggle] = useToggle(false)

defineProps<{
  disabled?: boolean
}>()

const styles = tv({
  base: 'leading-none gap-[1ch] transition-colors py-2 px-4 flex items-center w-full text-shadow-md',
  variants: {
    disabled: {
      true: 'pointer-events-none text-zinc-300/10',
      false: 'cursor-pointer',
    },
    active: {
      true: '',
    },
    warning: {
      true: 'border-l-2 border-red-500',
    },
  },
  defaultVariants: {
    disabled: false,
    warning: false,
    active: false,
  },
  compoundVariants: [{
    active: false,
    disabled: false,
    class: 'text-neutral-300/50 hover:text-neutral-300 hover:bg-zinc-700',
  }, {
    active: true,
    disabled: false,
    class: 'text-neutral-300 bg-zinc-700 hover:text-neutral-700 hover:bg-zinc-300',
  }],
})

onClickOutside(useTemplateRef('elementRef'), () => state.value = false)
</script>

<template>
  <section
    ref="elementRef"
    class="relative"
  >
    <button
      type="button"
      :class="styles({ active: state, disabled })"
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
    </button>

    <ul
      v-if="state"
      class="absolute top-full left-0 w-full bg-zinc-800 shadow-md z-10"
    >
      <slot name="body" />
    </ul>
  </section>
</template>
