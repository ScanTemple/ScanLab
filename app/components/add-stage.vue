<script lang="ts" setup>

const stages = [{
    category: '',
    stages: [
        { name: 'Rotate', value: 'rotate', icon: 'ic:baseline-rotate-right' },
        { name: 'Crop', value: 'crop', icon: 'ic:baseline-crop' },
        { name: 'Resize', value: 'resize', icon: 'ic:baseline-resize' },
    ]
},{
    category: 'Basic',
    stages: [
        { name: 'Flip', value: 'flip', icon: 'ic:baseline-flip' },
        { name: 'Filter', value: 'filter', icon: 'ic:baseline-filter' },
    ]
},{
    category: 'Advanced', // just for example
    stages: [
        { name: 'Color Correction', value: 'color-correction', icon: 'ic:baseline-colorize' },
        { name: 'Sharpen', value: 'sharpen', icon: 'ic:baseline-sharpness' },
        { name: 'Noise Reduction', value: 'noise-reduction', icon: 'ic:baseline-noise-control' },
        { name: 'HDR Merge', value: 'hdr-merge', icon: 'ic:baseline-hdr-strong' },
    ]
}]
</script>

<template>
  <section>
    <div
      class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-30 z-50"
    >
      <div class="p-8 rounded-lg shadow-lg flex flex-col items-center">
        <div v-for="category in stages" :key="category.category" class="mb-4">
          <h2 class="text-lg font-semibold mb-2">{{ category.category }}</h2>
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
