<script setup lang="ts">
const scans = useScansStore()
// const stages = useStagesStore()
</script>

<template>
  <!-- <section class="relative">
        <p class="min-w-24">
          stages
        </p>

        <section class="flex flex-col absolute top-full left-0 z-10 bg-zinc-800 max-h-72 overflow-y-auto border border-neutral-800 shadow ">
          <button
            v-for="stage of stages.available"
            :key="stage.uuid"
            type="button"
            class="cursor-pointer hover:bg-zinc-700 text-neutral-300/50 hover:text-neutral-300 transition-colors py-2 px-4 "
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
                class="hover:bg-zinc-700 [&.router-link-active]:bg-zinc-700 text-neutral-300/50 not-[&.router-link-active]:hover:text-amber-300 [&.router-link-active]:text-amber-300 transition-colors py-2 px-4  border-zinc-700 border-x shadow flex items-center leading-none [&.router-link-active]:pointer-events-none w-full"
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

  <section class="bg-neutral-800 text-neutral-300 h-dvh grid p-2 gap-2">
    <main class="overflow-hidden p-2 bg-workspace">
      <slot />
    </main>

    <section class="grid grid-cols-[auto_1fr_auto] grid-rows-[1fr_auto] gap-2 fixed left-2 right-2 bottom-2 top-2 pointer-events-none">
      <aside class="shadow overflow-hidden bg-zinc-800/30 col-start-1 pointer-events-auto backdrop-blur-md w-64">
        <section class="h-full py-2">
          <section class="space-y-2 px-2 overflow-y-auto overflow-x-hidden h-full">
            <section id="toolbar" />
          </section>
        </section>
      </aside>

      <aside class="shadow overflow-hidden bg-zinc-800/30 col-start-3 pointer-events-auto backdrop-blur-md w-48">
        <section class="h-full py-2">
          <section class="space-y-2 px-2 overflow-y-auto overflow-x-hidden h-full">
            <layout-default-preview
              v-for="scan in scans.data"
              v-bind="scan"
              :key="scan.uuid"
            />
          </section>
        </section>
      </aside>

      <footer class="p-2 flex justify-center col-span-full bg-zinc-800/30 shadow leading-none row-start-2 pointer-events-auto backdrop-blur-md">
        <fileprocess-stage />
      </footer>
    </section>
  </section>

  <!-- https://youtu.be/ChmVcJK7N-c?list=RDEM9bvfNCNqbQ4iScFk23dYAg -->
</template>
