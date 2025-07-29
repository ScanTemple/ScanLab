<script setup lang="ts">
const stageLayers = useStageLayersStore()
</script>

<template>
  <ul class="space-x-2">
    <template
      v-for="layer in stageLayers.defined"
      :key="layer.uuid"
    >
      <fileprocess-stage-item
        class="inline-block"
        :reference="layer.reference"
      />

      <icon
        v-if="layer !== stageLayers.defined.at(-1)"
        name="ic:baseline-chevron-right"
        class="text-neutral-300/50"
      />
    </template>
  </ul>
</template>
