<script setup lang="ts">
const styles = tv({
  base: 'p-2 border border-zinc-700 bg-transparent shadow-md leading-none',
})

const model = defineModel<PropertyKey>({ required: true })
</script>

<template>
  <input
    v-bind="$attrs"
    v-model="model"
    type="text"
    :class="styles()"
    autocomplete="off"
  >
</template>
