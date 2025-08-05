<script setup lang="ts">
const state = useState(crypto.randomUUID(), () => faker.number.float({ min: 0, max: 360 }))

function addDegrees(degrees: number) {
  state.value = (state.value + degrees) % 360
}

function subtractDegrees(degrees: number) {
  state.value = (state.value - degrees) % 360
}

const styles = {
  block: tv({
    base: 'p-2  shadow-md leading-none backdrop-blur-[2px] border border-zinc-700',
  }),
}

const apply = reactive({
  to: {
    ref: ref(0),
    list: [
      '—',
      'Each',
      'Every second',
    ],
  },

  point: {
    ref: ref(0),
    list: [
      '—',
      'First',
      'Current',
    ],
  },
})

const isGlobal = computed(() => apply.to.ref && apply.point.ref)
</script>

<template>
  <section>
    <aside class="space-y-4 w-fit">
      <p class="text-lime-300/50 px-6">
        {{ state.toFixed(2) }}
      </p>

      <section :class="styles.block()">
        <header
          class="font-mono uppercase text-end pb-2 text-xs"
          :class="{
            'inline-flex items-center gap-2 justify-between pr-2': isGlobal,
            'px-2': !isGlobal,
          }"
        >
          <span
            v-show="isGlobal"
            class="text-amber-300 inline-flex items-center gap-[1ch] px-2"
          >
            <icon name="ic:baseline-warning-amber" />
            <span>will be applied globally</span>
          </span>

          <span>actions</span>
        </header>

        <ui-button
          type="button"
          @click="addDegrees(90)"
        >
          <icon name="ic:baseline-rotate-left" />
          <span>90° right</span>
        </ui-button>

        <ui-button
          type="button"
          @click="subtractDegrees(90)"
        >
          <icon name="ic:baseline-rotate-right" />
          <span>90° left</span>
        </ui-button>
      </section>

      <section :class="styles.block()">
        <header class="font-mono uppercase text-end pb-2 text-xs px-2">
          globals
        </header>

        <ui-dropdown>
          <template #header>
            <span>
              Apply to
            </span>

            <span
              v-if="apply.to.ref !== 0"
              class="text-amber-300/50"
            >
              {{ apply.to.list[apply.to.ref] }}
            </span>
          </template>

          <template #body>
            <ui-dropdown-item
              v-for="item, index in apply.to.list"
              :key="index"
              :disabled="index === apply.to.ref"
              @click="apply.to.ref = index"
            >
              {{ item }}
            </ui-dropdown-item>
          </template>
        </ui-dropdown>

        <ui-dropdown :disabled="apply.to.ref === 0">
          <template #header>
            <span>
              Start from
            </span>

            <span
              v-if="apply.point.ref !== 0"
              class="text-amber-300/50"
            >
              {{ apply.point.list[apply.point.ref] }}
            </span>
          </template>

          <template #body>
            <ui-dropdown-item
              v-for="item, index in apply.point.list"
              :key="index"
              :disabled="index === apply.point.ref"
              @click="apply.point.ref = index"
            >
              {{ item }}
            </ui-dropdown-item>
          </template>
        </ui-dropdown>
      </section>
    </aside>
    <!-- q(≧▽≦q) -->
  </section>
</template>
