<script setup lang="ts">
import type { TauriEvent } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWebview, type DragDropEvent } from '@tauri-apps/api/webview'

definePageMeta({
  layout: 'project',
})

const newProjectName = ref(await useCommands().generateRandomName())
const newProjectDirectory = ref(faker.system.directoryPath())
//
// Generate 5 fake recent projects
const recent = ref(Array.from({ length: 5 }, () => ({
  uuid: faker.string.uuid(),
  name: faker.commerce.productName(),
  path: 'Q:\\Users\\sawic\\OneDrive\\Рабочий стол\\RAW\\boundless-sheep.ScanLab',
  date: faker.date.recent({ days: 10 }),
})))

async function naviagetTo() {
  const project = useProjectStore()

  await project.refreshStages()
  const x = project.projectStages[0]

  const router = useRouter()
  await router.push({ path: '/open' })
}

async function createProject() {
  const commands = useCommands()
  const value = await commands.createProject(newProjectName.value, newProjectDirectory.value)

  if (value) {
    await naviagetTo()
  }
}

async function loadProject() {
  const commands = useCommands()
  const value = await commands.loadProject()

  if (value) {
    await naviagetTo()
  }
}

async function loadRecent(path: string) {
  const commands = useCommands()
  const value = await commands.loadRecentProject(path)

  if (value) {
    await naviagetTo()
  }
}

async function selectSaveFolder() {
  const commands = useCommands()
  const selected = await commands.selectDirectory()
  if (selected) {
    newProjectDirectory.value = selected
  }
}

const unlisten = await getCurrentWebview().onDragDropEvent(async (e) => {
  if (e.payload.type === 'drop') {
    const commands = useCommands()
    if (!(await commands.createTempProject())) {
      return
    }

    if (!(await commands.dropImages(e.payload.paths))) {
      return
    }

    await naviagetTo()
  }
})

onBeforeUnmount(() => {
  unlisten()
})

const isFormValid = computed(() => {
  return !!newProjectName.value && !!newProjectDirectory.value
})
</script>

<template>
  <section class="grid grid-cols-2 size-full gap-2">
    <div class="flex flex-col items-center justify-center gap-2 group relative">
      <ui-modal-window>
        <template #trigger="{ show, state }">
          <ui-button
            class="text-xl"
            @click="show()"
          >
            <icon name="ic:baseline-create-new-folder" />
            <span>Create New Project: {{ state }}</span>
          </ui-button>
        </template>

        <template #header>
          <section class="flex gap-2 justify-between items-center">
            <span>New project</span>

            <ui-button
              id="create"
              :disabled="!isFormValid"
              @click="createProject"
            >
              Create
            </ui-button>
          </section>
        </template>

        <template #body>
          <form class="space-y-2">
            <section class="flex gap-2">
              <ui-input
                id="name"
                v-model="newProjectName"
                placeholder="Project name"
                class="w-[50dvw]"
              />

              <ui-button
                id="dir"
                @click="selectSaveFolder"
              >
                Select directory
              </ui-button>
            </section>

            <p
              v-show="newProjectDirectory"
              class="inline-flex items-center gap-2 px-2 italic text-neutral-300/50"
            >
              <icon name="ic:baseline-folder" />
              <span>{{ newProjectDirectory }}</span>
            </p>
          </form>
        </template>
      </ui-modal-window>

      <div class="flex items-center w-full gap-2">
        <div class="flex-grow h-px bg-zinc-700 group-hover:bg-zinc-600 transition-colors" />
        <span class="text-zinc-700 group-hover:text-zinc-600 transition-colors">OR</span>
        <div class="flex-grow h-px bg-zinc-700 group-hover:bg-zinc-600 transition-colors" />
      </div>

      <ui-button
        class="text-xl"
        @click="loadProject"
      >
        <icon name="ic:baseline-folder-open" />
        <span>Open Existing Project</span>
      </ui-button>

      <div class="absolute bottom-0 left-0 text-neutral-300/50 inline-flex items-center gap-[1ch] text-shadow-md">
        <icon name="ic:baseline-move-to-inbox" />
        <span>You can also just drag n drop files into this window without project creation.</span>
      </div>
    </div>

    <aside class="p-2 backdrop-blur-[2px] border border-zinc-700 hover:border-zinc-600 transition-colors shadow-md flex flex-col overflow-hidden">
      <header class="text-center font-mono px-2 text-lg mb-2 uppercase">
        Recent projects
      </header>

      <section class="">
        <ul class="">
          <li
            v-for="project in recent"
            :key="project.uuid"
          >
            <ui-button
              class="flex gap-2 w-full"
              @click="loadRecent(project.path)"
            >
              <span class="text-nowrap text-start text-indigo-300/50 group-hover/button:text-indigo-300 transition-colors">
                {{ project.name }}
              </span>

              <span class="truncate text-start font-mono text-xs">
                {{ project.path }}
              </span>

              <span class="text-end font-mono text-xs">
                {{
                  project.date.toLocaleDateString(undefined, {
                    year: "numeric",
                    month: "2-digit",
                    day: "2-digit",
                  })
                }}
              </span>
            </ui-button>
          </li>
        </ul>
      </section>
    </aside>
  </section>
<!--
  <section class="flex flex-col items-center justify-center size-full">

    <div
      v-if="showCreateModal"
      class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-30 z-50"
    >
      <div class="p-8 rounded-lg shadow-lg flex flex-col items-center">
        <div class="text-lg font-semibold mb-4">
          Create New Project
        </div>
        Project name:
        <input
          v-model="newProjectName"
          placeholder="Enter project name"
          class="border px-3 py-2 rounded w-64 mb-4"
        >
        <div class="flex gap-4">
          <Icon
            name="ic:autorenew"
            class="cursor-pointer text-gray-500 hover:text-gray-700 transition"
            @click="generateProjectName"
          />
          <button
            class=""
            @click="selectSaveFolder"
          >
            Select folder
          </button>
          <div>
            Folder: {{ newProjectDirectory }}
          </div>
          <button
            class="bg-blue-600 text-white px-6 py-2 rounded"
            :disabled="!newProjectName.trim()"
            @click="createProject"
          >
            Create
          </button>
          <button
            class="bg-gray-300 text-gray-700 px-6 py-2 rounded"
            @click="showCreateModal = false"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  </section> -->
</template>
