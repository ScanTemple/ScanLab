<script setup lang="ts">
const scans = useScansStore()
// todo
</script>

<template>
  <image-grid>
    <filerotate-preview
      v-for="image in scans.data"
      :key="image"
      :image="image"
    />
  </image-grid>
</template>
