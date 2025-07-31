<script setup lang="ts">
defineProps<{
  uuid: DataThumbnail['uuid']
  cover: DataThumbnail['cover']
  name: DataThumbnail['name']
}>()

const scan = useScanStore()

const styles = tv({
  variants: {
    button: {
      [0 as number]: 'cursor-pointer hover:bg-zinc-700 text-neutral-300/50 hover:text-neutral-300 transition-colors',
      [1 as number]: 'bg-zinc-700 pointer-events-none border-r-zinc-300',
    },
  },
})
</script>

<template>
  <button
    type="button"
    class="p-2 space-y-2 border-x border-transparent w-full"
    :class="styles({ button: Number(scan.data?.uuid === uuid) })"
    @click="scan.pick(uuid)"
  >
    <img
      :src="cover"
      class="w-full"
    >

    <p class="truncate uppercase font-mono text-xs px-2">
      {{ name.replaceAll(/_|-/g, ' ') }}
    </p>
  </button>
</template>
