<script setup lang="ts">
// import { useMouse } from '@vueuse/core'
import { useEventListener } from '@vueuse/core'

const scan = useScanStore()

const workspace = useTemplateRef('workspace')

const scaleStep = 0.1
const scaleLevelDefault = 0.65
const scaleLevel = ref(scaleLevelDefault)

const postionDefault = {
  x: 0,
  y: 0,
}

const postion = reactive({
  x: postionDefault.x,
  y: postionDefault.y,
})

const styles = computed(() => ({
  transform: `translateX(${postion.x}px) translateY(${postion.y}px) scale(${scaleLevel.value})`,
}))

useEventListener(workspace, 'wheel', (e) => {
  console.log('mouse wheel delta:', e.deltaX, e.deltaY, e.deltaZ)

  const delta = e.deltaY < 0 ? 1 : -1

  scaleLevel.value *= (1 + scaleStep * delta)
  scaleLevel.value = Math.max(0.1, Math.min(scaleLevel.value, 10))
})

useEventListener(workspace, 'click', (e) => {
  // todo: debug
  //       Надо на кнопку какую-то назначить в интерфейсе...
  //       ...потом
  if (e.button === 0) {
    scaleLevel.value = scaleLevelDefault

    postion.x = postionDefault.x
    postion.y = postionDefault.y
  }
})
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
