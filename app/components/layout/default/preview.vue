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
      [0 as number]: 'cursor-pointer hover:bg-zinc-700 text-neutral-300/50 hover:text-amber-300',
      [1 as number]: 'bg-zinc-700 text-amber-300 pointer-events-none',
    },
  },
})
</script>

<template>
  <button
    type="button"
    class="p-2 rounded shadow border-x border-zinc-700"
    :class="styles({ button: Number(scan.data?.uuid === uuid) })"
    @click="scan.pick(uuid)"
  >
    <section class="flex flex-col gap-2">
      <img
        :src="image"
        class="w-full rounded"
      >

      <p class="wrap-break-word">
        {{ normalized.replaceAll(/_|-/g, ' ') }}
      </p>
    </section>
  </button>
</template>
