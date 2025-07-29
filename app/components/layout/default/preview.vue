<script setup lang="ts">
import { path } from '@tauri-apps/api'

const props = defineProps<{
  uuid: DataScan['uuid']
  image: DataScan['image']
  name: DataScan['name']
}>()

const normalized = await path.basename(props.name)

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
    class="p-2 space-y-2 border border-zinc-700 w-full"
    :class="styles({ button: Number(scan.data?.uuid === uuid) })"
    @click="scan.pick(uuid)"
  >
    <img
      :src="image"
      class="w-full"
    >

    <p class="truncate uppercase font-mono text-xs px-2">
      {{ normalized.replaceAll(/_|-/g, ' ') }}
    </p>
  </button>
</template>
