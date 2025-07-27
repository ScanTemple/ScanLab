<script setup lang="ts">
const scans = useScansStore()
</script>

<template>
  <image-grid>
    <filecrop-preview
      v-for="image in scans.data"
      :key="image"
      :image="image"
    />
  </image-grid>
</template>
