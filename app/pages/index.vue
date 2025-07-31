<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

const newProjectName = ref('')

// Generate 5 fake recent projects
const recentProjects = ref<string[]>(Array.from({ length: 5 }, () => faker.commerce.productName()))

const generateProjectName = async () => {
  newProjectName.value = await invoke<string>('generate_random_name')
}

const showCreateModal = ref(false)

const createProject = () => {
  // TODO: Implement actual project creation logic
  showCreateModal.value = false
  newProjectName.value = ''
}

const openProject = async () => {
  // const selected = await open({ directory: true })
  // if (selected) {
  //   // TODO: Implement logic to open the selected project
  // }
}

onMounted(() => {
  generateProjectName()
})
</script>

<template>
  <section class="flex flex-col items-center justify-center h-screen">
    <button
      class="w-64 py-4 mb-8 text-xl font-semibold cursor-pointer rounded-lg shadow"
      @click="showCreateModal = true"
    >
      Create New Project
    </button>

    <div class="flex items-center w-64 mb-8">
      <div class="flex-grow h-px bg-gray-700" />
      <span class="mx-4 text-gray-500 font-semibold">OR</span>
      <div class="flex-grow h-px bg-gray-700" />
    </div>

    <button
      class="w-64 py-4 text-xl font-semibold cursor-pointer rounded-lg shadow"
      @click="openProject"
    >
      Open Existing Project
    </button>

    <!-- Recent Projects List -->
    <div class="w-64 mt-10">
      <div class="text-gray-700 font-semibold mb-2 text-center">
        Recent Projects
      </div>
      <ul class="space-y-2">
        <li
          v-for="project in recentProjects"
          :key="project"
        >
          <button
            class="w-full px-4 py-2 rounded-lg text-left"
            @click="newProjectName = project"
          >
            {{ project }}
          </button>
        </li>
      </ul>
    </div>

    <!-- Modal for creating a new project -->
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
  </section>
</template>
