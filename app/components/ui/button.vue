<script lang="ts" setup>
defineProps<{
  active?: boolean
  disabled?: boolean
}>()

const styles = tv({
  base: 'leading-none gap-[1ch] transition-colors p-2 flex items-center w-full text-shadow-md group/button',
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
    class: 'text-neutral-300/50 hover:text-neutral-300 hover:opacity-100 hover:bg-zinc-700',
  }, {
    active: true,
    disabled: false,
    class: 'text-neutral-300 bg-zinc-700 hover:text-neutral-700 hover:bg-zinc-300',
  }],
})
</script>

<template>
  <button
    type="button"
    :class="styles({ active, disabled })"
  >
    <slot />
  </button>
</template>
