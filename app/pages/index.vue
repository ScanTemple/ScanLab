<script setup lang="ts">
const scans = useScansStore()

function pickFolder() {
  scans.dummy()
}

function pickFiles() {
  scans.dummy()
}

const actions = tv({
  variants: {
    header: {
      [0 as number]: 'min-h-dvh flex flex-col gap-16 items-center justify-center',
      [1 as number]: 'flex gap-4 sticky top-0 bg-neutral-800 z-10 p-4 items-center shadow',
    },
    text: {
      [0 as number]: 'text-6xl',
      [1 as number]: '',
    },
    buttons: {
      [0 as number]: 'flex gap-16 items-center',
      [1 as number]: 'flex gap-4 items-center',
    },
    button: {
      [0 as number]: 'text-neutral-800 px-4 py-2 text-6xl cursor-pointer bg-neutral-300 hover:bg-neutral-300/30 transition-colors',
      [1 as number]: 'text-neutral-800 px-2 py-1 cursor-pointer bg-neutral-300/30 hover:bg-neutral-300 transition-colors w-fit',
    },
  },

})
</script>

<template>
  <section>
    <section :class="actions({ header: Number(scans.data.length > 0) })">
      <p :class="actions({ text: Number(scans.data.length > 0) })">
        Pick
      </p>

      <section :class="actions({ buttons: Number(scans.data.length > 0) })">
        <button
          type="button"
          :class="actions({ button: Number(scans.data.length > 0) })"
          @click="pickFolder"
        >
          folder
        </button>

        <p :class="actions({ text: Number(scans.data.length > 0) })">
          or
        </p>

        <button
          type="button"
          :class="actions({ button: Number(scans.data.length > 0) })"
          @click="pickFiles"
        >
          files
        </button>
      </section>
    </section>

    <image-grid class="grow">
      <filepick-preview
        v-for="image in scans.data"
        :key="image"
        :image="image"
        :name="'test name'"
      />
    </image-grid>
  </section>
</template>
