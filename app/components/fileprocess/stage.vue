<script setup lang="ts">
const stageLayers = useStageLayersStore()
</script>

<template>
  <ul class="space-x-2">
    <fileprocess-stage-item
      v-for="layer in stageLayers.defined"
      :key="layer.uuid"
      class="inline-block"
      :reference="layer.reference"
    />
  </ul>
</template>
