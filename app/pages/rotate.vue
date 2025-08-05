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

const applyTo = [
  '—',
  'Each',
  'Every second',
]

const applyToRef = ref(0)

const applyPoint = [
  '—',
  'First',
  'Current',
]

const applyPointRef = ref(0)
</script>

<template>
  <section>
    <aside class="space-y-4 w-fit">
      <p class="text-lime-300/50 px-6">
        {{ state.toFixed(2) }}
      </p>

      <section :class="styles.block()">
        <header class="font-mono uppercase text-end pb-2 text-xs">
          actions
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
        <header class="font-mono uppercase text-end pb-2 text-xs">
          globals
        </header>

        <ui-dropdown>
          <template #header>
            <span>
              Apply to
            </span>

            <span
              v-if="applyToRef !== 0"
              class="text-amber-300/50"
            >
              {{ applyTo[applyToRef] }}
            </span>
          </template>

          <template #body>
            <ui-dropdown-item
              v-for="item, index in applyTo"
              :key="index"
              :disabled="index === applyToRef"
              @click="applyToRef = index"
            >
              {{ item }}
            </ui-dropdown-item>
          </template>
        </ui-dropdown>

        <ui-dropdown :disabled="applyToRef === 0">
          <template #header>
            <span>
              Start from
            </span>

            <span
              v-if="applyPointRef !== 0"
              class="text-amber-300/50"
            >
              {{ applyPoint[applyPointRef] }}
            </span>
          </template>

          <template #body>
            <ui-dropdown-item
              v-for="item, index in applyPoint"
              :key="index"
              :disabled="index === applyPointRef"
              @click="applyPointRef = index"
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
