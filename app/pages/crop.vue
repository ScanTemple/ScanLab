<script setup lang="ts">
// import { useMouse } from '@vueuse/core'
import { useEventListener } from '@vueuse/core'

const scan = useScanStore()

const workspace = useTemplateRef('workspace')

const scaleLevel = ref(1)

const styles = computed(() => ({
  // transform: 'translateX(-50%) translateY(-50%)',
  transform: `scale(${scaleLevel.value})`,
}))

// onMounted(() => {
useEventListener(workspace, 'wheel', (e) => {
  const SCALE_STEP = 0.1

  console.log('mouse wheel delta:', e.deltaX, e.deltaY, e.deltaZ)

  const delta = e.deltaY < 0 ? 1 : -1

  // Экспоненциальное изменение масштаба для постоянной скорости
  scaleLevel.value *= (1 + SCALE_STEP * delta)

  // Ограничиваем масштаб (опционально)
  scaleLevel.value = Math.max(0.1, Math.min(scaleLevel.value, 10)) // Минимум 0.1, максимум 10

  console.log(scaleLevel.value)
})

useEventListener(workspace, 'keydown', () => {
  console.log('x')
})

useEventListener(workspace, 'click', () => {
  scaleLevel.value = 1
  console.log('x')
})
// })
</script>

<template>
  <section class="fixed top-0 left-0 right-0 bottom-0">
    <template v-if="scan.data">
      <section
        ref="workspace"
        class="h-full w-full flex items-center justify-center"
      >
        <img
          :src="scan.data.image"
          :style="styles"
          class="h-full"
        >
      </section>
    </template>

    <template v-else>
      pick scan in right bar ┻━┻ ︵ ＼( °□° )／ ︵ ┻━┻
    </template>
  </section>
</template>

<style scoped>
/* https://stackoverflow.com/questions/14068103/disable-antialising-when-scaling-images */
img {
    image-rendering: optimizeSpeed;             /* STOP SMOOTHING, GIVE ME SPEED  */
    image-rendering: -moz-crisp-edges;          /* Firefox                        */
    image-rendering: -o-crisp-edges;            /* Opera                          */
    image-rendering: -webkit-optimize-contrast; /* Chrome (and eventually Safari) */
    image-rendering: pixelated;                 /* Universal support since 2021   */
    image-rendering: optimize-contrast;         /* CSS3 Proposed                  */
    -ms-interpolation-mode: nearest-neighbor;   /* IE8+                           */
}
</style>
