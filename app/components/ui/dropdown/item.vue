<script lang="ts" setup>
defineProps<{
  disabled?: boolean
}>()
</script>

<template>
  <li>
    <ui-button
      type="button"
      v-bind="$attrs"
      :disabled="disabled"
    >
      <slot />
    </ui-button>
  </li>
</template>
