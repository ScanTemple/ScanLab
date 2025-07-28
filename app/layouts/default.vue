<script setup lang="ts">
const scans = useScansStore()
// const stages = useStagesStore()
</script>

<template>
  <section class="bg-neutral-800 text-neutral-300 h-dvh grid grid-cols-[1fr_4fr_1fr] grid-rows-[1fr_auto] overflow-hidden p-2 gap-2">
    <aside class="shadow rounded overflow-hidden bg-zinc-800">
      <section class="h-full py-2">
        <section class="space-y-2 px-2 overflow-y-auto h-full">
          <!-- <section class="relative">
        <p class="min-w-24">
          stages
        </p>

        <section class="flex flex-col absolute top-full left-0 z-10 bg-zinc-800 max-h-72 overflow-y-auto border border-neutral-800 shadow rounded">
          <button
            v-for="stage of stages.available"
            :key="stage.uuid"
            type="button"
            class="cursor-pointer hover:bg-zinc-700 text-neutral-300/50 hover:text-neutral-300 transition-colors py-2 px-4 rounded"
          >
            {{ stage.label }}
          </button>
        </section>
      </section> -->
          <!--
      <layout-default-sidebar-block>
        <template #header>
          stages
        </template>

        <template #body>
          <ul class="space-y-2">
            <li
              v-for="layer in stageLayers.defined"
              :key="layer.uuid"
            >
              <NuxtLink
                type="button"
                class="hover:bg-zinc-700 [&.router-link-active]:bg-zinc-700 text-neutral-300/50 not-[&.router-link-active]:hover:text-amber-300 [&.router-link-active]:text-amber-300 transition-colors py-2 px-4 rounded border-zinc-700 border-x shadow flex items-center leading-none [&.router-link-active]:pointer-events-none w-full"
                :to="layer.reference.url"
              >
                <icon
                  :name="layer.reference.icon"
                  class="mr-2"
                />

                <span>
                  {{ layer.reference.label }}
                </span>
              </NuxtLink>
            </li>

            <li>
              add
            </li>
          </ul>
        </template>
      </layout-default-sidebar-block> -->

          <layout-default-sidebar-block
            v-for="i of faker.number.int({ min: 12, max: 32 })"
            :key="i"
          >
            <template #header>
              {{ faker.lorem.words({ min: 1, max: 3 }) }}
            </template>

            <template #body>
              {{ faker.lorem.paragraph() }}
            </template>
          </layout-default-sidebar-block>
        </section>
      </section>
    </aside>

    <main class="overflow-hidden p-2">
      <slot />
    </main>

    <aside class="shadow rounded overflow-hidden bg-zinc-800">
      <section class="h-full py-2">
        <section class="space-y-2 px-2 overflow-y-auto h-full">
          <layout-default-preview
            v-for="scan in scans.data"
            v-bind="scan"
            :key="scan.uuid"
          />
        </section>
      </section>
    </aside>

    <section class="p-2 flex justify-center col-span-full bg-zinc-800 rounded shadow leading-none">
      <fileprocess-stage />
    </section>
  </section>
</template>
