<script lang="ts" setup>
const commands = useCommands()
</script>

<template>
  <section
    v-if="commands.error.value"
    class="fixed top-0 left-0 right-0 shadow-md bg-red-800 leading-none p-2 text-center flex gap-2 items-center z-50"
  >
    <span class="grow">{{ commands.error.value }}</span>
    <ui-button
      @click="commands.clearError()"
    >
      <icon name="ic:baseline-close" />
    </ui-button>
  </section>
</template>
