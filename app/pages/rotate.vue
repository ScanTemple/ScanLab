<script setup lang="ts">
import type { UUID } from 'crypto'

type DataRotateLayerBase = {
  uuid: UUID
  value: number
}

type DataRotateLayerGlobal = DataRotateLayerBase & {
  type: 'global'
  rules: unknown[]
}

type DataRotateLayerLocal = DataRotateLayerBase & {
  type: 'local'
}

type DataRotateLayer = DataRotateLayerGlobal | DataRotateLayerLocal

const shaderd = [{
  uuid: crypto.randomUUID(),
  value: faker.number.int({ min: 0, max: 360 }),
  type: 'global',
  rules: [],
}, {
  uuid: crypto.randomUUID(),
  value: faker.number.int({ min: 0, max: 360 }),
  type: 'global',
  rules: [],
}] satisfies DataRotateLayer[] as DataRotateLayer[]

const local = [{
  uuid: crypto.randomUUID(),
  value: faker.number.int({ min: 0, max: 360 }),
  type: 'local',
}, {
  uuid: crypto.randomUUID(),
  value: faker.number.int({ min: 0, max: 360 }),
  type: 'local',
}] satisfies DataRotateLayer[] as DataRotateLayer[]

const _active = ref(undefined as undefined | UUID)

const layers = computed(() => [...shaderd, ...local])
const active = computed(() => layers.value.find(e => e.uuid === _active.value))

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
      base: 'transition-colors text-end',
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
          false: 'cursor-pointer text-neutral-300/50 hover:text-neutral-300',
        },
      },
      defaultVariants: {
        disabled: false,
      },
    }),
  },
}
</script>

<template>
  <section>
    <aside class="space-y-16">
      <section class="w-fit grid grid-cols-[auto_1fr] items-center">
        <template
          v-for="layer of layers"
          :key="layer.uuid"
        >
          <span
            :class="styles.layer.value({ active: layer.uuid === _active })"
            class="text-shadow-md"
          >
            {{ layer.value }}°
          </span>

          <button
            :class="styles.layer.button({ active: layer.uuid === _active })"
            @click="_active = layer.uuid"
          >
            <icon
              v-if="layer.uuid === _active"
              name="ic:outline-layers"
            />
            <icon
              v-else
              name="ic:baseline-layers"
            />

            <span :class="styles.layer.type({ type: layer.type, active: layer.uuid === _active })">
              {{ layer.type }}
            </span>

            <span>layer</span>
          </button>
        </template>

        <button
          type="button"
          :class="styles.action.button()"
          class="col-start-2 mt-2"
        >
          <icon name="ic:baseline-add" />
          <span>add layer</span>
        </button>

        <section class="col-start-2 pt-16">
          <button
            type="button"
            :class="styles.action.button({ disabled: !active })"
          >
            <icon name="ic:baseline-rotate-left" />
            <span>90° right</span>
          </button>

          <button
            type="button"
            :class="styles.action.button({ disabled: !active })"
          >
            <icon name="ic:baseline-rotate-right" />
            <span>90° left</span>
          </button>
        </section>
      </section>
    </aside>
    <!-- q(≧▽≦q) -->
  </section>
</template>
