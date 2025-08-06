<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

definePageMeta({
  layout: 'project',
})

const newProjectName = ref('')
const newProjectDirectory = ref('' as string)

// Generate 5 fake recent projects
const recent = ref<string[]>(Array.from({ length: 5 }, () => faker.commerce.productName()))

const generateProjectName = async () => {
  newProjectName.value = await invoke<string>('generate_random_name')
}

const showCreateModal = ref(false)

async function createProject() {
  await invoke('create_project', {
    name: newProjectName.value,
    dir: newProjectDirectory.value,
  }).then(() => {
    showCreateModal.value = false
  }).catch((error) => {
    console.error('Error creating project:', error)
  })
}

async function loadProject() {
  const selectedFile = await open({ multiple: false })
  if (selectedFile) {
    await invoke('load_project', { path: selectedFile })
      .then(() => {
        console.log('Project loaded successfully')
      })
      .catch((error) => {
        console.error('Error opening project:', error)
      })

    await invoke('add_stage', { stage: 'rotate' })

    await invoke('list_stages')
      .then((result) => {
        const stages = result as ProcessingStage[]
        console.log('Available stages:', stages)
      })

    await invoke('get_stage', { index: 1 })
      .then((result) => {
        const stage = result as ProcessingStage
        console.log('Stage at index 1:', stage)
      })
  }
}

async function createTempProject() {
  await invoke('create_temp_project')
    .then(() => {
      console.log('Temporary project created successfully')
    })
    .catch((error) => {
      console.error('Error creating temporary project:', error)
    })
}

async function selectSaveFolder() {
  const selected = await open({ directory: true })
  if (selected) {
    newProjectDirectory.value = selected
  }
}

onMounted(() => {
  console.log({ angle: 0 } as RotateParams)
})
</script>

<template>
  <section class="grid grid-cols-2 size-full gap-2">
    <div class="flex flex-col items-center justify-center gap-2">
      <ui-button
        class="text-xl"
        @click="showCreateModal = true; generateProjectName()"
      >
        <icon name="ic:baseline-create-new-folder" />
        <span>Create New Project</span>
      </ui-button>

      <div class="flex items-center w-full group gap-2">
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
    </div>

    <aside class="p-2 backdrop-blur-[2px] border border-zinc-700 hover:border-zinc-600 transition-colors shadow-md">
      <header class="text-center font-mono px-2 text-lg mb-2 uppercase">
        Recent projects
      </header>

      <section>
        <ul>
          <li
            v-for="project in recent"
            :key="project"
          >
            <ui-button
              class="flex gap-2"
              @click="newProjectName = project"
            >
              <span class="truncate text-start text-indigo-300/50 group-hover/button:text-indigo-300 transition-colors">
                {{ project }}
              </span>

              <span class="truncate text-start font-mono grow text-xs">
                {{ faker.system.filePath() }}
              </span>

              <span class="truncate text-end font-mono">
                {{
                  faker.date.recent({ days: 10 }).toLocaleDateString(undefined, {
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
