<script setup lang="ts">
const scan = useScanStore()
</script>

<template>
  <section class="h-full">
    <template v-if="scan.data">
      <img
        :src="scan.data.image"
        class="h-full w-full object-contain"
      >
    </template>

    <template v-else>
      pick scan in right bar ┻━┻ ︵ ＼( °□° )／ ︵ ┻━┻
    </template>
  </section>
</template>
