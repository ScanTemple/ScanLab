<script lang="ts" setup>
const styles = tv({
  base: 'leading-none gap-[1ch] transition-colors py-2 px-4 flex items-center w-full text-shadow-md',
  variants: {
    disabled: {
      true: 'pointer-events-none text-zinc-300/10',
      false: 'cursor-pointer text-neutral-300/50 hover:text-neutral-300 hover:bg-zinc-700',
    },
    warning: {
      true: 'border-l-2 border-red-500',
    },
  },
  defaultVariants: {
    disabled: false,
    warning: false,
  },
})

defineProps<{
  disabled?: boolean
}>()
</script>

<template>
  <li>
    <button
      type="button"
      :class="styles({ disabled })"
      v-bind="$attrs"
      :disabled="disabled"
    >
      <slot />
    </button>
  </li>
</template>
