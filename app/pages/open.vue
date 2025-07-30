<script setup lang="ts">
import { useMagicKeys, watchArray } from '@vueuse/core'

definePageMeta({
  layout: 'open',
})

const temp = useTempStore()
const files = computed(() => temp.files)
const thumbnails = computed(() => temp.thumbnails)

const previewStyles = tv({
  base: 'absolute inset-0 transition-colors shadow outline hover:outline-neutral-300 hover:text-neutral-300',
  variants: {
    select: {
      false: 'cursor-pointer',
    },
    active: {
      true: ' bg-gray-800/80 text-sky-300 outline-sky-300',
      false: 'bg-neutral-800/80 text-neutral-300/80 outline-transparent',
    },
  },
})

const { ctrl, shift, escape, ctrl_a, alt_a } = useMagicKeys({ passive: true })

watchArray(() => [escape?.value, alt_a?.value], () => {
  last.value = undefined

  for (const file of files.value) {
    thumbnails.value[file]!.selected = false
  }
})

watch(() => ctrl_a?.value, () => {
  last.value = undefined

  for (const file of files.value) {
    thumbnails.value[file]!.selected = true
  }
})

const isSelectMode = computed(() => ctrl?.value || shift?.value || false)

const last = ref(undefined as number | undefined)

function toggle(value: DataThumbnail, index: number, preview: () => void) {
  const mode = !value.selected

  if (ctrl?.value) {
    last.value = index
    value.selected = mode
  }

  else if (shift?.value) {
    value.selected = mode

    if (last.value !== undefined) {
      const start = Math.min(last.value, index)
      const end = Math.max(last.value, index)

      for (let i = start; i <= end; i++) {
        thumbnails.value[files.value[i]!]!.selected = mode
      }
    }

    last.value = index
  }

  else {
    preview()
  }
}

const help = [{
  keys: ['LMB'],
  label: 'full page preview',
}, {
  keys: ['CTRL', 'LMB'],
  label: 'single select',
}, {
  keys: ['SHIFT', 'LMB'],
  label: 'multiple select',
}, {
  keys: ['ESC'],
  label: 'clear select',
}] as const

const helpStyles = tv({
  base: '',
  variants: {
    button: {
      LMB: 'text-sky-300',
      CTRL: 'text-amber-300',
      SHIFT: 'text-lime-300',
      ESC: 'text-red-300',
    },
  },
})
</script>

<template>
  <section>
    <button @click="temp.selectDirectoryAndListFiles">
      Select Directory
    </button>

    <section class="grid grid-cols-[1fr_auto_1fr] gap-2 items-start">
      <div class="grid grid-cols-[auto_auto_1fr] sticky top-2 gap-2 opacity-30">
        <template
          v-for="{ keys, label }, index in help"
          :key="index"
        >
          <div class="space-x-2 text-end col-start-1">
            <file-open-help-button
              v-for="key in keys"
              :key="key"
              :class="helpStyles({ button: key })"
            >
              {{ key }}
            </file-open-help-button>
          </div>

          <div class="col-start-3">
            {{ label }}
          </div>
        </template>

        <div class="w-[1px] bg-zinc-700 col-start-2 row-start-1 row-span-4" />
      </div>

      <div class="grid grid-cols-[repeat(5,auto)] gap-2 justify-center">
        <div
          v-for="file, index in files"
          :key="file"
          class="flex items-center justify-center relative"
        >
          <template v-if="thumbnails[file]">
            <img
              :src="thumbnails[file].cover"
              :alt="thumbnails[file].name"
              class="object-cover border rounded border-zinc-700 shadow"
            >

            <FileOpenFullPreview
              v-slot="{ toggle: preview }"
              :src="thumbnails[file].cover"
              :alt="thumbnails[file].name"
            >
              <button
                :class="previewStyles({ active: thumbnails[file].selected, select: isSelectMode })"
                @click="toggle(thumbnails[file], index, preview)"
              >
                <div class="absolute inset-0 flex items-center justify-center text-4xl">
                  {{ 1 + index }}
                </div>

                <p class="absolute left-2 right-2 bottom-2 font-mono uppercase text-xs wrap-break-word">
                  {{ thumbnails[file].name.replaceAll('.', ' ') }}
                </p>
              </button>
            </FileOpenFullPreview>
          </template>

          <template v-else>
            <Icon
              name="ic:image"
              class="cursor-pointer text-4xl"
            />
          </template>
        </div>
      </div>

      <div>
        dummy
      </div>
    </section>
  </section>
</template>
