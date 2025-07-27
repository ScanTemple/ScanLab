<script setup lang="ts">
const scans = useScansStore()

const stages = [{
  label: 'Pick',
  url: '/',
}, {
  label: 'Rotate',
  url: '/rotate',
}, {
  label: 'Crop',
  url: '/crop',
}, {
  label: 'Save',
  url: '/save',
}]
</script>

<template>
  <section class="bg-neutral-800 text-neutral-300 min-h-dvh font-display flex flex-col">
    <section class="grow">
      <slot />
    </section>

    <fileprocess-stage v-if="scans.data.length > 0">
      <fileprocess-stage-item
        v-for="stage in stages"
        :key="stage.url"
        :to="stage.url"
      >
        {{ stage.label }}
      </fileprocess-stage-item>
    </fileprocess-stage>
  </section>
</template>
