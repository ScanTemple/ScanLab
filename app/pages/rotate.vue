<script setup lang="ts">
const state = useState(crypto.randomUUID(), () => faker.number.float({ min: 0, max: 360 }))

function addDegrees(degrees: number) {
  state.value = (state.value + degrees) % 360
}

function subtractDegrees(degrees: number) {
  state.value = (state.value - degrees) % 360
}

const styles = {
  layer: {
    button: tv({
      base: 'leading-none gap-[1ch] items-center transition py-2 px-4 w-full group/button grid grid-cols-[auto_1fr_auto] text-end hover:translate-x-2 text-shadow-md',
      variants: {
        active: {
          true: 'pointer-events-none text-neutral-300 border-b-zinc-300',
          false: 'cursor-pointer hover:bg-zinc-700s text-neutral-300/50 hover:text-neutral-300',
        },
      },
    }),

    value: tv({
      base: 'transition-colors text-end text-shadow-md font-mono text-sm',
      variants: {
        active: {
          true: 'text-lime-300',
          false: 'text-lime-300/50 group-hover/button:text-lime-300',
        },
      },
    }),

    type: tv({
      base: 'transition-colors',
      variants: {
        type: {
          global: '',
          local: '',
        },
        active: {
          true: '',
          false: '',
        },
      },
      compoundVariants: [{
        active: true,
        type: 'global',
        class: 'text-amber-300',
      }, {
        active: true,
        type: 'local',
        class: 'text-sky-300',
      }, {
        active: false,
        type: 'global',
        class: 'text-amber-300/50 group-hover/button:text-amber-300',
      }, {
        active: false,
        type: 'local',
        class: 'text-sky-300/50 group-hover/button:text-sky-300',
      }],
    }),
  },

  action: {
    button: tv({
      base: 'leading-none gap-[1ch] transition-colors py-2 px-4 flex items-center w-full text-shadow-md',
      variants: {
        disabled: {
          true: 'pointer-events-none text-zinc-300/10',
          false: 'cursor-pointer text-neutral-300/50 hover:text-neutral-300 hover:bg-zinc-700',
        },
        warning: {
          true: 'border-l-2 border-red-500',
        },
      },
      defaultVariants: {
        disabled: false,
        warning: false,
      },
    }),
  },

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

        <button
          type="button"
          :class="styles.action.button({ })"
          @click="addDegrees(90)"
        >
          <icon name="ic:baseline-rotate-left" />
          <span>90° right</span>
        </button>

        <button
          type="button"
          :class="styles.action.button({ })"
          @click="subtractDegrees(90)"
        >
          <icon name="ic:baseline-rotate-right" />
          <span>90° left</span>
        </button>
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
