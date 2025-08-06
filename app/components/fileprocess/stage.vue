<script setup lang="ts">
// const stageLayers = useStageLayersStore()
const project = useProjectStore()
</script>

<template>
  <ul class="space-x-2 border border-zinc-700 shadow leading-none row-start-2 backdrop-blur-[2px] p-2 flex justify-center items-center">
    <template
      v-for="layer in project.projectStages"
      :key="layer.uuid"
    >
      <fileprocess-stage-item
        class="inline-block"
        v-bind="layer"
      />

      <icon
        name="ic:baseline-chevron-right"
        class="text-neutral-300/50"
      />
    </template>

    <add-stage v-show="false" />
  </ul>
</template>
