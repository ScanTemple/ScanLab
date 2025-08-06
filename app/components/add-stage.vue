<script lang="ts" setup>
const { stagesArchive } = useProjectStore()
</script>

<template>
  <section>
    <div
      class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-30 z-50"
    >
      <div class="p-8 rounded-lg shadow-lg flex flex-col items-center">
        <div
          v-for="category in stagesArchive"
          :key="category.category"
          class="mb-4"
        >
          <h2 class="text-lg font-semibold mb-2">
            {{ category.category }}
          </h2>
          <div class="flex gap-4">
            <ui-button
              v-for="stage in category.stages"
              :key="stage.value"
            >
              <Icon :name="stage.icon" />
              <span>{{ stage.name }}</span>
            </ui-button>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
